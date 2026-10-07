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

//! Response types of the `rollups_common` specification namespace

#![allow(missing_docs)]

#[doc = "Defines a cron schedule for the rollup job execution."]
#[derive(Debug, Clone)]
pub struct Cron {
    #[doc = "The cron expression that defines the schedule."]
    pub expression: Option<String>,
    #[doc = "The timezone for interpreting the cron expression."]
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Cron {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Cron;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Cron object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_expression: Option<String> = None;
                let mut seen_expression = false;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "expression" => {
                            if seen_expression {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            seen_expression = true;
                            f_expression = map.next_value::<Option<String>>()?;
                        }
                        "timezone" => {
                            if seen_timezone {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            seen_timezone = true;
                            f_timezone = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Cron {
                    expression: f_expression,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Cron {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.expression {
            map.serialize_entry("expression", v)?;
        }
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Configures a date histogram dimension for rollup aggregation."]
#[derive(Debug, Clone)]
pub struct DateHistogramDimension {
    #[doc = "The calendar interval for the date histogram."]
    pub calendar_interval: Option<String>,
    #[doc = "The fixed interval for the date histogram."]
    pub fixed_interval: Option<String>,
    #[doc = "The date format pattern for the histogram values."]
    pub format: Option<String>,
    #[doc = "The field from the source documents to use for the date histogram."]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the date histogram results."]
    pub target_field: Option<String>,
    #[doc = "The timezone for the date histogram calculations."]
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateHistogramDimension {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateHistogramDimension;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateHistogramDimension object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_calendar_interval: Option<String> = None;
                let mut seen_calendar_interval = false;
                let mut f_fixed_interval: Option<String> = None;
                let mut seen_fixed_interval = false;
                let mut f_format: Option<String> = None;
                let mut seen_format = false;
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "calendar_interval" => {
                            if seen_calendar_interval {
                                return Err(serde::de::Error::duplicate_field("calendar_interval"));
                            }
                            seen_calendar_interval = true;
                            f_calendar_interval = map.next_value::<Option<String>>()?;
                        }
                        "fixed_interval" => {
                            if seen_fixed_interval {
                                return Err(serde::de::Error::duplicate_field("fixed_interval"));
                            }
                            seen_fixed_interval = true;
                            f_fixed_interval = map.next_value::<Option<String>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map.next_value::<Option<String>>()?;
                        }
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        "timezone" => {
                            if seen_timezone {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            seen_timezone = true;
                            f_timezone = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateHistogramDimension {
                    calendar_interval: f_calendar_interval,
                    fixed_interval: f_fixed_interval,
                    format: f_format,
                    source_field: f_source_field,
                    target_field: f_target_field,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateHistogramDimension {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.calendar_interval {
            map.serialize_entry("calendar_interval", v)?;
        }
        if let Some(v) = &self.fixed_interval {
            map.serialize_entry("fixed_interval", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Defines a dimension for aggregating data in a rollup job."]
#[derive(Debug, Clone)]
pub struct DimensionsConfigItem {
    pub date_histogram: Option<crate::types::rollups_common::DateHistogramDimension>,
    pub histogram: Option<crate::types::rollups_common::HistogramDimension>,
    pub terms: Option<crate::types::rollups_common::TermsDimension>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DimensionsConfigItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DimensionsConfigItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DimensionsConfigItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_date_histogram: Option<
                    crate::types::rollups_common::DateHistogramDimension,
                > = None;
                let mut seen_date_histogram = false;
                let mut f_histogram: Option<crate::types::rollups_common::HistogramDimension> =
                    None;
                let mut seen_histogram = false;
                let mut f_terms: Option<crate::types::rollups_common::TermsDimension> = None;
                let mut seen_terms = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "date_histogram" => {
                            if seen_date_histogram {
                                return Err(serde::de::Error::duplicate_field("date_histogram"));
                            }
                            seen_date_histogram = true;
                            f_date_histogram = map.next_value::<Option<crate::types::rollups_common::DateHistogramDimension>>()?;
                        }
                        "histogram" => {
                            if seen_histogram {
                                return Err(serde::de::Error::duplicate_field("histogram"));
                            }
                            seen_histogram = true;
                            f_histogram = map.next_value::<Option<crate::types::rollups_common::HistogramDimension>>()?;
                        }
                        "terms" => {
                            if seen_terms {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            seen_terms = true;
                            f_terms = map
                                .next_value::<Option<crate::types::rollups_common::TermsDimension>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DimensionsConfigItem {
                    date_histogram: f_date_histogram,
                    histogram: f_histogram,
                    terms: f_terms,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DimensionsConfigItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.date_histogram {
            map.serialize_entry("date_histogram", v)?;
        }
        if let Some(v) = &self.histogram {
            map.serialize_entry("histogram", v)?;
        }
        if let Some(v) = &self.terms {
            map.serialize_entry("terms", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Provides explanation details for a rollup job."]
#[derive(Debug, Clone)]
pub struct Explain {
    #[doc = "The metadata ID for the rollup job."]
    pub metadata_id: Option<String>,
    #[doc = "The metadata associated with the rollup job execution."]
    pub rollup_metadata:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Explain {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Explain;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Explain object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_metadata_id: Option<String> = None;
                let mut seen_metadata_id = false;
                let mut f_rollup_metadata: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_rollup_metadata = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "metadata_id" => {
                            if seen_metadata_id {
                                return Err(serde::de::Error::duplicate_field("metadata_id"));
                            }
                            seen_metadata_id = true;
                            f_metadata_id = map.next_value::<Option<String>>()?;
                        }
                        "rollup_metadata" => {
                            if seen_rollup_metadata {
                                return Err(serde::de::Error::duplicate_field("rollup_metadata"));
                            }
                            seen_rollup_metadata = true;
                            f_rollup_metadata = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Explain {
                    metadata_id: f_metadata_id,
                    rollup_metadata: f_rollup_metadata,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Explain {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.metadata_id {
            map.serialize_entry("metadata_id", v)?;
        }
        if let Some(v) = &self.rollup_metadata {
            map.serialize_entry("rollup_metadata", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Maps rollup IDs to their explanation details."]
pub type ExplainEntities =
    std::collections::BTreeMap<String, crate::types::rollups_common::Explain>;

#[doc = "Configures a numeric histogram dimension for rollup aggregation."]
#[derive(Debug, Clone)]
pub struct HistogramDimension {
    #[doc = "The interval for the histogram buckets."]
    pub interval: Option<String>,
    #[doc = "The field from the source documents to use for the histogram."]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the histogram results."]
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HistogramDimension {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HistogramDimension;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HistogramDimension object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_interval: Option<String> = None;
                let mut seen_interval = false;
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "interval" => {
                            if seen_interval {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            seen_interval = true;
                            f_interval = map.next_value::<Option<String>>()?;
                        }
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HistogramDimension {
                    interval: f_interval,
                    source_field: f_source_field,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HistogramDimension {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.interval {
            map.serialize_entry("interval", v)?;
        }
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Specifies the timing details for rollup job execution."]
#[derive(Debug, Clone)]
pub struct Interval {
    #[doc = "The cron schedule configuration for the rollup job."]
    pub cron: Option<crate::types::rollups_common::IntervalCron>,
    #[doc = "The numerical value for the execution interval."]
    pub period: Option<f64>,
    #[doc = "The delay in milliseconds before starting the rollup process."]
    pub schedule_delay: Option<f64>,
    #[doc = "When the rollup job should start, in epoch milliseconds."]
    pub start_time: Option<f64>,
    #[doc = "The time unit for the execution interval, such as `MINUTES` or `HOURS`."]
    pub unit: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Interval;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Interval object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cron: Option<crate::types::rollups_common::IntervalCron> = None;
                let mut seen_cron = false;
                let mut f_period: Option<f64> = None;
                let mut seen_period = false;
                let mut f_schedule_delay: Option<f64> = None;
                let mut seen_schedule_delay = false;
                let mut f_start_time: Option<f64> = None;
                let mut seen_start_time = false;
                let mut f_unit: Option<String> = None;
                let mut seen_unit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cron" => {
                            if seen_cron {
                                return Err(serde::de::Error::duplicate_field("cron"));
                            }
                            seen_cron = true;
                            f_cron = map
                                .next_value::<Option<crate::types::rollups_common::IntervalCron>>(
                                )?;
                        }
                        "period" => {
                            if seen_period {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            seen_period = true;
                            f_period = map.next_value::<Option<f64>>()?;
                        }
                        "schedule_delay" => {
                            if seen_schedule_delay {
                                return Err(serde::de::Error::duplicate_field("schedule_delay"));
                            }
                            seen_schedule_delay = true;
                            f_schedule_delay = map.next_value::<Option<f64>>()?;
                        }
                        "start_time" => {
                            if seen_start_time {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            seen_start_time = true;
                            f_start_time = map.next_value::<Option<f64>>()?;
                        }
                        "unit" => {
                            if seen_unit {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            seen_unit = true;
                            f_unit = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Interval {
                    cron: f_cron,
                    period: f_period,
                    schedule_delay: f_schedule_delay,
                    start_time: f_start_time,
                    unit: f_unit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cron {
            map.serialize_entry("cron", v)?;
        }
        if let Some(v) = &self.period {
            map.serialize_entry("period", v)?;
        }
        if let Some(v) = &self.schedule_delay {
            map.serialize_entry("schedule_delay", v)?;
        }
        if let Some(v) = &self.start_time {
            map.serialize_entry("start_time", v)?;
        }
        if let Some(v) = &self.unit {
            map.serialize_entry("unit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The cron schedule configuration for the rollup job."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IntervalCron {
    Array(Vec<crate::types::rollups_common::Cron>),
    Cron(crate::types::rollups_common::Cron),
}
impl<'de> serde::Deserialize<'de> for IntervalCron {
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
            Some(b'[') => serde_json::from_str(text).map(IntervalCron::Array),
            Some(b'{') => serde_json::from_str(text).map(IntervalCron::Cron),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a IntervalCron value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Defines metric aggregations for a field in a rollup job."]
#[derive(Debug, Clone)]
pub struct MetricsConfigItem {
    #[doc = "The list of metrics to calculate for the field."]
    pub metrics: Option<Vec<crate::types::rollups_common::MetricsConfigMetrics>>,
    #[doc = "The field from the source documents to calculate metrics on."]
    pub source_field: Option<String>,
    #[doc = "The field name prefix to use in the target index for the metric results."]
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MetricsConfigItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MetricsConfigItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MetricsConfigItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_metrics: Option<Vec<crate::types::rollups_common::MetricsConfigMetrics>> =
                    None;
                let mut seen_metrics = false;
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "metrics" => {
                            if seen_metrics {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            seen_metrics = true;
                            f_metrics = map.next_value::<Option<Vec<crate::types::rollups_common::MetricsConfigMetrics>>>()?;
                        }
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MetricsConfigItem {
                    metrics: f_metrics,
                    source_field: f_source_field,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MetricsConfigItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.metrics {
            map.serialize_entry("metrics", v)?;
        }
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Specifies a metric aggregation type to calculate."]
#[derive(Debug, Clone)]
pub struct MetricsConfigMetrics {
    #[doc = "Calculates the average value of a field."]
    pub avg: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Finds the maximum value of a field."]
    pub max: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Finds the minimum value of a field."]
    pub min: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Calculates the sum of values for a field."]
    pub sum: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Counts the number of values for a field."]
    pub value_count: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MetricsConfigMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MetricsConfigMetrics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MetricsConfigMetrics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_avg: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_avg = false;
                let mut f_max: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_max = false;
                let mut f_min: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_min = false;
                let mut f_sum: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_sum = false;
                let mut f_value_count: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_value_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "avg" => {
                            if seen_avg {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            seen_avg = true;
                            f_avg = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "sum" => {
                            if seen_sum {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            seen_sum = true;
                            f_sum = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "value_count" => {
                            if seen_value_count {
                                return Err(serde::de::Error::duplicate_field("value_count"));
                            }
                            seen_value_count = true;
                            f_value_count = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MetricsConfigMetrics {
                    avg: f_avg,
                    max: f_max,
                    min: f_min,
                    sum: f_sum,
                    value_count: f_value_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MetricsConfigMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.avg {
            map.serialize_entry("avg", v)?;
        }
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.sum {
            map.serialize_entry("sum", v)?;
        }
        if let Some(v) = &self.value_count {
            map.serialize_entry("value_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Defines the configuration for a rollup job."]
#[derive(Debug, Clone)]
pub struct Rollup {
    #[doc = "Whether the rollup job runs continuously."]
    pub continuous: Option<bool>,
    #[doc = "The time delay in milliseconds between consecutive rollup executions."]
    pub delay: Option<f64>,
    #[doc = "The description of the rollup job."]
    pub description: Option<String>,
    #[doc = "The dimensions to aggregate data in the rollup."]
    pub dimensions: Option<Vec<crate::types::rollups_common::DimensionsConfigItem>>,
    #[doc = "Whether the rollup job is enabled."]
    pub enabled: Option<bool>,
    #[doc = "When the rollup job was enabled."]
    pub enabled_time: Option<f64>,
    #[doc = "Set up a Mustache message template for error notifications. For example, if an index rollup job fails, the system can send  a message to a Slack channel."]
    pub error_notification: Option<String>,
    #[doc = "When the rollup job was last updated."]
    pub last_updated_time: Option<f64>,
    #[doc = "The metadata ID associated with the rollup job."]
    pub metadata_id: Option<String>,
    #[doc = "The metrics to calculate in the rollup."]
    pub metrics: Option<Vec<crate::types::rollups_common::MetricsConfigItem>>,
    #[doc = "The number of documents to process per iteration."]
    pub page_size: Option<f64>,
    #[doc = "The ID for the rollup job."]
    pub rollup_id: Option<String>,
    pub schedule: Option<crate::types::rollups_common::Schedule>,
    #[doc = "The schema version for the rollup job."]
    pub schema_version: Option<f64>,
    #[doc = "The source index pattern for the rollup job."]
    pub source_index: Option<String>,
    #[doc = "The target index where rollup results are stored."]
    pub target_index: Option<String>,
    pub target_index_settings: Option<crate::types::indices_common::IndexSettings>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Rollup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Rollup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Rollup object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_continuous: Option<bool> = None;
                let mut seen_continuous = false;
                let mut f_delay: Option<f64> = None;
                let mut seen_delay = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_dimensions: Option<
                    Vec<crate::types::rollups_common::DimensionsConfigItem>,
                > = None;
                let mut seen_dimensions = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_enabled_time: Option<f64> = None;
                let mut seen_enabled_time = false;
                let mut f_error_notification: Option<String> = None;
                let mut seen_error_notification = false;
                let mut f_last_updated_time: Option<f64> = None;
                let mut seen_last_updated_time = false;
                let mut f_metadata_id: Option<String> = None;
                let mut seen_metadata_id = false;
                let mut f_metrics: Option<Vec<crate::types::rollups_common::MetricsConfigItem>> =
                    None;
                let mut seen_metrics = false;
                let mut f_page_size: Option<f64> = None;
                let mut seen_page_size = false;
                let mut f_rollup_id: Option<String> = None;
                let mut seen_rollup_id = false;
                let mut f_schedule: Option<crate::types::rollups_common::Schedule> = None;
                let mut seen_schedule = false;
                let mut f_schema_version: Option<f64> = None;
                let mut seen_schema_version = false;
                let mut f_source_index: Option<String> = None;
                let mut seen_source_index = false;
                let mut f_target_index: Option<String> = None;
                let mut seen_target_index = false;
                let mut f_target_index_settings: Option<
                    crate::types::indices_common::IndexSettings,
                > = None;
                let mut seen_target_index_settings = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "continuous" => {
                            if seen_continuous {
                                return Err(serde::de::Error::duplicate_field("continuous"));
                            }
                            seen_continuous = true;
                            f_continuous = map.next_value::<Option<bool>>()?;
                        }
                        "delay" => {
                            if seen_delay {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            seen_delay = true;
                            f_delay = map.next_value::<Option<f64>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "dimensions" => {
                            if seen_dimensions {
                                return Err(serde::de::Error::duplicate_field("dimensions"));
                            }
                            seen_dimensions = true;
                            f_dimensions = map.next_value::<Option<Vec<crate::types::rollups_common::DimensionsConfigItem>>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "enabled_time" => {
                            if seen_enabled_time {
                                return Err(serde::de::Error::duplicate_field("enabled_time"));
                            }
                            seen_enabled_time = true;
                            f_enabled_time = map.next_value::<Option<f64>>()?;
                        }
                        "error_notification" => {
                            if seen_error_notification {
                                return Err(serde::de::Error::duplicate_field(
                                    "error_notification",
                                ));
                            }
                            seen_error_notification = true;
                            f_error_notification = map.next_value::<Option<String>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<f64>>()?;
                        }
                        "metadata_id" => {
                            if seen_metadata_id {
                                return Err(serde::de::Error::duplicate_field("metadata_id"));
                            }
                            seen_metadata_id = true;
                            f_metadata_id = map.next_value::<Option<String>>()?;
                        }
                        "metrics" => {
                            if seen_metrics {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            seen_metrics = true;
                            f_metrics = map.next_value::<Option<Vec<crate::types::rollups_common::MetricsConfigItem>>>()?;
                        }
                        "page_size" => {
                            if seen_page_size {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            seen_page_size = true;
                            f_page_size = map.next_value::<Option<f64>>()?;
                        }
                        "rollup_id" => {
                            if seen_rollup_id {
                                return Err(serde::de::Error::duplicate_field("rollup_id"));
                            }
                            seen_rollup_id = true;
                            f_rollup_id = map.next_value::<Option<String>>()?;
                        }
                        "schedule" => {
                            if seen_schedule {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            seen_schedule = true;
                            f_schedule =
                                map.next_value::<Option<crate::types::rollups_common::Schedule>>()?;
                        }
                        "schema_version" => {
                            if seen_schema_version {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            seen_schema_version = true;
                            f_schema_version = map.next_value::<Option<f64>>()?;
                        }
                        "source_index" => {
                            if seen_source_index {
                                return Err(serde::de::Error::duplicate_field("source_index"));
                            }
                            seen_source_index = true;
                            f_source_index = map.next_value::<Option<String>>()?;
                        }
                        "target_index" => {
                            if seen_target_index {
                                return Err(serde::de::Error::duplicate_field("target_index"));
                            }
                            seen_target_index = true;
                            f_target_index = map.next_value::<Option<String>>()?;
                        }
                        "target_index_settings" => {
                            if seen_target_index_settings {
                                return Err(serde::de::Error::duplicate_field(
                                    "target_index_settings",
                                ));
                            }
                            seen_target_index_settings = true;
                            f_target_index_settings = map
                                .next_value::<Option<crate::types::indices_common::IndexSettings>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Rollup {
                    continuous: f_continuous,
                    delay: f_delay,
                    description: f_description,
                    dimensions: f_dimensions,
                    enabled: f_enabled,
                    enabled_time: f_enabled_time,
                    error_notification: f_error_notification,
                    last_updated_time: f_last_updated_time,
                    metadata_id: f_metadata_id,
                    metrics: f_metrics,
                    page_size: f_page_size,
                    rollup_id: f_rollup_id,
                    schedule: f_schedule,
                    schema_version: f_schema_version,
                    source_index: f_source_index,
                    target_index: f_target_index,
                    target_index_settings: f_target_index_settings,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Rollup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.continuous {
            map.serialize_entry("continuous", v)?;
        }
        if let Some(v) = &self.delay {
            map.serialize_entry("delay", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.dimensions {
            map.serialize_entry("dimensions", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.enabled_time {
            map.serialize_entry("enabled_time", v)?;
        }
        if let Some(v) = &self.error_notification {
            map.serialize_entry("error_notification", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.metadata_id {
            map.serialize_entry("metadata_id", v)?;
        }
        if let Some(v) = &self.metrics {
            map.serialize_entry("metrics", v)?;
        }
        if let Some(v) = &self.page_size {
            map.serialize_entry("page_size", v)?;
        }
        if let Some(v) = &self.rollup_id {
            map.serialize_entry("rollup_id", v)?;
        }
        if let Some(v) = &self.schedule {
            map.serialize_entry("schedule", v)?;
        }
        if let Some(v) = &self.schema_version {
            map.serialize_entry("schema_version", v)?;
        }
        if let Some(v) = &self.source_index {
            map.serialize_entry("source_index", v)?;
        }
        if let Some(v) = &self.target_index {
            map.serialize_entry("target_index", v)?;
        }
        if let Some(v) = &self.target_index_settings {
            map.serialize_entry("target_index_settings", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A rollup job entity with metadata."]
#[derive(Debug, Clone)]
pub struct RollupEntity {
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub rollup: Option<crate::types::rollups_common::Rollup>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RollupEntity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RollupEntity;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RollupEntity object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_rollup: Option<crate::types::rollups_common::Rollup> = None;
                let mut seen_rollup = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "rollup" => {
                            if seen_rollup {
                                return Err(serde::de::Error::duplicate_field("rollup"));
                            }
                            seen_rollup = true;
                            f_rollup =
                                map.next_value::<Option<crate::types::rollups_common::Rollup>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RollupEntity {
                    id: f_id,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    version: f_version,
                    rollup: f_rollup,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RollupEntity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.rollup {
            map.serialize_entry("rollup", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Defines the execution schedule for a rollup job."]
#[derive(Debug, Clone)]
pub struct Schedule {
    pub interval: Option<crate::types::rollups_common::Interval>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Schedule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Schedule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Schedule object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_interval: Option<crate::types::rollups_common::Interval> = None;
                let mut seen_interval = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "interval" => {
                            if seen_interval {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            seen_interval = true;
                            f_interval =
                                map.next_value::<Option<crate::types::rollups_common::Interval>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Schedule {
                    interval: f_interval,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Schedule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.interval {
            map.serialize_entry("interval", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Configures a terms dimension for rollup aggregation."]
#[derive(Debug, Clone)]
pub struct TermsDimension {
    #[doc = "The field from the source documents to use for terms aggregation."]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the terms aggregation results."]
    pub target_field: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TermsDimension {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TermsDimension;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TermsDimension object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_source_field: Option<String> = None;
                let mut seen_source_field = false;
                let mut f_target_field: Option<String> = None;
                let mut seen_target_field = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "source_field" => {
                            if seen_source_field {
                                return Err(serde::de::Error::duplicate_field("source_field"));
                            }
                            seen_source_field = true;
                            f_source_field = map.next_value::<Option<String>>()?;
                        }
                        "target_field" => {
                            if seen_target_field {
                                return Err(serde::de::Error::duplicate_field("target_field"));
                            }
                            seen_target_field = true;
                            f_target_field = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TermsDimension {
                    source_field: f_source_field,
                    target_field: f_target_field,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TermsDimension {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.source_field {
            map.serialize_entry("source_field", v)?;
        }
        if let Some(v) = &self.target_field {
            map.serialize_entry("target_field", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
