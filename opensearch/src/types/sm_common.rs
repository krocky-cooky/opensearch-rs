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

//! Response types of the `sm_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct CreationConfig {
    pub schedule: crate::types::sm_common::CronSchedule,
    #[doc = "The maximum amount of time for allowed snapshot creation."]
    pub time_limit: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CreationConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CreationConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CreationConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_schedule: Option<crate::types::sm_common::CronSchedule> = None;
                let mut f_time_limit: Option<String> = None;
                let mut seen_time_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "schedule" => {
                            if f_schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            f_schedule =
                                Some(map.next_value::<crate::types::sm_common::CronSchedule>()?);
                        }
                        "time_limit" => {
                            if seen_time_limit {
                                return Err(serde::de::Error::duplicate_field("time_limit"));
                            }
                            seen_time_limit = true;
                            f_time_limit = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CreationConfig {
                    schedule: f_schedule
                        .ok_or_else(|| serde::de::Error::missing_field("schedule"))?,
                    time_limit: f_time_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CreationConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("schedule", &self.schedule)?;
        if let Some(v) = &self.time_limit {
            map.serialize_entry("time_limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CronExpression {
    #[doc = "The cron expression that defines the schedule."]
    pub expression: String,
    #[doc = "The timezone used for the cron schedule."]
    pub timezone: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CronExpression {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CronExpression;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CronExpression object")
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
                let mut f_timezone: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "expression" => {
                            if f_expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            f_expression = Some(map.next_value::<String>()?);
                        }
                        "timezone" => {
                            if f_timezone.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            f_timezone = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CronExpression {
                    expression: f_expression
                        .ok_or_else(|| serde::de::Error::missing_field("expression"))?,
                    timezone: f_timezone
                        .ok_or_else(|| serde::de::Error::missing_field("timezone"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CronExpression {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("expression", &self.expression)?;
        map.serialize_entry("timezone", &self.timezone)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CronSchedule {
    pub cron: Option<crate::types::sm_common::CronExpression>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CronSchedule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CronSchedule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CronSchedule object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cron: Option<crate::types::sm_common::CronExpression> = None;
                let mut seen_cron = false;
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
                                .next_value::<Option<crate::types::sm_common::CronExpression>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CronSchedule {
                    cron: f_cron,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CronSchedule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cron {
            map.serialize_entry("cron", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DeletionCondition {
    pub max_age: Option<String>,
    pub max_count: Option<i64>,
    pub min_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeletionCondition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeletionCondition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeletionCondition object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_age: Option<String> = None;
                let mut seen_max_age = false;
                let mut f_max_count: Option<i64> = None;
                let mut seen_max_count = false;
                let mut f_min_count: Option<i64> = None;
                let mut seen_min_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_age" => {
                            if seen_max_age {
                                return Err(serde::de::Error::duplicate_field("max_age"));
                            }
                            seen_max_age = true;
                            f_max_age = map.next_value::<Option<String>>()?;
                        }
                        "max_count" => {
                            if seen_max_count {
                                return Err(serde::de::Error::duplicate_field("max_count"));
                            }
                            seen_max_count = true;
                            f_max_count = map.next_value::<Option<i64>>()?;
                        }
                        "min_count" => {
                            if seen_min_count {
                                return Err(serde::de::Error::duplicate_field("min_count"));
                            }
                            seen_min_count = true;
                            f_min_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeletionCondition {
                    max_age: f_max_age,
                    max_count: f_max_count,
                    min_count: f_min_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeletionCondition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_age {
            map.serialize_entry("max_age", v)?;
        }
        if let Some(v) = &self.max_count {
            map.serialize_entry("max_count", v)?;
        }
        if let Some(v) = &self.min_count {
            map.serialize_entry("min_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DeletionConfig {
    pub condition: Option<crate::types::sm_common::DeletionCondition>,
    pub schedule: Option<crate::types::sm_common::CronSchedule>,
    #[doc = "The maximum amount of time allowed for snapshot deletion operations."]
    pub time_limit: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DeletionConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DeletionConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DeletionConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_condition: Option<crate::types::sm_common::DeletionCondition> = None;
                let mut seen_condition = false;
                let mut f_schedule: Option<crate::types::sm_common::CronSchedule> = None;
                let mut seen_schedule = false;
                let mut f_time_limit: Option<String> = None;
                let mut seen_time_limit = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "condition" => {
                            if seen_condition {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            seen_condition = true;
                            f_condition = map
                                .next_value::<Option<crate::types::sm_common::DeletionCondition>>(
                                )?;
                        }
                        "schedule" => {
                            if seen_schedule {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            seen_schedule = true;
                            f_schedule =
                                map.next_value::<Option<crate::types::sm_common::CronSchedule>>()?;
                        }
                        "time_limit" => {
                            if seen_time_limit {
                                return Err(serde::de::Error::duplicate_field("time_limit"));
                            }
                            seen_time_limit = true;
                            f_time_limit = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DeletionConfig {
                    condition: f_condition,
                    schedule: f_schedule,
                    time_limit: f_time_limit,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DeletionConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.condition {
            map.serialize_entry("condition", v)?;
        }
        if let Some(v) = &self.schedule {
            map.serialize_entry("schedule", v)?;
        }
        if let Some(v) = &self.time_limit {
            map.serialize_entry("time_limit", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionInfo {
    #[doc = "The reason policy execution failed, if applicable."]
    pub cause: Option<String>,
    #[doc = "A detailed message about the policy execution result."]
    pub message: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecutionInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecutionInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecutionInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cause: Option<String> = None;
                let mut seen_cause = false;
                let mut f_message: Option<String> = None;
                let mut seen_message = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cause" => {
                            if seen_cause {
                                return Err(serde::de::Error::duplicate_field("cause"));
                            }
                            seen_cause = true;
                            f_cause = map.next_value::<Option<String>>()?;
                        }
                        "message" => {
                            if seen_message {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            seen_message = true;
                            f_message = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecutionInfo {
                    cause: f_cause,
                    message: f_message,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecutionInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cause {
            map.serialize_entry("cause", v)?;
        }
        if let Some(v) = &self.message {
            map.serialize_entry("message", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub info: Option<crate::types::sm_common::ExecutionInfo>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExecutionMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExecutionMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExecutionMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_info: Option<crate::types::sm_common::ExecutionInfo> = None;
                let mut seen_info = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "info" => {
                            if seen_info {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            seen_info = true;
                            f_info =
                                map.next_value::<Option<crate::types::sm_common::ExecutionInfo>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExecutionMetadata {
                    info: f_info,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExecutionMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.info {
            map.serialize_entry("info", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainedPolicy {
    pub creation: Option<crate::types::sm_common::StateMetadata>,
    pub deletion: Option<crate::types::sm_common::StateMetadata>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub policy_primary_term: Option<i64>,
    pub policy_seq_no: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainedPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainedPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainedPolicy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_creation: Option<crate::types::sm_common::StateMetadata> = None;
                let mut seen_creation = false;
                let mut f_deletion: Option<crate::types::sm_common::StateMetadata> = None;
                let mut seen_deletion = false;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_policy_primary_term: Option<i64> = None;
                let mut seen_policy_primary_term = false;
                let mut f_policy_seq_no: Option<i64> = None;
                let mut seen_policy_seq_no = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "creation" => {
                            if seen_creation {
                                return Err(serde::de::Error::duplicate_field("creation"));
                            }
                            seen_creation = true;
                            f_creation =
                                map.next_value::<Option<crate::types::sm_common::StateMetadata>>()?;
                        }
                        "deletion" => {
                            if seen_deletion {
                                return Err(serde::de::Error::duplicate_field("deletion"));
                            }
                            seen_deletion = true;
                            f_deletion =
                                map.next_value::<Option<crate::types::sm_common::StateMetadata>>()?;
                        }
                        "enabled" => {
                            if seen_enabled {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            seen_enabled = true;
                            f_enabled = map.next_value::<Option<bool>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "policy_primary_term" => {
                            if seen_policy_primary_term {
                                return Err(serde::de::Error::duplicate_field(
                                    "policy_primary_term",
                                ));
                            }
                            seen_policy_primary_term = true;
                            f_policy_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "policy_seq_no" => {
                            if seen_policy_seq_no {
                                return Err(serde::de::Error::duplicate_field("policy_seq_no"));
                            }
                            seen_policy_seq_no = true;
                            f_policy_seq_no = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainedPolicy {
                    creation: f_creation,
                    deletion: f_deletion,
                    enabled: f_enabled,
                    name: f_name,
                    policy_primary_term: f_policy_primary_term,
                    policy_seq_no: f_policy_seq_no,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainedPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.creation {
            map.serialize_entry("creation", v)?;
        }
        if let Some(v) = &self.deletion {
            map.serialize_entry("deletion", v)?;
        }
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.policy_primary_term {
            map.serialize_entry("policy_primary_term", v)?;
        }
        if let Some(v) = &self.policy_seq_no {
            map.serialize_entry("policy_seq_no", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetPoliciesResponse {
    #[doc = "A list of snapshot management policies."]
    pub policies: Vec<crate::types::sm_common::ListedPolicy>,
    #[doc = "The total number of policies found."]
    pub total_policies: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GetPoliciesResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GetPoliciesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GetPoliciesResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_policies: Option<Vec<crate::types::sm_common::ListedPolicy>> = None;
                let mut f_total_policies: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "policies" => {
                            if f_policies.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            f_policies = Some(
                                map.next_value::<Vec<crate::types::sm_common::ListedPolicy>>()?,
                            );
                        }
                        "total_policies" => {
                            if f_total_policies.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_policies"));
                            }
                            f_total_policies = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetPoliciesResponse {
                    policies: f_policies
                        .ok_or_else(|| serde::de::Error::missing_field("policies"))?,
                    total_policies: f_total_policies
                        .ok_or_else(|| serde::de::Error::missing_field("total_policies"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GetPoliciesResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("policies", &self.policies)?;
        map.serialize_entry("total_policies", &self.total_policies)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalConfig {
    #[doc = "The number of time units between executions."]
    pub period: i64,
    #[doc = "When the interval schedule should start."]
    pub start_time: i64,
    #[doc = "The time unit for the interval period (Days, Hours, or Minutes)."]
    pub unit: crate::types::sm_common::IntervalUnit,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_period: Option<i64> = None;
                let mut f_start_time: Option<i64> = None;
                let mut f_unit: Option<crate::types::sm_common::IntervalUnit> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "period" => {
                            if f_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            f_period = Some(map.next_value::<i64>()?);
                        }
                        "start_time" => {
                            if f_start_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            f_start_time = Some(map.next_value::<i64>()?);
                        }
                        "unit" => {
                            if f_unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            f_unit =
                                Some(map.next_value::<crate::types::sm_common::IntervalUnit>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalConfig {
                    period: f_period.ok_or_else(|| serde::de::Error::missing_field("period"))?,
                    start_time: f_start_time
                        .ok_or_else(|| serde::de::Error::missing_field("start_time"))?,
                    unit: f_unit.ok_or_else(|| serde::de::Error::missing_field("unit"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("period", &self.period)?;
        map.serialize_entry("start_time", &self.start_time)?;
        map.serialize_entry("unit", &self.unit)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct IntervalSchedule {
    pub interval: Option<crate::types::sm_common::IntervalConfig>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IntervalSchedule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IntervalSchedule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IntervalSchedule object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_interval: Option<crate::types::sm_common::IntervalConfig> = None;
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
                            f_interval = map
                                .next_value::<Option<crate::types::sm_common::IntervalConfig>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IntervalSchedule {
                    interval: f_interval,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IntervalSchedule {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntervalUnit {
    #[doc = "`Days`"]
    Days,
    #[doc = "`Hours`"]
    Hours,
    #[doc = "`Minutes`"]
    Minutes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntervalUnit {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntervalUnit::Days => "Days",
            IntervalUnit::Hours => "Hours",
            IntervalUnit::Minutes => "Minutes",
            IntervalUnit::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntervalUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntervalUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Days" => IntervalUnit::Days,
            "Hours" => IntervalUnit::Hours,
            "Minutes" => IntervalUnit::Minutes,
            _ => IntervalUnit::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ListedPolicy {
    #[doc = "The policy ID."]
    pub id: String,
    #[doc = "The primary term for optimistic concurrency control."]
    pub primary_term: Option<i64>,
    #[doc = "The sequence number for optimistic concurrency control."]
    pub seq_no: Option<i64>,
    #[doc = "The complete snapshot management policy configuration."]
    pub sm_policy: crate::types::sm_common::SMPolicy,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ListedPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ListedPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ListedPolicy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<i64> = None;
                let mut seen_seq_no = false;
                let mut f_sm_policy: Option<crate::types::sm_common::SMPolicy> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
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
                            f_seq_no = map.next_value::<Option<i64>>()?;
                        }
                        "sm_policy" => {
                            if f_sm_policy.is_some() {
                                return Err(serde::de::Error::duplicate_field("sm_policy"));
                            }
                            f_sm_policy =
                                Some(map.next_value::<crate::types::sm_common::SMPolicy>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ListedPolicy {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    sm_policy: f_sm_policy
                        .ok_or_else(|| serde::de::Error::missing_field("sm_policy"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ListedPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        map.serialize_entry("sm_policy", &self.sm_policy)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationChannel {
    pub id: String,
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
                let mut f_id: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationChannel {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("id"))?,
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
        map.serialize_entry("id", &self.id)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationConditions {
    #[doc = "Whether to send notifications for a successful snapshot creation."]
    pub creation: Option<bool>,
    #[doc = "Whether to send notifications for a successful snapshot deletion."]
    pub deletion: Option<bool>,
    #[doc = "Whether to send notifications for failed operations."]
    pub failure: Option<bool>,
    #[doc = "Whether to send notifications when operations exceed time limits."]
    pub time_limit_exceeded: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NotificationConditions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NotificationConditions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NotificationConditions object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_creation: Option<bool> = None;
                let mut seen_creation = false;
                let mut f_deletion: Option<bool> = None;
                let mut seen_deletion = false;
                let mut f_failure: Option<bool> = None;
                let mut seen_failure = false;
                let mut f_time_limit_exceeded: Option<bool> = None;
                let mut seen_time_limit_exceeded = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "creation" => {
                            if seen_creation {
                                return Err(serde::de::Error::duplicate_field("creation"));
                            }
                            seen_creation = true;
                            f_creation = map.next_value::<Option<bool>>()?;
                        }
                        "deletion" => {
                            if seen_deletion {
                                return Err(serde::de::Error::duplicate_field("deletion"));
                            }
                            seen_deletion = true;
                            f_deletion = map.next_value::<Option<bool>>()?;
                        }
                        "failure" => {
                            if seen_failure {
                                return Err(serde::de::Error::duplicate_field("failure"));
                            }
                            seen_failure = true;
                            f_failure = map.next_value::<Option<bool>>()?;
                        }
                        "time_limit_exceeded" => {
                            if seen_time_limit_exceeded {
                                return Err(serde::de::Error::duplicate_field(
                                    "time_limit_exceeded",
                                ));
                            }
                            seen_time_limit_exceeded = true;
                            f_time_limit_exceeded = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationConditions {
                    creation: f_creation,
                    deletion: f_deletion,
                    failure: f_failure,
                    time_limit_exceeded: f_time_limit_exceeded,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NotificationConditions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.creation {
            map.serialize_entry("creation", v)?;
        }
        if let Some(v) = &self.deletion {
            map.serialize_entry("deletion", v)?;
        }
        if let Some(v) = &self.failure {
            map.serialize_entry("failure", v)?;
        }
        if let Some(v) = &self.time_limit_exceeded {
            map.serialize_entry("time_limit_exceeded", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub channel: Option<crate::types::sm_common::NotificationChannel>,
    pub conditions: Option<crate::types::sm_common::NotificationConditions>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NotificationConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NotificationConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NotificationConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_channel: Option<crate::types::sm_common::NotificationChannel> = None;
                let mut seen_channel = false;
                let mut f_conditions: Option<crate::types::sm_common::NotificationConditions> =
                    None;
                let mut seen_conditions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "channel" => {
                            if seen_channel {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            seen_channel = true;
                            f_channel = map
                                .next_value::<Option<crate::types::sm_common::NotificationChannel>>(
                                )?;
                        }
                        "conditions" => {
                            if seen_conditions {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            seen_conditions = true;
                            f_conditions = map.next_value::<Option<crate::types::sm_common::NotificationConditions>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NotificationConfig {
                    channel: f_channel,
                    conditions: f_conditions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NotificationConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.channel {
            map.serialize_entry("channel", v)?;
        }
        if let Some(v) = &self.conditions {
            map.serialize_entry("conditions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PolicyExplanation {
    pub policies: Option<Vec<crate::types::sm_common::ExplainedPolicy>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PolicyExplanation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PolicyExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PolicyExplanation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_policies: Option<Vec<crate::types::sm_common::ExplainedPolicy>> = None;
                let mut seen_policies = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "policies" => {
                            if seen_policies {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            seen_policies = true;
                            f_policies = map.next_value::<Option<Vec<crate::types::sm_common::ExplainedPolicy>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PolicyExplanation {
                    policies: f_policies,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PolicyExplanation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.policies {
            map.serialize_entry("policies", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PolicyResponse {
    #[doc = "The policy ID."]
    pub id: String,
    #[doc = "The primary term for optimistic concurrency control."]
    pub primary_term: i64,
    #[doc = "The sequence number for optimistic concurrency control."]
    pub seq_no: i64,
    #[doc = "The version number of the policy document."]
    pub version: i64,
    #[doc = "The complete snapshot management policy configuration."]
    pub sm_policy: crate::types::sm_common::SMPolicy,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PolicyResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PolicyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PolicyResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_seq_no: Option<i64> = None;
                let mut f_version: Option<i64> = None;
                let mut f_sm_policy: Option<crate::types::sm_common::SMPolicy> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<String>()?);
                        }
                        "_primary_term" => {
                            if f_primary_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            f_primary_term = Some(map.next_value::<i64>()?);
                        }
                        "_seq_no" => {
                            if f_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            f_seq_no = Some(map.next_value::<i64>()?);
                        }
                        "_version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            f_version = Some(map.next_value::<i64>()?);
                        }
                        "sm_policy" => {
                            if f_sm_policy.is_some() {
                                return Err(serde::de::Error::duplicate_field("sm_policy"));
                            }
                            f_sm_policy =
                                Some(map.next_value::<crate::types::sm_common::SMPolicy>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PolicyResponse {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    primary_term: f_primary_term
                        .ok_or_else(|| serde::de::Error::missing_field("_primary_term"))?,
                    seq_no: f_seq_no.ok_or_else(|| serde::de::Error::missing_field("_seq_no"))?,
                    version: f_version
                        .ok_or_else(|| serde::de::Error::missing_field("_version"))?,
                    sm_policy: f_sm_policy
                        .ok_or_else(|| serde::de::Error::missing_field("sm_policy"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PolicyResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_primary_term", &self.primary_term)?;
        map.serialize_entry("_seq_no", &self.seq_no)?;
        map.serialize_entry("_version", &self.version)?;
        map.serialize_entry("sm_policy", &self.sm_policy)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RetryMetadata {
    #[doc = "The number of retry attempts made."]
    pub count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RetryMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RetryMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RetryMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RetryMetadata {
                    count: f_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RetryMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SMPolicy {
    pub creation: crate::types::sm_common::CreationConfig,
    #[doc = "The configuration for snapshot deletion."]
    pub deletion: Option<crate::types::sm_common::DeletionConfig>,
    pub description: String,
    pub enabled: Option<bool>,
    #[doc = "When the policy was last enabled."]
    pub enabled_time: Option<i64>,
    #[doc = "When the policy was last modified."]
    pub last_updated_time: Option<i64>,
    #[doc = "The unique identifier for the snapshot policy."]
    pub name: String,
    pub notification: Option<crate::types::sm_common::NotificationConfig>,
    #[doc = "The system-generated schedule metadata returned by the API."]
    pub schedule: Option<crate::types::sm_common::IntervalSchedule>,
    #[doc = "The version number of the policy schema."]
    pub schema_version: Option<i64>,
    pub snapshot_config: crate::types::sm_common::SnapshotConfig,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SMPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SMPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SMPolicy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_creation: Option<crate::types::sm_common::CreationConfig> = None;
                let mut f_deletion: Option<crate::types::sm_common::DeletionConfig> = None;
                let mut seen_deletion = false;
                let mut f_description: Option<String> = None;
                let mut f_enabled: Option<bool> = None;
                let mut seen_enabled = false;
                let mut f_enabled_time: Option<i64> = None;
                let mut seen_enabled_time = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_name: Option<String> = None;
                let mut f_notification: Option<crate::types::sm_common::NotificationConfig> = None;
                let mut seen_notification = false;
                let mut f_schedule: Option<crate::types::sm_common::IntervalSchedule> = None;
                let mut seen_schedule = false;
                let mut f_schema_version: Option<i64> = None;
                let mut seen_schema_version = false;
                let mut f_snapshot_config: Option<crate::types::sm_common::SnapshotConfig> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "creation" => {
                            if f_creation.is_some() {
                                return Err(serde::de::Error::duplicate_field("creation"));
                            }
                            f_creation =
                                Some(map.next_value::<crate::types::sm_common::CreationConfig>()?);
                        }
                        "deletion" => {
                            if seen_deletion {
                                return Err(serde::de::Error::duplicate_field("deletion"));
                            }
                            seen_deletion = true;
                            f_deletion = map
                                .next_value::<Option<crate::types::sm_common::DeletionConfig>>()?;
                        }
                        "description" => {
                            if f_description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            f_description = Some(map.next_value::<String>()?);
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
                            f_enabled_time = map.next_value::<Option<i64>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<String>()?);
                        }
                        "notification" => {
                            if seen_notification {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            seen_notification = true;
                            f_notification = map
                                .next_value::<Option<crate::types::sm_common::NotificationConfig>>(
                                )?;
                        }
                        "schedule" => {
                            if seen_schedule {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            seen_schedule = true;
                            f_schedule = map
                                .next_value::<Option<crate::types::sm_common::IntervalSchedule>>(
                                )?;
                        }
                        "schema_version" => {
                            if seen_schema_version {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            seen_schema_version = true;
                            f_schema_version = map.next_value::<Option<i64>>()?;
                        }
                        "snapshot_config" => {
                            if f_snapshot_config.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot_config"));
                            }
                            f_snapshot_config =
                                Some(map.next_value::<crate::types::sm_common::SnapshotConfig>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SMPolicy {
                    creation: f_creation
                        .ok_or_else(|| serde::de::Error::missing_field("creation"))?,
                    deletion: f_deletion,
                    description: f_description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    enabled: f_enabled,
                    enabled_time: f_enabled_time,
                    last_updated_time: f_last_updated_time,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    notification: f_notification,
                    schedule: f_schedule,
                    schema_version: f_schema_version,
                    snapshot_config: f_snapshot_config
                        .ok_or_else(|| serde::de::Error::missing_field("snapshot_config"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SMPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("creation", &self.creation)?;
        if let Some(v) = &self.deletion {
            map.serialize_entry("deletion", v)?;
        }
        map.serialize_entry("description", &self.description)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.enabled_time {
            map.serialize_entry("enabled_time", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        map.serialize_entry("name", &self.name)?;
        if let Some(v) = &self.notification {
            map.serialize_entry("notification", v)?;
        }
        if let Some(v) = &self.schedule {
            map.serialize_entry("schedule", v)?;
        }
        if let Some(v) = &self.schema_version {
            map.serialize_entry("schema_version", v)?;
        }
        map.serialize_entry("snapshot_config", &self.snapshot_config)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    #[doc = "Default \"yyyy-MM-dd'T'HH:mm:ss\""]
    pub date_format: Option<String>,
    #[doc = "Default false"]
    pub ignore_unavailable: Option<bool>,
    #[doc = "Default true"]
    pub include_global_state: Option<bool>,
    #[doc = "Default \"*\""]
    pub indices: Option<String>,
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "Default false"]
    pub partial: Option<bool>,
    pub repository: String,
    #[doc = "Default UTC"]
    pub timezone: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SnapshotConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SnapshotConfig object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_date_format: Option<String> = None;
                let mut seen_date_format = false;
                let mut f_ignore_unavailable: Option<bool> = None;
                let mut seen_ignore_unavailable = false;
                let mut f_include_global_state: Option<bool> = None;
                let mut seen_include_global_state = false;
                let mut f_indices: Option<String> = None;
                let mut seen_indices = false;
                let mut f_metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut seen_metadata = false;
                let mut f_partial: Option<bool> = None;
                let mut seen_partial = false;
                let mut f_repository: Option<String> = None;
                let mut f_timezone: Option<String> = None;
                let mut seen_timezone = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "date_format" => {
                            if seen_date_format {
                                return Err(serde::de::Error::duplicate_field("date_format"));
                            }
                            seen_date_format = true;
                            f_date_format = map.next_value::<Option<String>>()?;
                        }
                        "ignore_unavailable" => {
                            if seen_ignore_unavailable {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignore_unavailable",
                                ));
                            }
                            seen_ignore_unavailable = true;
                            f_ignore_unavailable = map.next_value::<Option<bool>>()?;
                        }
                        "include_global_state" => {
                            if seen_include_global_state {
                                return Err(serde::de::Error::duplicate_field(
                                    "include_global_state",
                                ));
                            }
                            seen_include_global_state = true;
                            f_include_global_state = map.next_value::<Option<bool>>()?;
                        }
                        "indices" => {
                            if seen_indices {
                                return Err(serde::de::Error::duplicate_field("indices"));
                            }
                            seen_indices = true;
                            f_indices = map.next_value::<Option<String>>()?;
                        }
                        "metadata" => {
                            if seen_metadata {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            seen_metadata = true;
                            f_metadata = map
                                .next_value::<Option<std::collections::BTreeMap<String, String>>>(
                                )?;
                        }
                        "partial" => {
                            if seen_partial {
                                return Err(serde::de::Error::duplicate_field("partial"));
                            }
                            seen_partial = true;
                            f_partial = map.next_value::<Option<bool>>()?;
                        }
                        "repository" => {
                            if f_repository.is_some() {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            f_repository = Some(map.next_value::<String>()?);
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
                Ok(SnapshotConfig {
                    date_format: f_date_format,
                    ignore_unavailable: f_ignore_unavailable,
                    include_global_state: f_include_global_state,
                    indices: f_indices,
                    metadata: f_metadata,
                    partial: f_partial,
                    repository: f_repository
                        .ok_or_else(|| serde::de::Error::missing_field("repository"))?,
                    timezone: f_timezone,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SnapshotConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.date_format {
            map.serialize_entry("date_format", v)?;
        }
        if let Some(v) = &self.ignore_unavailable {
            map.serialize_entry("ignore_unavailable", v)?;
        }
        if let Some(v) = &self.include_global_state {
            map.serialize_entry("include_global_state", v)?;
        }
        if let Some(v) = &self.indices {
            map.serialize_entry("indices", v)?;
        }
        if let Some(v) = &self.metadata {
            map.serialize_entry("metadata", v)?;
        }
        if let Some(v) = &self.partial {
            map.serialize_entry("partial", v)?;
        }
        map.serialize_entry("repository", &self.repository)?;
        if let Some(v) = &self.timezone {
            map.serialize_entry("timezone", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StateMetadata {
    #[doc = "The current state of the policy execution."]
    pub current_state: Option<String>,
    #[doc = "Details about the most recent policy execution."]
    pub latest_execution: Option<crate::types::sm_common::ExecutionMetadata>,
    #[doc = "Information about any retry attempts that occurred."]
    pub retry: Option<crate::types::sm_common::RetryMetadata>,
    #[doc = "Any information about what triggered the policy execution."]
    pub trigger: Option<crate::types::sm_common::TriggerMetadata>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StateMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StateMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StateMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_current_state: Option<String> = None;
                let mut seen_current_state = false;
                let mut f_latest_execution: Option<crate::types::sm_common::ExecutionMetadata> =
                    None;
                let mut seen_latest_execution = false;
                let mut f_retry: Option<crate::types::sm_common::RetryMetadata> = None;
                let mut seen_retry = false;
                let mut f_trigger: Option<crate::types::sm_common::TriggerMetadata> = None;
                let mut seen_trigger = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "current_state" => {
                            if seen_current_state {
                                return Err(serde::de::Error::duplicate_field("current_state"));
                            }
                            seen_current_state = true;
                            f_current_state = map.next_value::<Option<String>>()?;
                        }
                        "latest_execution" => {
                            if seen_latest_execution {
                                return Err(serde::de::Error::duplicate_field("latest_execution"));
                            }
                            seen_latest_execution = true;
                            f_latest_execution = map
                                .next_value::<Option<crate::types::sm_common::ExecutionMetadata>>(
                                )?;
                        }
                        "retry" => {
                            if seen_retry {
                                return Err(serde::de::Error::duplicate_field("retry"));
                            }
                            seen_retry = true;
                            f_retry =
                                map.next_value::<Option<crate::types::sm_common::RetryMetadata>>()?;
                        }
                        "trigger" => {
                            if seen_trigger {
                                return Err(serde::de::Error::duplicate_field("trigger"));
                            }
                            seen_trigger = true;
                            f_trigger = map
                                .next_value::<Option<crate::types::sm_common::TriggerMetadata>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StateMetadata {
                    current_state: f_current_state,
                    latest_execution: f_latest_execution,
                    retry: f_retry,
                    trigger: f_trigger,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StateMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.current_state {
            map.serialize_entry("current_state", v)?;
        }
        if let Some(v) = &self.latest_execution {
            map.serialize_entry("latest_execution", v)?;
        }
        if let Some(v) = &self.retry {
            map.serialize_entry("retry", v)?;
        }
        if let Some(v) = &self.trigger {
            map.serialize_entry("trigger", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TriggerMetadata {
    #[doc = "When the policy was triggered."]
    pub time: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TriggerMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TriggerMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TriggerMetadata object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_time: Option<i64> = None;
                let mut seen_time = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "time" => {
                            if seen_time {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            seen_time = true;
                            f_time = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TriggerMetadata {
                    time: f_time,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TriggerMetadata {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.time {
            map.serialize_entry("time", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
