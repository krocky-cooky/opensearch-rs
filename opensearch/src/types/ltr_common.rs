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

//! Response types of the `ltr_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct AcknowledgedResponse {
    #[doc = "Indicates whether the request was acknowledged."]
    pub acknowledged: Option<bool>,
    #[doc = "The name of the index."]
    pub index: Option<String>,
    #[doc = "Indicates whether the required active shards were acknowledged."]
    pub shards_acknowledged: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AcknowledgedResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AcknowledgedResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AcknowledgedResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_acknowledged: Option<bool> = None;
                let mut seen_acknowledged = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_shards_acknowledged: Option<bool> = None;
                let mut seen_shards_acknowledged = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "acknowledged" => {
                            if seen_acknowledged {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            seen_acknowledged = true;
                            f_acknowledged = map.next_value::<Option<bool>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "shards_acknowledged" => {
                            if seen_shards_acknowledged {
                                return Err(serde::de::Error::duplicate_field(
                                    "shards_acknowledged",
                                ));
                            }
                            seen_shards_acknowledged = true;
                            f_shards_acknowledged = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AcknowledgedResponse {
                    acknowledged: f_acknowledged,
                    index: f_index,
                    shards_acknowledged: f_shards_acknowledged,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AcknowledgedResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.acknowledged {
            map.serialize_entry("acknowledged", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.shards_acknowledged {
            map.serialize_entry("shards_acknowledged", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Aggregate cache statistics across all nodes."]
#[derive(Debug, Clone)]
pub struct CacheAllStats {
    pub features: Option<crate::types::ltr_common::CacheItemStats>,
    pub featuresets: Option<crate::types::ltr_common::CacheItemStats>,
    pub models: Option<crate::types::ltr_common::CacheItemStats>,
    pub total: Option<crate::types::ltr_common::CacheItemStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheAllStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheAllStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheAllStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_features: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_features = false;
                let mut f_featuresets: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_featuresets = false;
                let mut f_models: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_models = false;
                let mut f_total: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "features" => {
                            if seen_features {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            seen_features = true;
                            f_features = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "featuresets" => {
                            if seen_featuresets {
                                return Err(serde::de::Error::duplicate_field("featuresets"));
                            }
                            seen_featuresets = true;
                            f_featuresets = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "models" => {
                            if seen_models {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            seen_models = true;
                            f_models = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CacheAllStats {
                    features: f_features,
                    featuresets: f_featuresets,
                    models: f_models,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheAllStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.features {
            map.serialize_entry("features", v)?;
        }
        if let Some(v) = &self.featuresets {
            map.serialize_entry("featuresets", v)?;
        }
        if let Some(v) = &self.models {
            map.serialize_entry("models", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CacheItemStats {
    #[doc = "Count of cached items."]
    pub count: Option<i64>,
    #[doc = "Memory usage in bytes."]
    pub ram: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheItemStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheItemStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheItemStats object")
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
                let mut f_ram: Option<i64> = None;
                let mut seen_ram = false;
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
                        "ram" => {
                            if seen_ram {
                                return Err(serde::de::Error::duplicate_field("ram"));
                            }
                            seen_ram = true;
                            f_ram = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CacheItemStats {
                    count: f_count,
                    ram: f_ram,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheItemStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.count {
            map.serialize_entry("count", v)?;
        }
        if let Some(v) = &self.ram {
            map.serialize_entry("ram", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStat {
    pub entry_count: Option<i64>,
    pub eviction_count: Option<i64>,
    pub hit_count: Option<i64>,
    pub memory_usage_in_bytes: Option<i64>,
    pub miss_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheStat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheStat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheStat object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_entry_count: Option<i64> = None;
                let mut seen_entry_count = false;
                let mut f_eviction_count: Option<i64> = None;
                let mut seen_eviction_count = false;
                let mut f_hit_count: Option<i64> = None;
                let mut seen_hit_count = false;
                let mut f_memory_usage_in_bytes: Option<i64> = None;
                let mut seen_memory_usage_in_bytes = false;
                let mut f_miss_count: Option<i64> = None;
                let mut seen_miss_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "entry_count" => {
                            if seen_entry_count {
                                return Err(serde::de::Error::duplicate_field("entry_count"));
                            }
                            seen_entry_count = true;
                            f_entry_count = map.next_value::<Option<i64>>()?;
                        }
                        "eviction_count" => {
                            if seen_eviction_count {
                                return Err(serde::de::Error::duplicate_field("eviction_count"));
                            }
                            seen_eviction_count = true;
                            f_eviction_count = map.next_value::<Option<i64>>()?;
                        }
                        "hit_count" => {
                            if seen_hit_count {
                                return Err(serde::de::Error::duplicate_field("hit_count"));
                            }
                            seen_hit_count = true;
                            f_hit_count = map.next_value::<Option<i64>>()?;
                        }
                        "memory_usage_in_bytes" => {
                            if seen_memory_usage_in_bytes {
                                return Err(serde::de::Error::duplicate_field(
                                    "memory_usage_in_bytes",
                                ));
                            }
                            seen_memory_usage_in_bytes = true;
                            f_memory_usage_in_bytes = map.next_value::<Option<i64>>()?;
                        }
                        "miss_count" => {
                            if seen_miss_count {
                                return Err(serde::de::Error::duplicate_field("miss_count"));
                            }
                            seen_miss_count = true;
                            f_miss_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CacheStat {
                    entry_count: f_entry_count,
                    eviction_count: f_eviction_count,
                    hit_count: f_hit_count,
                    memory_usage_in_bytes: f_memory_usage_in_bytes,
                    miss_count: f_miss_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheStat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.entry_count {
            map.serialize_entry("entry_count", v)?;
        }
        if let Some(v) = &self.eviction_count {
            map.serialize_entry("eviction_count", v)?;
        }
        if let Some(v) = &self.hit_count {
            map.serialize_entry("hit_count", v)?;
        }
        if let Some(v) = &self.memory_usage_in_bytes {
            map.serialize_entry("memory_usage_in_bytes", v)?;
        }
        if let Some(v) = &self.miss_count {
            map.serialize_entry("miss_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub feature: Option<crate::types::ltr_common::CacheStat>,
    pub featureset: Option<crate::types::ltr_common::CacheStat>,
    pub model: Option<crate::types::ltr_common::CacheStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_feature: Option<crate::types::ltr_common::CacheStat> = None;
                let mut seen_feature = false;
                let mut f_featureset: Option<crate::types::ltr_common::CacheStat> = None;
                let mut seen_featureset = false;
                let mut f_model: Option<crate::types::ltr_common::CacheStat> = None;
                let mut seen_model = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "feature" => {
                            if seen_feature {
                                return Err(serde::de::Error::duplicate_field("feature"));
                            }
                            seen_feature = true;
                            f_feature =
                                map.next_value::<Option<crate::types::ltr_common::CacheStat>>()?;
                        }
                        "featureset" => {
                            if seen_featureset {
                                return Err(serde::de::Error::duplicate_field("featureset"));
                            }
                            seen_featureset = true;
                            f_featureset =
                                map.next_value::<Option<crate::types::ltr_common::CacheStat>>()?;
                        }
                        "model" => {
                            if seen_model {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            seen_model = true;
                            f_model =
                                map.next_value::<Option<crate::types::ltr_common::CacheStat>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CacheStats {
                    feature: f_feature,
                    featureset: f_featureset,
                    model: f_model,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.feature {
            map.serialize_entry("feature", v)?;
        }
        if let Some(v) = &self.featureset {
            map.serialize_entry("featureset", v)?;
        }
        if let Some(v) = &self.model {
            map.serialize_entry("model", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStatsResponse {
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub all: Option<crate::types::ltr_common::CacheAllStats>,
    pub cluster_name: Option<crate::types::common::Name>,
    #[doc = "Cache statistics per node."]
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::ltr_common::NodeDetails>>,
    #[doc = "Cache statistics by store."]
    pub stores: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CacheStatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CacheStatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CacheStatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_all: Option<crate::types::ltr_common::CacheAllStats> = None;
                let mut seen_all = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut seen_cluster_name = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::ltr_common::NodeDetails>,
                > = None;
                let mut seen__nodes = false;
                let mut f_stores: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_stores = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "all" => {
                            if seen_all {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            seen_all = true;
                            f_all = map
                                .next_value::<Option<crate::types::ltr_common::CacheAllStats>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::ltr_common::NodeDetails,
                                >,
                            >>()?;
                        }
                        "stores" => {
                            if seen_stores {
                                return Err(serde::de::Error::duplicate_field("stores"));
                            }
                            seen_stores = true;
                            f_stores = map.next_value::<Option<
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
                Ok(CacheStatsResponse {
                    nodes: f_nodes,
                    all: f_all,
                    cluster_name: f_cluster_name,
                    _nodes: f__nodes,
                    stores: f_stores,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CacheStatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.all {
            map.serialize_entry("all", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        if let Some(v) = &self.stores {
            map.serialize_entry("stores", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ListStoresResponse {
    #[doc = "Map of available feature stores."]
    pub stores: std::collections::BTreeMap<String, crate::types::ltr_common::StoreDetails>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ListStoresResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ListStoresResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ListStoresResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stores: Option<
                    std::collections::BTreeMap<String, crate::types::ltr_common::StoreDetails>,
                > = None;
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
                            f_stores = Some(map.next_value::<std::collections::BTreeMap<
                                String,
                                crate::types::ltr_common::StoreDetails,
                            >>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ListStoresResponse {
                    stores: f_stores.ok_or_else(|| serde::de::Error::missing_field("stores"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ListStoresResponse {
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

#[derive(Debug, Clone)]
pub struct NodeDetails {
    #[doc = "Node hostname."]
    pub hostname: Option<String>,
    #[doc = "Node name."]
    pub name: Option<String>,
    pub stats: Option<crate::types::ltr_common::NodeStatsDetails>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hostname: Option<String> = None;
                let mut seen_hostname = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut f_stats: Option<crate::types::ltr_common::NodeStatsDetails> = None;
                let mut seen_stats = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hostname" => {
                            if seen_hostname {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            seen_hostname = true;
                            f_hostname = map.next_value::<Option<String>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        "stats" => {
                            if seen_stats {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            seen_stats = true;
                            f_stats = map
                                .next_value::<Option<crate::types::ltr_common::NodeStatsDetails>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeDetails {
                    hostname: f_hostname,
                    name: f_name,
                    stats: f_stats,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hostname {
            map.serialize_entry("hostname", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        if let Some(v) = &self.stats {
            map.serialize_entry("stats", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NodeStats {
    pub cache: Option<crate::types::ltr_common::CacheStats>,
    pub request_error_count: Option<i64>,
    pub request_total_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cache: Option<crate::types::ltr_common::CacheStats> = None;
                let mut seen_cache = false;
                let mut f_request_error_count: Option<i64> = None;
                let mut seen_request_error_count = false;
                let mut f_request_total_count: Option<i64> = None;
                let mut seen_request_total_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cache" => {
                            if seen_cache {
                                return Err(serde::de::Error::duplicate_field("cache"));
                            }
                            seen_cache = true;
                            f_cache =
                                map.next_value::<Option<crate::types::ltr_common::CacheStats>>()?;
                        }
                        "request_error_count" => {
                            if seen_request_error_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_error_count",
                                ));
                            }
                            seen_request_error_count = true;
                            f_request_error_count = map.next_value::<Option<i64>>()?;
                        }
                        "request_total_count" => {
                            if seen_request_total_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "request_total_count",
                                ));
                            }
                            seen_request_total_count = true;
                            f_request_total_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeStats {
                    cache: f_cache,
                    request_error_count: f_request_error_count,
                    request_total_count: f_request_total_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cache {
            map.serialize_entry("cache", v)?;
        }
        if let Some(v) = &self.request_error_count {
            map.serialize_entry("request_error_count", v)?;
        }
        if let Some(v) = &self.request_total_count {
            map.serialize_entry("request_total_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Cache statistics for a node."]
#[derive(Debug, Clone)]
pub struct NodeStatsDetails {
    pub features: Option<crate::types::ltr_common::CacheItemStats>,
    pub featuresets: Option<crate::types::ltr_common::CacheItemStats>,
    pub models: Option<crate::types::ltr_common::CacheItemStats>,
    pub total: Option<crate::types::ltr_common::CacheItemStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NodeStatsDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NodeStatsDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NodeStatsDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_features: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_features = false;
                let mut f_featuresets: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_featuresets = false;
                let mut f_models: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_models = false;
                let mut f_total: Option<crate::types::ltr_common::CacheItemStats> = None;
                let mut seen_total = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "features" => {
                            if seen_features {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            seen_features = true;
                            f_features = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "featuresets" => {
                            if seen_featuresets {
                                return Err(serde::de::Error::duplicate_field("featuresets"));
                            }
                            seen_featuresets = true;
                            f_featuresets = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "models" => {
                            if seen_models {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            seen_models = true;
                            f_models = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        "total" => {
                            if seen_total {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            seen_total = true;
                            f_total = map
                                .next_value::<Option<crate::types::ltr_common::CacheItemStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NodeStatsDetails {
                    features: f_features,
                    featuresets: f_featuresets,
                    models: f_models,
                    total: f_total,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NodeStatsDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.features {
            map.serialize_entry("features", v)?;
        }
        if let Some(v) = &self.featuresets {
            map.serialize_entry("featuresets", v)?;
        }
        if let Some(v) = &self.models {
            map.serialize_entry("models", v)?;
        }
        if let Some(v) = &self.total {
            map.serialize_entry("total", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: Option<crate::types::common::Name>,
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::ltr_common::NodeStats>>,
    pub status: Option<String>,
    pub stores: Option<crate::types::ltr_common::StoreStats>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Stats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Stats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Stats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut seen_cluster_name = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<String, crate::types::ltr_common::NodeStats>,
                > = None;
                let mut seen__nodes = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut f_stores: Option<crate::types::ltr_common::StoreStats> = None;
                let mut seen_stores = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::ltr_common::NodeStats,
                                >,
                            >>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        "stores" => {
                            if seen_stores {
                                return Err(serde::de::Error::duplicate_field("stores"));
                            }
                            seen_stores = true;
                            f_stores =
                                map.next_value::<Option<crate::types::ltr_common::StoreStats>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Stats {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name,
                    _nodes: f__nodes,
                    status: f_status,
                    stores: f_stores,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        if let Some(v) = &self.stores {
            map.serialize_entry("stores", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StoreDetails {
    #[doc = "Count statistics for this store."]
    pub counts: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The index name for this store."]
    pub index: Option<String>,
    #[doc = "The name of the store."]
    pub store: Option<String>,
    #[doc = "Store version."]
    pub version: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoreDetails {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoreDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoreDetails object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_counts: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_counts = false;
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_store: Option<String> = None;
                let mut seen_store = false;
                let mut f_version: Option<i64> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "counts" => {
                            if seen_counts {
                                return Err(serde::de::Error::duplicate_field("counts"));
                            }
                            seen_counts = true;
                            f_counts = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<String>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StoreDetails {
                    counts: f_counts,
                    index: f_index,
                    store: f_store,
                    version: f_version,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StoreDetails {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.counts {
            map.serialize_entry("counts", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
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

#[derive(Debug, Clone)]
pub struct StoreExistsResponse {
    #[doc = "Indicates whether the store exists."]
    pub exists: Option<bool>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoreExistsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoreExistsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoreExistsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_exists: Option<bool> = None;
                let mut seen_exists = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "exists" => {
                            if seen_exists {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            seen_exists = true;
                            f_exists = map.next_value::<Option<bool>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StoreExistsResponse {
                    exists: f_exists,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StoreExistsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.exists {
            map.serialize_entry("exists", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StoreStat {
    pub feature_count: Option<i64>,
    pub featureset_count: Option<i64>,
    pub model_count: Option<i64>,
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StoreStat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StoreStat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StoreStat object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_feature_count: Option<i64> = None;
                let mut seen_feature_count = false;
                let mut f_featureset_count: Option<i64> = None;
                let mut seen_featureset_count = false;
                let mut f_model_count: Option<i64> = None;
                let mut seen_model_count = false;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "feature_count" => {
                            if seen_feature_count {
                                return Err(serde::de::Error::duplicate_field("feature_count"));
                            }
                            seen_feature_count = true;
                            f_feature_count = map.next_value::<Option<i64>>()?;
                        }
                        "featureset_count" => {
                            if seen_featureset_count {
                                return Err(serde::de::Error::duplicate_field("featureset_count"));
                            }
                            seen_featureset_count = true;
                            f_featureset_count = map.next_value::<Option<i64>>()?;
                        }
                        "model_count" => {
                            if seen_model_count {
                                return Err(serde::de::Error::duplicate_field("model_count"));
                            }
                            seen_model_count = true;
                            f_model_count = map.next_value::<Option<i64>>()?;
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StoreStat {
                    feature_count: f_feature_count,
                    featureset_count: f_featureset_count,
                    model_count: f_model_count,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StoreStat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.feature_count {
            map.serialize_entry("feature_count", v)?;
        }
        if let Some(v) = &self.featureset_count {
            map.serialize_entry("featureset_count", v)?;
        }
        if let Some(v) = &self.model_count {
            map.serialize_entry("model_count", v)?;
        }
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type StoreStats = std::collections::BTreeMap<String, crate::types::ltr_common::StoreStat>;
