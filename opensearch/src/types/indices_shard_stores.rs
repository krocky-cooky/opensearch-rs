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

//! Response types of the `indices_shard_stores` specification namespace

#![allow(missing_docs)]

#[doc = "The shard store information for indices."]
#[derive(Debug, Clone)]
pub struct IndicesShardStores {
    #[doc = "The store information for each shard."]
    pub shards:
        std::collections::BTreeMap<String, crate::types::indices_shard_stores::ShardStoreWrapper>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndicesShardStores {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndicesShardStores;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndicesShardStores object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_shards: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::indices_shard_stores::ShardStoreWrapper,
                    >,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            f_shards = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::indices_shard_stores::ShardStoreWrapper,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndicesShardStores {
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("shards"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndicesShardStores {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("shards", &self.shards)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The store information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardStore {
    pub allocation: crate::types::indices_shard_stores::ShardStoreAllocation,
    pub allocation_id: Option<crate::types::common::Id>,
    pub store_exception: Option<crate::types::indices_shard_stores::ShardStoreException>,
    pub additional: std::collections::BTreeMap<String, crate::types::common::NodeAttributes>,
}
impl<'de> serde::Deserialize<'de> for ShardStore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation: Option<
                    crate::types::indices_shard_stores::ShardStoreAllocation,
                > = None;
                let mut f_allocation_id: Option<crate::types::common::Id> = None;
                let mut seen_allocation_id = false;
                let mut f_store_exception: Option<
                    crate::types::indices_shard_stores::ShardStoreException,
                > = None;
                let mut seen_store_exception = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::NodeAttributes,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation" => {
                            if f_allocation.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocation"));
                            }
                            f_allocation = Some(map.next_value::<crate::types::indices_shard_stores::ShardStoreAllocation>()?);
                        }
                        "allocation_id" => {
                            if seen_allocation_id {
                                return Err(serde::de::Error::duplicate_field("allocation_id"));
                            }
                            seen_allocation_id = true;
                            f_allocation_id =
                                map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "store_exception" => {
                            if seen_store_exception {
                                return Err(serde::de::Error::duplicate_field("store_exception"));
                            }
                            seen_store_exception = true;
                            f_store_exception = map.next_value::<Option<crate::types::indices_shard_stores::ShardStoreException>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStore {
                    allocation: f_allocation
                        .ok_or_else(|| serde::de::Error::missing_field("allocation"))?,
                    allocation_id: f_allocation_id,
                    store_exception: f_store_exception,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allocation", &self.allocation)?;
        if let Some(v) = &self.allocation_id {
            map.serialize_entry("allocation_id", v)?;
        }
        if let Some(v) = &self.store_exception {
            map.serialize_entry("store_exception", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The allocation type of the shard store."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardStoreAllocation {
    #[doc = "`primary`"]
    Primary,
    #[doc = "`replica`"]
    Replica,
    #[doc = "`unused`"]
    Unused,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardStoreAllocation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardStoreAllocation::Primary => "primary",
            ShardStoreAllocation::Replica => "replica",
            ShardStoreAllocation::Unused => "unused",
            ShardStoreAllocation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardStoreAllocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardStoreAllocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "primary" => ShardStoreAllocation::Primary,
            "replica" => ShardStoreAllocation::Replica,
            "unused" => ShardStoreAllocation::Unused,
            _ => ShardStoreAllocation::Unknown(s),
        })
    }
}

#[doc = "The exception information for a shard store."]
#[derive(Debug, Clone)]
pub struct ShardStoreException {
    #[doc = "The reason for the store exception."]
    pub reason: String,
    #[doc = "The type of store exception."]
    pub r#type: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStoreException {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStoreException;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStoreException object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_reason: Option<String> = None;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<String>()?);
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
                Ok(ShardStoreException {
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStoreException {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The wrapper containing store information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardStoreWrapper {
    #[doc = "The list of stores for the shard."]
    pub stores: Vec<crate::types::indices_shard_stores::ShardStore>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStoreWrapper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStoreWrapper;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStoreWrapper object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stores: Option<Vec<crate::types::indices_shard_stores::ShardStore>> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stores" => {
                            if f_stores.is_some() {
                                return Err(serde::de::Error::duplicate_field("stores"));
                            }
                            f_stores = Some(map.next_value::<Vec<crate::types::indices_shard_stores::ShardStore>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStoreWrapper {
                    stores: f_stores.ok_or_else(|| serde::de::Error::missing_field("stores"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStoreWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("stores", &self.stores)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The response of the `indices.shard_stores` API"]
pub type ShardStoresResponse = crate::types::indices::ShardStoresResponse;
