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

//! Response types of the `ism_common` specification namespace

#![allow(missing_docs)]

#[doc = "An action to perform."]
#[derive(Debug, Clone)]
pub struct Action {
    #[doc = "Adds or removes an index alias."]
    pub alias: Option<crate::types::ism_common::ActionAlias>,
    #[doc = "Sets the allocations settings for index."]
    pub allocation: Option<crate::types::ism_common::ActionAllocation>,
    #[doc = "Closes an index."]
    pub close: Option<crate::types::ism_common::ActionClose>,
    #[doc = "Enables custom operations to be performed on the index."]
    pub custom: Option<crate::types::ism_common::ActionCustom>,
    #[doc = "Deletes an index."]
    pub delete: Option<crate::types::ism_common::ActionDelete>,
    #[doc = "Force merges an index."]
    pub force_merge: Option<crate::types::ism_common::ActionForceMerge>,
    #[doc = "The priority level of the index."]
    pub index_priority: Option<crate::types::ism_common::ActionIndexPriority>,
    #[doc = "Sets a policy which determines when to send a notification regarding the index."]
    pub notification: Option<crate::types::ism_common::ActionNotification>,
    #[doc = "Opens an index."]
    pub open: Option<crate::types::ism_common::ActionOpen>,
    #[doc = "Changes the index permissions to read-only."]
    pub read_only: Option<crate::types::ism_common::ActionReadOnly>,
    #[doc = "Changes the index permissions to read-write."]
    pub read_write: Option<crate::types::ism_common::ActionReadWrite>,
    #[doc = "The number of replicas for the index."]
    pub replica_count: Option<crate::types::ism_common::ActionReplicaCount>,
    #[doc = "The number of times to retry the action."]
    pub retry: Option<crate::types::ism_common::ActionRetry>,
    #[doc = "Sets a rollover policy for the index."]
    pub rollover: Option<crate::types::ism_common::ActionRollover>,
    #[doc = "Determines when to perform an index rollup."]
    pub rollup: Option<crate::types::ism_common::ActionRollup>,
    #[doc = "Determines when to shrink the index."]
    pub shrink: Option<crate::types::ism_common::ActionShrink>,
    #[doc = "Takes a snapshot of the index."]
    pub snapshot: Option<crate::types::ism_common::ActionSnapshot>,
    #[doc = "The amount of time to wait before the action fails."]
    pub timeout: Option<crate::types::ism_common::ActionTimeout>,
    #[doc = "Determines when to perform an index transform."]
    pub transform: Option<crate::types::ism_common::ActionTransform>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Action;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Action object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_alias: Option<crate::types::ism_common::ActionAlias> = None;
                let mut seen_alias = false;
                let mut f_allocation: Option<crate::types::ism_common::ActionAllocation> = None;
                let mut seen_allocation = false;
                let mut f_close: Option<crate::types::ism_common::ActionClose> = None;
                let mut seen_close = false;
                let mut f_custom: Option<crate::types::ism_common::ActionCustom> = None;
                let mut seen_custom = false;
                let mut f_delete: Option<crate::types::ism_common::ActionDelete> = None;
                let mut seen_delete = false;
                let mut f_force_merge: Option<crate::types::ism_common::ActionForceMerge> = None;
                let mut seen_force_merge = false;
                let mut f_index_priority: Option<crate::types::ism_common::ActionIndexPriority> =
                    None;
                let mut seen_index_priority = false;
                let mut f_notification: Option<crate::types::ism_common::ActionNotification> = None;
                let mut seen_notification = false;
                let mut f_open: Option<crate::types::ism_common::ActionOpen> = None;
                let mut seen_open = false;
                let mut f_read_only: Option<crate::types::ism_common::ActionReadOnly> = None;
                let mut seen_read_only = false;
                let mut f_read_write: Option<crate::types::ism_common::ActionReadWrite> = None;
                let mut seen_read_write = false;
                let mut f_replica_count: Option<crate::types::ism_common::ActionReplicaCount> =
                    None;
                let mut seen_replica_count = false;
                let mut f_retry: Option<crate::types::ism_common::ActionRetry> = None;
                let mut seen_retry = false;
                let mut f_rollover: Option<crate::types::ism_common::ActionRollover> = None;
                let mut seen_rollover = false;
                let mut f_rollup: Option<crate::types::ism_common::ActionRollup> = None;
                let mut seen_rollup = false;
                let mut f_shrink: Option<crate::types::ism_common::ActionShrink> = None;
                let mut seen_shrink = false;
                let mut f_snapshot: Option<crate::types::ism_common::ActionSnapshot> = None;
                let mut seen_snapshot = false;
                let mut f_timeout: Option<crate::types::ism_common::ActionTimeout> = None;
                let mut seen_timeout = false;
                let mut f_transform: Option<crate::types::ism_common::ActionTransform> = None;
                let mut seen_transform = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "alias" => {
                            if seen_alias {
                                return Err(serde::de::Error::duplicate_field("alias"));
                            }
                            seen_alias = true;
                            f_alias =
                                map.next_value::<Option<crate::types::ism_common::ActionAlias>>()?;
                        }
                        "allocation" => {
                            if seen_allocation {
                                return Err(serde::de::Error::duplicate_field("allocation"));
                            }
                            seen_allocation = true;
                            f_allocation = map
                                .next_value::<Option<crate::types::ism_common::ActionAllocation>>(
                                )?;
                        }
                        "close" => {
                            if seen_close {
                                return Err(serde::de::Error::duplicate_field("close"));
                            }
                            seen_close = true;
                            f_close =
                                map.next_value::<Option<crate::types::ism_common::ActionClose>>()?;
                        }
                        "custom" => {
                            if seen_custom {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            seen_custom = true;
                            f_custom =
                                map.next_value::<Option<crate::types::ism_common::ActionCustom>>()?;
                        }
                        "delete" => {
                            if seen_delete {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            seen_delete = true;
                            f_delete =
                                map.next_value::<Option<crate::types::ism_common::ActionDelete>>()?;
                        }
                        "force_merge" => {
                            if seen_force_merge {
                                return Err(serde::de::Error::duplicate_field("force_merge"));
                            }
                            seen_force_merge = true;
                            f_force_merge = map
                                .next_value::<Option<crate::types::ism_common::ActionForceMerge>>(
                                )?;
                        }
                        "index_priority" => {
                            if seen_index_priority {
                                return Err(serde::de::Error::duplicate_field("index_priority"));
                            }
                            seen_index_priority = true;
                            f_index_priority = map.next_value::<Option<crate::types::ism_common::ActionIndexPriority>>()?;
                        }
                        "notification" => {
                            if seen_notification {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            seen_notification = true;
                            f_notification = map
                                .next_value::<Option<crate::types::ism_common::ActionNotification>>(
                                )?;
                        }
                        "open" => {
                            if seen_open {
                                return Err(serde::de::Error::duplicate_field("open"));
                            }
                            seen_open = true;
                            f_open =
                                map.next_value::<Option<crate::types::ism_common::ActionOpen>>()?;
                        }
                        "read_only" => {
                            if seen_read_only {
                                return Err(serde::de::Error::duplicate_field("read_only"));
                            }
                            seen_read_only = true;
                            f_read_only = map
                                .next_value::<Option<crate::types::ism_common::ActionReadOnly>>()?;
                        }
                        "read_write" => {
                            if seen_read_write {
                                return Err(serde::de::Error::duplicate_field("read_write"));
                            }
                            seen_read_write = true;
                            f_read_write = map
                                .next_value::<Option<crate::types::ism_common::ActionReadWrite>>(
                                )?;
                        }
                        "replica_count" => {
                            if seen_replica_count {
                                return Err(serde::de::Error::duplicate_field("replica_count"));
                            }
                            seen_replica_count = true;
                            f_replica_count = map
                                .next_value::<Option<crate::types::ism_common::ActionReplicaCount>>(
                                )?;
                        }
                        "retry" => {
                            if seen_retry {
                                return Err(serde::de::Error::duplicate_field("retry"));
                            }
                            seen_retry = true;
                            f_retry =
                                map.next_value::<Option<crate::types::ism_common::ActionRetry>>()?;
                        }
                        "rollover" => {
                            if seen_rollover {
                                return Err(serde::de::Error::duplicate_field("rollover"));
                            }
                            seen_rollover = true;
                            f_rollover = map
                                .next_value::<Option<crate::types::ism_common::ActionRollover>>()?;
                        }
                        "rollup" => {
                            if seen_rollup {
                                return Err(serde::de::Error::duplicate_field("rollup"));
                            }
                            seen_rollup = true;
                            f_rollup =
                                map.next_value::<Option<crate::types::ism_common::ActionRollup>>()?;
                        }
                        "shrink" => {
                            if seen_shrink {
                                return Err(serde::de::Error::duplicate_field("shrink"));
                            }
                            seen_shrink = true;
                            f_shrink =
                                map.next_value::<Option<crate::types::ism_common::ActionShrink>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map
                                .next_value::<Option<crate::types::ism_common::ActionSnapshot>>()?;
                        }
                        "timeout" => {
                            if seen_timeout {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            seen_timeout = true;
                            f_timeout = map
                                .next_value::<Option<crate::types::ism_common::ActionTimeout>>()?;
                        }
                        "transform" => {
                            if seen_transform {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            seen_transform = true;
                            f_transform = map
                                .next_value::<Option<crate::types::ism_common::ActionTransform>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Action {
                    alias: f_alias,
                    allocation: f_allocation,
                    close: f_close,
                    custom: f_custom,
                    delete: f_delete,
                    force_merge: f_force_merge,
                    index_priority: f_index_priority,
                    notification: f_notification,
                    open: f_open,
                    read_only: f_read_only,
                    read_write: f_read_write,
                    replica_count: f_replica_count,
                    retry: f_retry,
                    rollover: f_rollover,
                    rollup: f_rollup,
                    shrink: f_shrink,
                    snapshot: f_snapshot,
                    timeout: f_timeout,
                    transform: f_transform,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.alias {
            map.serialize_entry("alias", v)?;
        }
        if let Some(v) = &self.allocation {
            map.serialize_entry("allocation", v)?;
        }
        if let Some(v) = &self.close {
            map.serialize_entry("close", v)?;
        }
        if let Some(v) = &self.custom {
            map.serialize_entry("custom", v)?;
        }
        if let Some(v) = &self.delete {
            map.serialize_entry("delete", v)?;
        }
        if let Some(v) = &self.force_merge {
            map.serialize_entry("force_merge", v)?;
        }
        if let Some(v) = &self.index_priority {
            map.serialize_entry("index_priority", v)?;
        }
        if let Some(v) = &self.notification {
            map.serialize_entry("notification", v)?;
        }
        if let Some(v) = &self.open {
            map.serialize_entry("open", v)?;
        }
        if let Some(v) = &self.read_only {
            map.serialize_entry("read_only", v)?;
        }
        if let Some(v) = &self.read_write {
            map.serialize_entry("read_write", v)?;
        }
        if let Some(v) = &self.replica_count {
            map.serialize_entry("replica_count", v)?;
        }
        if let Some(v) = &self.retry {
            map.serialize_entry("retry", v)?;
        }
        if let Some(v) = &self.rollover {
            map.serialize_entry("rollover", v)?;
        }
        if let Some(v) = &self.rollup {
            map.serialize_entry("rollup", v)?;
        }
        if let Some(v) = &self.shrink {
            map.serialize_entry("shrink", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        if let Some(v) = &self.timeout {
            map.serialize_entry("timeout", v)?;
        }
        if let Some(v) = &self.transform {
            map.serialize_entry("transform", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionAlias {
    #[doc = "The alias actions to perform."]
    pub actions: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionAlias {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionAlias;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionAlias object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_actions: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_actions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "actions" => {
                            if seen_actions {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            seen_actions = true;
                            f_actions = map.next_value::<Option<
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
                Ok(ActionAlias {
                    actions: f_actions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionAlias {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.actions {
            map.serialize_entry("actions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionAllocation {
    #[doc = "The excluded allocation attributes."]
    pub exclude: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The included allocation attributes."]
    pub include: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The required allocation attributes."]
    pub require: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Whether to wait for the allocation to complete."]
    pub wait_for: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionAllocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionAllocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionAllocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_exclude: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_exclude = false;
                let mut f_include: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_include = false;
                let mut f_require: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_require = false;
                let mut f_wait_for: Option<bool> = None;
                let mut seen_wait_for = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "exclude" => {
                            if seen_exclude {
                                return Err(serde::de::Error::duplicate_field("exclude"));
                            }
                            seen_exclude = true;
                            f_exclude = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "include" => {
                            if seen_include {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            seen_include = true;
                            f_include = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "require" => {
                            if seen_require {
                                return Err(serde::de::Error::duplicate_field("require"));
                            }
                            seen_require = true;
                            f_require = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "wait_for" => {
                            if seen_wait_for {
                                return Err(serde::de::Error::duplicate_field("wait_for"));
                            }
                            seen_wait_for = true;
                            f_wait_for = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionAllocation {
                    exclude: f_exclude,
                    include: f_include,
                    require: f_require,
                    wait_for: f_wait_for,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionAllocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.exclude {
            map.serialize_entry("exclude", v)?;
        }
        if let Some(v) = &self.include {
            map.serialize_entry("include", v)?;
        }
        if let Some(v) = &self.require {
            map.serialize_entry("require", v)?;
        }
        if let Some(v) = &self.wait_for {
            map.serialize_entry("wait_for", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The close action configuration."]
pub type ActionClose = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The custom action configuration."]
pub type ActionCustom = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The delete action configuration."]
pub type ActionDelete = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone)]
pub struct ActionForceMerge {
    #[doc = "The maximum number of segments to merge to."]
    pub max_num_segments: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionForceMerge {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionForceMerge;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionForceMerge object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_max_num_segments: Option<i64> = None;
                let mut seen_max_num_segments = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "max_num_segments" => {
                            if seen_max_num_segments {
                                return Err(serde::de::Error::duplicate_field("max_num_segments"));
                            }
                            seen_max_num_segments = true;
                            f_max_num_segments = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionForceMerge {
                    max_num_segments: f_max_num_segments,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionForceMerge {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.max_num_segments {
            map.serialize_entry("max_num_segments", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionIndexPriority {
    #[doc = "The priority to set for the index."]
    pub priority: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionIndexPriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionIndexPriority;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionIndexPriority object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_priority: Option<f64> = None;
                let mut seen_priority = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionIndexPriority {
                    priority: f_priority,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionIndexPriority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The notification action configuration."]
pub type ActionNotification = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The open action configuration."]
pub type ActionOpen = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The read-only action configuration."]
pub type ActionReadOnly = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The read-write action configuration."]
pub type ActionReadWrite = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone)]
pub struct ActionReplicaCount {
    #[doc = "The number of replicas to set."]
    pub number_of_replicas: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionReplicaCount {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionReplicaCount;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionReplicaCount object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_number_of_replicas: Option<f64> = None;
                let mut seen_number_of_replicas = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "number_of_replicas" => {
                            if seen_number_of_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_replicas",
                                ));
                            }
                            seen_number_of_replicas = true;
                            f_number_of_replicas = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionReplicaCount {
                    number_of_replicas: f_number_of_replicas,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionReplicaCount {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.number_of_replicas {
            map.serialize_entry("number_of_replicas", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionRetry {
    #[doc = "The backoff strategy for retries."]
    pub backoff: Option<String>,
    #[doc = "The number of retry attempts."]
    pub count: Option<i64>,
    #[doc = "The delay between retry attempts."]
    pub delay: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionRetry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionRetry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionRetry object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_backoff: Option<String> = None;
                let mut seen_backoff = false;
                let mut f_count: Option<i64> = None;
                let mut seen_count = false;
                let mut f_delay: Option<String> = None;
                let mut seen_delay = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "backoff" => {
                            if seen_backoff {
                                return Err(serde::de::Error::duplicate_field("backoff"));
                            }
                            seen_backoff = true;
                            f_backoff = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if seen_count {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            seen_count = true;
                            f_count = map.next_value::<Option<i64>>()?;
                        }
                        "delay" => {
                            if seen_delay {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            seen_delay = true;
                            f_delay = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionRetry {
                    backoff: f_backoff,
                    count: f_count,
                    delay: f_delay,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionRetry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.backoff {
            map.serialize_entry("backoff", v)?;
        }
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.delay {
            map.serialize_entry("delay", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionRollover {
    #[doc = "Whether to copy the alias to the new index."]
    pub copy_alias: Option<bool>,
    #[doc = "The minimum document count to trigger rollover."]
    pub min_doc_count: Option<f64>,
    #[doc = "The minimum index age to trigger rollover."]
    pub min_index_age: Option<String>,
    #[doc = "The minimum primary shard size to trigger rollover."]
    pub min_primary_shard_size: Option<String>,
    #[doc = "The minimum size to trigger rollover."]
    pub min_size: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionRollover {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionRollover;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionRollover object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_copy_alias: Option<bool> = None;
                let mut seen_copy_alias = false;
                let mut f_min_doc_count: Option<f64> = None;
                let mut seen_min_doc_count = false;
                let mut f_min_index_age: Option<String> = None;
                let mut seen_min_index_age = false;
                let mut f_min_primary_shard_size: Option<String> = None;
                let mut seen_min_primary_shard_size = false;
                let mut f_min_size: Option<String> = None;
                let mut seen_min_size = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "copy_alias" => {
                            if seen_copy_alias {
                                return Err(serde::de::Error::duplicate_field("copy_alias"));
                            }
                            seen_copy_alias = true;
                            f_copy_alias = map.next_value::<Option<bool>>()?;
                        }
                        "min_doc_count" => {
                            if seen_min_doc_count {
                                return Err(serde::de::Error::duplicate_field("min_doc_count"));
                            }
                            seen_min_doc_count = true;
                            f_min_doc_count = map.next_value::<Option<f64>>()?;
                        }
                        "min_index_age" => {
                            if seen_min_index_age {
                                return Err(serde::de::Error::duplicate_field("min_index_age"));
                            }
                            seen_min_index_age = true;
                            f_min_index_age = map.next_value::<Option<String>>()?;
                        }
                        "min_primary_shard_size" => {
                            if seen_min_primary_shard_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "min_primary_shard_size",
                                ));
                            }
                            seen_min_primary_shard_size = true;
                            f_min_primary_shard_size = map.next_value::<Option<String>>()?;
                        }
                        "min_size" => {
                            if seen_min_size {
                                return Err(serde::de::Error::duplicate_field("min_size"));
                            }
                            seen_min_size = true;
                            f_min_size = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionRollover {
                    copy_alias: f_copy_alias,
                    min_doc_count: f_min_doc_count,
                    min_index_age: f_min_index_age,
                    min_primary_shard_size: f_min_primary_shard_size,
                    min_size: f_min_size,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionRollover {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.copy_alias {
            map.serialize_entry("copy_alias", v)?;
        }
        if let Some(v) = &self.min_doc_count {
            map.serialize_entry("min_doc_count", v)?;
        }
        if let Some(v) = &self.min_index_age {
            map.serialize_entry("min_index_age", v)?;
        }
        if let Some(v) = &self.min_primary_shard_size {
            map.serialize_entry("min_primary_shard_size", v)?;
        }
        if let Some(v) = &self.min_size {
            map.serialize_entry("min_size", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The rollup action configuration."]
pub type ActionRollup = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The shrink action configuration."]
pub type ActionShrink = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone)]
pub struct ActionSnapshot {
    #[doc = "Whether to include the global state in the snapshot."]
    pub include_global_state: Option<bool>,
    #[doc = "The repository to store the snapshot."]
    pub repository: Option<String>,
    #[doc = "The name of the snapshot."]
    pub snapshot: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionSnapshot {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionSnapshot object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_include_global_state: Option<bool> = None;
                let mut seen_include_global_state = false;
                let mut f_repository: Option<String> = None;
                let mut seen_repository = false;
                let mut f_snapshot: Option<String> = None;
                let mut seen_snapshot = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "include_global_state" => {
                            if seen_include_global_state {
                                return Err(serde::de::Error::duplicate_field(
                                    "include_global_state",
                                ));
                            }
                            seen_include_global_state = true;
                            f_include_global_state = map.next_value::<Option<bool>>()?;
                        }
                        "repository" => {
                            if seen_repository {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            seen_repository = true;
                            f_repository = map.next_value::<Option<String>>()?;
                        }
                        "snapshot" => {
                            if seen_snapshot {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            seen_snapshot = true;
                            f_snapshot = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ActionSnapshot {
                    include_global_state: f_include_global_state,
                    repository: f_repository,
                    snapshot: f_snapshot,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionSnapshot {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.include_global_state {
            map.serialize_entry("include_global_state", v)?;
        }
        if let Some(v) = &self.repository {
            map.serialize_entry("repository", v)?;
        }
        if let Some(v) = &self.snapshot {
            map.serialize_entry("snapshot", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ActionTimeout {
    #[doc = "The timeout configuration for the action."]
    pub timeout: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ActionTimeout {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ActionTimeout;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ActionTimeout object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_timeout: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_timeout = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "timeout" => {
                            if seen_timeout {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            seen_timeout = true;
                            f_timeout = map.next_value::<Option<
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
                Ok(ActionTimeout {
                    timeout: f_timeout,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ActionTimeout {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.timeout {
            map.serialize_entry("timeout", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The transform action configuration."]
pub type ActionTransform = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type AddPolicyResponse = crate::types::ism_common::ChangePolicyResponse;

pub type ChangePolicyResponse = crate::types::ism_common::ChangeResponse;

#[derive(Debug, Clone)]
pub struct ChangeResponse {
    #[doc = "The list of indices that failed to update."]
    pub failed_indices: Option<Vec<crate::types::ism_common::FailedIndex>>,
    #[doc = "Whether there were any failures."]
    pub failures: Option<bool>,
    #[doc = "The number of updated indices."]
    pub updated_indices: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ChangeResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ChangeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ChangeResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed_indices: Option<Vec<crate::types::ism_common::FailedIndex>> = None;
                let mut seen_failed_indices = false;
                let mut f_failures: Option<bool> = None;
                let mut seen_failures = false;
                let mut f_updated_indices: Option<f64> = None;
                let mut seen_updated_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed_indices" => {
                            if seen_failed_indices {
                                return Err(serde::de::Error::duplicate_field("failed_indices"));
                            }
                            seen_failed_indices = true;
                            f_failed_indices = map
                                .next_value::<Option<Vec<crate::types::ism_common::FailedIndex>>>(
                                )?;
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<bool>>()?;
                        }
                        "updated_indices" => {
                            if seen_updated_indices {
                                return Err(serde::de::Error::duplicate_field("updated_indices"));
                            }
                            seen_updated_indices = true;
                            f_updated_indices = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ChangeResponse {
                    failed_indices: f_failed_indices,
                    failures: f_failures,
                    updated_indices: f_updated_indices,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ChangeResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.failed_indices {
            map.serialize_entry("failed_indices", v)?;
        }
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        if let Some(v) = &self.updated_indices {
            map.serialize_entry("updated_indices", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    #[doc = "The ID of the notification channel."]
    pub id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Channel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Channel object")
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
                let mut seen_id = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Channel {
                    id: f_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type DeletePolicyResponse = crate::types::common::WriteResponseBase;

#[derive(Debug, Clone)]
pub struct ErrorNotification {
    pub channel: Option<crate::types::ism_common::Channel>,
    pub destination: Option<crate::types::ism_common::ErrorNotificationDestination>,
    #[doc = "The template for error notification messages."]
    pub message_template:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorNotification {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorNotification;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorNotification object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_channel: Option<crate::types::ism_common::Channel> = None;
                let mut seen_channel = false;
                let mut f_destination: Option<
                    crate::types::ism_common::ErrorNotificationDestination,
                > = None;
                let mut seen_destination = false;
                let mut f_message_template: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_message_template = false;
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
                            f_channel =
                                map.next_value::<Option<crate::types::ism_common::Channel>>()?;
                        }
                        "destination" => {
                            if seen_destination {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            seen_destination = true;
                            f_destination = map.next_value::<Option<crate::types::ism_common::ErrorNotificationDestination>>()?;
                        }
                        "message_template" => {
                            if seen_message_template {
                                return Err(serde::de::Error::duplicate_field("message_template"));
                            }
                            seen_message_template = true;
                            f_message_template = map.next_value::<Option<
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
                Ok(ErrorNotification {
                    channel: f_channel,
                    destination: f_destination,
                    message_template: f_message_template,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorNotification {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.channel {
            map.serialize_entry("channel", v)?;
        }
        if let Some(v) = &self.destination {
            map.serialize_entry("destination", v)?;
        }
        if let Some(v) = &self.message_template {
            map.serialize_entry("message_template", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ErrorNotificationChime {
    #[doc = "The URL for the Chime webhook."]
    pub url: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorNotificationChime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorNotificationChime;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorNotificationChime object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_url: Option<String> = None;
                let mut seen_url = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "url" => {
                            if seen_url {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            seen_url = true;
                            f_url = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorNotificationChime {
                    url: f_url,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorNotificationChime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.url {
            map.serialize_entry("url", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ErrorNotificationDestination {
    #[doc = "When the notification destination was last updated."]
    pub last_update_time: Option<i64>,
    #[doc = "The name of the notification destination."]
    pub name: Option<String>,
    pub additional: std::collections::BTreeMap<
        String,
        crate::types::ism_common::ErrorNotificationDestinationAdditional,
    >,
}
impl<'de> serde::Deserialize<'de> for ErrorNotificationDestination {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorNotificationDestination;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorNotificationDestination object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_last_update_time: Option<i64> = None;
                let mut seen_last_update_time = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::ism_common::ErrorNotificationDestinationAdditional,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "last_update_time" => {
                            if seen_last_update_time {
                                return Err(serde::de::Error::duplicate_field("last_update_time"));
                            }
                            seen_last_update_time = true;
                            f_last_update_time = map.next_value::<Option<i64>>()?;
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
                Ok(ErrorNotificationDestination {
                    last_update_time: f_last_update_time,
                    name: f_name,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorNotificationDestination {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.last_update_time {
            map.serialize_entry("last_update_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ErrorNotificationDestinationAdditional {
    raw: Box<serde_json::value::RawValue>,
}
impl ErrorNotificationDestinationAdditional {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `error_notification_chime` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_error_notification_chime(
        &self,
    ) -> std::result::Result<
        crate::types::ism_common::ErrorNotificationChime,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "error_notification_chime")
    }
    #[doc = "Parses the value as the `error_notification_slack` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_error_notification_slack(
        &self,
    ) -> std::result::Result<
        crate::types::ism_common::ErrorNotificationSlack,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "error_notification_slack")
    }
}

#[derive(Debug, Clone)]
pub struct ErrorNotificationSlack {
    pub custom_webhook: Option<crate::types::ism_common::SlackCustomWebhook>,
    #[doc = "The URL for the Slack webhook."]
    pub url: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorNotificationSlack {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorNotificationSlack;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorNotificationSlack object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_custom_webhook: Option<crate::types::ism_common::SlackCustomWebhook> =
                    None;
                let mut seen_custom_webhook = false;
                let mut f_url: Option<String> = None;
                let mut seen_url = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "custom_webhook" => {
                            if seen_custom_webhook {
                                return Err(serde::de::Error::duplicate_field("custom_webhook"));
                            }
                            seen_custom_webhook = true;
                            f_custom_webhook = map
                                .next_value::<Option<crate::types::ism_common::SlackCustomWebhook>>(
                                )?;
                        }
                        "url" => {
                            if seen_url {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            seen_url = true;
                            f_url = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorNotificationSlack {
                    custom_webhook: f_custom_webhook,
                    url: f_url,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorNotificationSlack {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.custom_webhook {
            map.serialize_entry("custom_webhook", v)?;
        }
        if let Some(v) = &self.url {
            map.serialize_entry("url", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainIndexResponse {
    #[doc = "The total number of managed indices."]
    pub total_managed_indices: Option<f64>,
    pub additional: std::collections::BTreeMap<String, crate::types::ism_common::ExplainPolicy>,
}
impl<'de> serde::Deserialize<'de> for ExplainIndexResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainIndexResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainIndexResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_managed_indices: Option<f64> = None;
                let mut seen_total_managed_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::ism_common::ExplainPolicy,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_managed_indices" => {
                            if seen_total_managed_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_managed_indices",
                                ));
                            }
                            seen_total_managed_indices = true;
                            f_total_managed_indices = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainIndexResponse {
                    total_managed_indices: f_total_managed_indices,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainIndexResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total_managed_indices {
            map.serialize_entry("total_managed_indices", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExplainPolicy {
    #[doc = "Whether the policy is enabled for the index."]
    pub enabled: Option<bool>,
    #[doc = "The ID of the OpenDistro policy applied to the index."]
    pub index_opendistro_index_state_management_policy_id: Option<String>,
    #[doc = "The ID of the policy applied to the index."]
    pub index_plugins_index_state_management_policy_id: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExplainPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainPolicy object")
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
                let mut f_index_opendistro_index_state_management_policy_id: Option<String> = None;
                let mut seen_index_opendistro_index_state_management_policy_id = false;
                let mut f_index_plugins_index_state_management_policy_id: Option<String> = None;
                let mut seen_index_plugins_index_state_management_policy_id = false;
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
                        "index.opendistro.index_state_management.policy_id" => {
                            if seen_index_opendistro_index_state_management_policy_id {
                                return Err(serde::de::Error::duplicate_field(
                                    "index.opendistro.index_state_management.policy_id",
                                ));
                            }
                            seen_index_opendistro_index_state_management_policy_id = true;
                            f_index_opendistro_index_state_management_policy_id =
                                map.next_value::<Option<String>>()?;
                        }
                        "index.plugins.index_state_management.policy_id" => {
                            if seen_index_plugins_index_state_management_policy_id {
                                return Err(serde::de::Error::duplicate_field(
                                    "index.plugins.index_state_management.policy_id",
                                ));
                            }
                            seen_index_plugins_index_state_management_policy_id = true;
                            f_index_plugins_index_state_management_policy_id =
                                map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainPolicy {
                    enabled: f_enabled,
                    index_opendistro_index_state_management_policy_id:
                        f_index_opendistro_index_state_management_policy_id,
                    index_plugins_index_state_management_policy_id:
                        f_index_plugins_index_state_management_policy_id,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.enabled {
            map.serialize_entry("enabled", v)?;
        }
        if let Some(v) = &self.index_opendistro_index_state_management_policy_id {
            map.serialize_entry("index.opendistro.index_state_management.policy_id", v)?;
        }
        if let Some(v) = &self.index_plugins_index_state_management_policy_id {
            map.serialize_entry("index.plugins.index_state_management.policy_id", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FailedIndex {
    #[doc = "The name of the failed index."]
    pub index_name: Option<String>,
    #[doc = "The UUID of the failed index."]
    pub index_uuid: Option<String>,
    #[doc = "The reason for the failure."]
    pub reason: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FailedIndex {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FailedIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FailedIndex object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index_name: Option<String> = None;
                let mut seen_index_name = false;
                let mut f_index_uuid: Option<String> = None;
                let mut seen_index_uuid = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index_name" => {
                            if seen_index_name {
                                return Err(serde::de::Error::duplicate_field("index_name"));
                            }
                            seen_index_name = true;
                            f_index_name = map.next_value::<Option<String>>()?;
                        }
                        "index_uuid" => {
                            if seen_index_uuid {
                                return Err(serde::de::Error::duplicate_field("index_uuid"));
                            }
                            seen_index_uuid = true;
                            f_index_uuid = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FailedIndex {
                    index_name: f_index_name,
                    index_uuid: f_index_uuid,
                    reason: f_reason,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FailedIndex {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index_name {
            map.serialize_entry("index_name", v)?;
        }
        if let Some(v) = &self.index_uuid {
            map.serialize_entry("index_uuid", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GetPoliciesResponse {
    #[doc = "The list of policies."]
    pub policies: Option<Vec<crate::types::ism_common::PolicyWithMetadata>>,
    #[doc = "The total number of policies."]
    pub total_policies: Option<f64>,
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
                let mut f_policies: Option<Vec<crate::types::ism_common::PolicyWithMetadata>> =
                    None;
                let mut seen_policies = false;
                let mut f_total_policies: Option<f64> = None;
                let mut seen_total_policies = false;
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
                            f_policies = map.next_value::<Option<Vec<crate::types::ism_common::PolicyWithMetadata>>>()?;
                        }
                        "total_policies" => {
                            if seen_total_policies {
                                return Err(serde::de::Error::duplicate_field("total_policies"));
                            }
                            seen_total_policies = true;
                            f_total_policies = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GetPoliciesResponse {
                    policies: f_policies,
                    total_policies: f_total_policies,
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
        if let Some(v) = &self.policies {
            map.serialize_entry("policies", v)?;
        }
        if let Some(v) = &self.total_policies {
            map.serialize_entry("total_policies", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type GetPolicyResponse = crate::types::ism_common::PolicyWithMetadata;

#[derive(Debug, Clone)]
pub struct IsmTemplate {
    #[doc = "The index patterns for the ISM template."]
    pub index_patterns: Option<Vec<String>>,
    #[doc = "When the ISM template was last updated."]
    pub last_updated_time: Option<i64>,
    #[doc = "The priority of the ISM template."]
    pub priority: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IsmTemplate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IsmTemplate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IsmTemplate object")
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
                let mut seen_index_patterns = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_priority: Option<f64> = None;
                let mut seen_priority = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index_patterns" => {
                            if seen_index_patterns {
                                return Err(serde::de::Error::duplicate_field("index_patterns"));
                            }
                            seen_index_patterns = true;
                            f_index_patterns = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IsmTemplate {
                    index_patterns: f_index_patterns,
                    last_updated_time: f_last_updated_time,
                    priority: f_priority,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IsmTemplate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index_patterns {
            map.serialize_entry("index_patterns", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The policy defines how an index is managed throughout its lifetime."]
#[derive(Debug, Clone)]
pub struct Policy {
    #[doc = "The default state an index is in."]
    pub default_state: Option<String>,
    #[doc = "The description of the policy."]
    pub description: Option<String>,
    #[doc = "The configuration for error notifications."]
    pub error_notification: Option<crate::types::ism_common::ErrorNotification>,
    #[doc = "The ISM template configuration."]
    pub ism_template: Option<crate::types::ism_common::PolicyIsmTemplate>,
    #[doc = "When the policy was last updated."]
    pub last_updated_time: Option<i64>,
    #[doc = "The unique identifier for the policy."]
    pub policy_id: Option<String>,
    #[doc = "The version of the policy schema."]
    pub schema_version: Option<f64>,
    #[doc = "The list of states for the index to transition between."]
    pub states: Option<Vec<crate::types::ism_common::States>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Policy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Policy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Policy object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_default_state: Option<String> = None;
                let mut seen_default_state = false;
                let mut f_description: Option<String> = None;
                let mut seen_description = false;
                let mut f_error_notification: Option<crate::types::ism_common::ErrorNotification> =
                    None;
                let mut seen_error_notification = false;
                let mut f_ism_template: Option<crate::types::ism_common::PolicyIsmTemplate> = None;
                let mut seen_ism_template = false;
                let mut f_last_updated_time: Option<i64> = None;
                let mut seen_last_updated_time = false;
                let mut f_policy_id: Option<String> = None;
                let mut seen_policy_id = false;
                let mut f_schema_version: Option<f64> = None;
                let mut seen_schema_version = false;
                let mut f_states: Option<Vec<crate::types::ism_common::States>> = None;
                let mut seen_states = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "default_state" => {
                            if seen_default_state {
                                return Err(serde::de::Error::duplicate_field("default_state"));
                            }
                            seen_default_state = true;
                            f_default_state = map.next_value::<Option<String>>()?;
                        }
                        "description" => {
                            if seen_description {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            seen_description = true;
                            f_description = map.next_value::<Option<String>>()?;
                        }
                        "error_notification" => {
                            if seen_error_notification {
                                return Err(serde::de::Error::duplicate_field(
                                    "error_notification",
                                ));
                            }
                            seen_error_notification = true;
                            f_error_notification = map
                                .next_value::<Option<crate::types::ism_common::ErrorNotification>>(
                                )?;
                        }
                        "ism_template" => {
                            if seen_ism_template {
                                return Err(serde::de::Error::duplicate_field("ism_template"));
                            }
                            seen_ism_template = true;
                            f_ism_template = map
                                .next_value::<Option<crate::types::ism_common::PolicyIsmTemplate>>(
                                )?;
                        }
                        "last_updated_time" => {
                            if seen_last_updated_time {
                                return Err(serde::de::Error::duplicate_field("last_updated_time"));
                            }
                            seen_last_updated_time = true;
                            f_last_updated_time = map.next_value::<Option<i64>>()?;
                        }
                        "policy_id" => {
                            if seen_policy_id {
                                return Err(serde::de::Error::duplicate_field("policy_id"));
                            }
                            seen_policy_id = true;
                            f_policy_id = map.next_value::<Option<String>>()?;
                        }
                        "schema_version" => {
                            if seen_schema_version {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            seen_schema_version = true;
                            f_schema_version = map.next_value::<Option<f64>>()?;
                        }
                        "states" => {
                            if seen_states {
                                return Err(serde::de::Error::duplicate_field("states"));
                            }
                            seen_states = true;
                            f_states =
                                map.next_value::<Option<Vec<crate::types::ism_common::States>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Policy {
                    default_state: f_default_state,
                    description: f_description,
                    error_notification: f_error_notification,
                    ism_template: f_ism_template,
                    last_updated_time: f_last_updated_time,
                    policy_id: f_policy_id,
                    schema_version: f_schema_version,
                    states: f_states,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Policy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.default_state {
            map.serialize_entry("default_state", v)?;
        }
        if let Some(v) = &self.description {
            map.serialize_entry("description", v)?;
        }
        if let Some(v) = &self.error_notification {
            map.serialize_entry("error_notification", v)?;
        }
        if let Some(v) = &self.ism_template {
            map.serialize_entry("ism_template", v)?;
        }
        if let Some(v) = &self.last_updated_time {
            map.serialize_entry("last_updated_time", v)?;
        }
        if let Some(v) = &self.policy_id {
            map.serialize_entry("policy_id", v)?;
        }
        if let Some(v) = &self.schema_version {
            map.serialize_entry("schema_version", v)?;
        }
        if let Some(v) = &self.states {
            map.serialize_entry("states", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PolicyEnvelope {
    pub policy: Option<crate::types::ism_common::Policy>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PolicyEnvelope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PolicyEnvelope;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PolicyEnvelope object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_policy: Option<crate::types::ism_common::Policy> = None;
                let mut seen_policy = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "policy" => {
                            if seen_policy {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            seen_policy = true;
                            f_policy =
                                map.next_value::<Option<crate::types::ism_common::Policy>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PolicyEnvelope {
                    policy: f_policy,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PolicyEnvelope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.policy {
            map.serialize_entry("policy", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The ISM template configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PolicyIsmTemplate {
    IsmTemplate(crate::types::ism_common::IsmTemplate),
    Array(Vec<crate::types::ism_common::IsmTemplate>),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for PolicyIsmTemplate {
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
            Some(b'{') => serde_json::from_str(text).map(PolicyIsmTemplate::IsmTemplate),
            Some(b'[') => serde_json::from_str(text).map(PolicyIsmTemplate::Array),
            Some(b'n') => return Ok(PolicyIsmTemplate::Null),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a PolicyIsmTemplate value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct PolicyWithMetadata {
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    pub primary_term: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub policy: Option<crate::types::ism_common::Policy>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PolicyWithMetadata {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PolicyWithMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PolicyWithMetadata object")
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
                let mut f_primary_term: Option<f64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_policy: Option<crate::types::ism_common::Policy> = None;
                let mut seen_policy = false;
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
                            f_primary_term = map.next_value::<Option<f64>>()?;
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
                        "policy" => {
                            if seen_policy {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            seen_policy = true;
                            f_policy =
                                map.next_value::<Option<crate::types::ism_common::Policy>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PolicyWithMetadata {
                    id: f_id,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    version: f_version,
                    policy: f_policy,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PolicyWithMetadata {
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
        if let Some(v) = &self.policy {
            map.serialize_entry("policy", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PutPolicyResponse {
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    pub primary_term: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub policy: Option<crate::types::ism_common::PolicyEnvelope>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PutPolicyResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PutPolicyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PutPolicyResponse object")
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
                let mut f_primary_term: Option<f64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_policy: Option<crate::types::ism_common::PolicyEnvelope> = None;
                let mut seen_policy = false;
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
                            f_primary_term = map.next_value::<Option<f64>>()?;
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
                        "policy" => {
                            if seen_policy {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            seen_policy = true;
                            f_policy = map
                                .next_value::<Option<crate::types::ism_common::PolicyEnvelope>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PutPolicyResponse {
                    id: f_id,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    version: f_version,
                    policy: f_policy,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PutPolicyResponse {
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
        if let Some(v) = &self.policy {
            map.serialize_entry("policy", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RefreshSearchAnalyzersResponse {
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "The details of successful analyzer refreshes."]
    pub successful_refresh_details:
        Option<Vec<crate::types::ism_common::RefreshSearchAnalyzersResponseDetails>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RefreshSearchAnalyzersResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RefreshSearchAnalyzersResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RefreshSearchAnalyzersResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut seen_shards = false;
                let mut f_successful_refresh_details: Option<
                    Vec<crate::types::ism_common::RefreshSearchAnalyzersResponseDetails>,
                > = None;
                let mut seen_successful_refresh_details = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardStatistics>>()?;
                        }
                        "successful_refresh_details" => {
                            if seen_successful_refresh_details {
                                return Err(serde::de::Error::duplicate_field(
                                    "successful_refresh_details",
                                ));
                            }
                            seen_successful_refresh_details = true;
                            f_successful_refresh_details = map.next_value::<Option<
                                Vec<
                                    crate::types::ism_common::RefreshSearchAnalyzersResponseDetails,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RefreshSearchAnalyzersResponse {
                    shards: f_shards,
                    successful_refresh_details: f_successful_refresh_details,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RefreshSearchAnalyzersResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.successful_refresh_details {
            map.serialize_entry("successful_refresh_details", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RefreshSearchAnalyzersResponseDetails {
    #[doc = "The name of the index."]
    pub index: Option<String>,
    #[doc = "The list of refreshed analyzers."]
    pub refreshed_analyzers: Option<Vec<String>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RefreshSearchAnalyzersResponseDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RefreshSearchAnalyzersResponseDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RefreshSearchAnalyzersResponseDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_refreshed_analyzers: Option<Vec<String>> = None;
                let mut seen_refreshed_analyzers = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "refreshed_analyzers" => {
                            if seen_refreshed_analyzers {
                                return Err(serde::de::Error::duplicate_field(
                                    "refreshed_analyzers",
                                ));
                            }
                            seen_refreshed_analyzers = true;
                            f_refreshed_analyzers = map.next_value::<Option<Vec<String>>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RefreshSearchAnalyzersResponseDetails {
                    index: f_index,
                    refreshed_analyzers: f_refreshed_analyzers,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RefreshSearchAnalyzersResponseDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.refreshed_analyzers {
            map.serialize_entry("refreshed_analyzers", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type RemovePolicyResponse = crate::types::ism_common::ChangePolicyResponse;

pub type RetryIndexResponse = crate::types::ism_common::ChangeResponse;

#[derive(Debug, Clone)]
pub struct SlackCustomWebhook {
    #[doc = "The header parameters for the custom Slack webhook."]
    pub header_params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The host for the custom Slack webhook."]
    pub host: Option<String>,
    #[doc = "The password for the custom Slack webhook."]
    pub password: Option<String>,
    #[doc = "The path for the custom Slack webhook."]
    pub path: Option<String>,
    #[doc = "The port for the custom Slack webhook."]
    pub port: Option<i64>,
    #[doc = "The query parameters for the custom Slack webhook."]
    pub query_params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The scheme for the custom Slack webhook."]
    pub scheme: Option<String>,
    #[doc = "The URL for the custom Slack webhook."]
    pub url: Option<String>,
    #[doc = "The username for the custom Slack webhook."]
    pub username: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SlackCustomWebhook {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SlackCustomWebhook;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SlackCustomWebhook object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_header_params: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_header_params = false;
                let mut f_host: Option<String> = None;
                let mut seen_host = false;
                let mut f_password: Option<String> = None;
                let mut seen_password = false;
                let mut f_path: Option<String> = None;
                let mut seen_path = false;
                let mut f_port: Option<i64> = None;
                let mut seen_port = false;
                let mut f_query_params: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_query_params = false;
                let mut f_scheme: Option<String> = None;
                let mut seen_scheme = false;
                let mut f_url: Option<String> = None;
                let mut seen_url = false;
                let mut f_username: Option<String> = None;
                let mut seen_username = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "header_params" => {
                            if seen_header_params {
                                return Err(serde::de::Error::duplicate_field("header_params"));
                            }
                            seen_header_params = true;
                            f_header_params = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "host" => {
                            if seen_host {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            seen_host = true;
                            f_host = map.next_value::<Option<String>>()?;
                        }
                        "password" => {
                            if seen_password {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            seen_password = true;
                            f_password = map.next_value::<Option<String>>()?;
                        }
                        "path" => {
                            if seen_path {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            seen_path = true;
                            f_path = map.next_value::<Option<String>>()?;
                        }
                        "port" => {
                            if seen_port {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            seen_port = true;
                            f_port = map.next_value::<Option<i64>>()?;
                        }
                        "query_params" => {
                            if seen_query_params {
                                return Err(serde::de::Error::duplicate_field("query_params"));
                            }
                            seen_query_params = true;
                            f_query_params = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "scheme" => {
                            if seen_scheme {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            seen_scheme = true;
                            f_scheme = map.next_value::<Option<String>>()?;
                        }
                        "url" => {
                            if seen_url {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            seen_url = true;
                            f_url = map.next_value::<Option<String>>()?;
                        }
                        "username" => {
                            if seen_username {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            seen_username = true;
                            f_username = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SlackCustomWebhook {
                    header_params: f_header_params,
                    host: f_host,
                    password: f_password,
                    path: f_path,
                    port: f_port,
                    query_params: f_query_params,
                    scheme: f_scheme,
                    url: f_url,
                    username: f_username,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SlackCustomWebhook {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.header_params {
            map.serialize_entry("header_params", v)?;
        }
        if let Some(v) = &self.host {
            map.serialize_entry("host", v)?;
        }
        if let Some(v) = &self.password {
            map.serialize_entry("password", v)?;
        }
        if let Some(v) = &self.path {
            map.serialize_entry("path", v)?;
        }
        if let Some(v) = &self.port {
            map.serialize_entry("port", v)?;
        }
        if let Some(v) = &self.query_params {
            map.serialize_entry("query_params", v)?;
        }
        if let Some(v) = &self.scheme {
            map.serialize_entry("scheme", v)?;
        }
        if let Some(v) = &self.url {
            map.serialize_entry("url", v)?;
        }
        if let Some(v) = &self.username {
            map.serialize_entry("username", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The list of actions to perform and transitions to enter a new state."]
#[derive(Debug, Clone)]
pub struct States {
    #[doc = "The list of actions to perform."]
    pub actions: Option<Vec<crate::types::ism_common::Action>>,
    #[doc = "The name of the state."]
    pub name: Option<String>,
    #[doc = "The list of transitions to enter a new state."]
    pub transitions: Option<Vec<crate::types::ism_common::Transition>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for States {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = States;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a States object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_actions: Option<Vec<crate::types::ism_common::Action>> = None;
                let mut seen_actions = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_transitions: Option<Vec<crate::types::ism_common::Transition>> = None;
                let mut seen_transitions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "actions" => {
                            if seen_actions {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            seen_actions = true;
                            f_actions =
                                map.next_value::<Option<Vec<crate::types::ism_common::Action>>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "transitions" => {
                            if seen_transitions {
                                return Err(serde::de::Error::duplicate_field("transitions"));
                            }
                            seen_transitions = true;
                            f_transitions = map
                                .next_value::<Option<Vec<crate::types::ism_common::Transition>>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(States {
                    actions: f_actions,
                    name: f_name,
                    transitions: f_transitions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for States {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.actions {
            map.serialize_entry("actions", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.transitions {
            map.serialize_entry("transitions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A transition into a new state."]
#[derive(Debug, Clone)]
pub struct Transition {
    #[doc = "The conditions for the transition."]
    pub conditions: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The name of the state to transition to."]
    pub state_name: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Transition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Transition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Transition object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_conditions: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_conditions = false;
                let mut f_state_name: Option<String> = None;
                let mut seen_state_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "conditions" => {
                            if seen_conditions {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            seen_conditions = true;
                            f_conditions = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "state_name" => {
                            if seen_state_name {
                                return Err(serde::de::Error::duplicate_field("state_name"));
                            }
                            seen_state_name = true;
                            f_state_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Transition {
                    conditions: f_conditions,
                    state_name: f_state_name,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Transition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.conditions {
            map.serialize_entry("conditions", v)?;
        }
        if let Some(v) = &self.state_name {
            map.serialize_entry("state_name", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
