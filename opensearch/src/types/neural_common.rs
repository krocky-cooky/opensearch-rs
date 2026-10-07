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

//! Response types of the `neural_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct FlatNeuralInfoStats {
    pub cluster_version: Option<crate::types::neural_common::InfoStringStat>,
    pub processors_ingest_skip_existing_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_sparse_encoding_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_chunking_delimiter_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_chunking_fixed_char_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_chunking_fixed_token_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_chunking_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_embedding_processors_in_pipelines:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_ingest_text_image_embedding_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_comb_arithmetic_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_comb_geometric_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_comb_harmonic_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_comb_rrf_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_norm_l_2_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_norm_minmax_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_norm_zscore_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_normalization_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_hybrid_rank_based_normalization_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_neural_query_enricher_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_neural_sparse_two_phase_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_rerank_by_field_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub processors_search_rerank_ml_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlatNeuralInfoStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlatNeuralInfoStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlatNeuralInfoStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cluster_version: Option<crate::types::neural_common::InfoStringStat> =
                    None;
                let mut seen_cluster_version = false;
                let mut f_processors_ingest_skip_existing_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_skip_existing_processors = false;
                let mut f_processors_ingest_sparse_encoding_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_sparse_encoding_processors = false;
                let mut f_processors_ingest_text_chunking_delimiter_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_delimiter_processors = false;
                let mut f_processors_ingest_text_chunking_fixed_char_length_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_fixed_char_length_processors = false;
                let mut f_processors_ingest_text_chunking_fixed_token_length_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_fixed_token_length_processors = false;
                let mut f_processors_ingest_text_chunking_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_processors = false;
                let mut f_processors_ingest_text_embedding_processors_in_pipelines: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_embedding_processors_in_pipelines = false;
                let mut f_processors_ingest_text_image_embedding_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_ingest_text_image_embedding_processors = false;
                let mut f_processors_search_hybrid_comb_arithmetic_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_arithmetic_processors = false;
                let mut f_processors_search_hybrid_comb_geometric_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_geometric_processors = false;
                let mut f_processors_search_hybrid_comb_harmonic_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_harmonic_processors = false;
                let mut f_processors_search_hybrid_comb_rrf_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_rrf_processors = false;
                let mut f_processors_search_hybrid_norm_l_2_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_l_2_processors = false;
                let mut f_processors_search_hybrid_norm_minmax_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_minmax_processors = false;
                let mut f_processors_search_hybrid_norm_zscore_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_zscore_processors = false;
                let mut f_processors_search_hybrid_normalization_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_normalization_processors = false;
                let mut f_processors_search_hybrid_rank_based_normalization_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_rank_based_normalization_processors = false;
                let mut f_processors_search_neural_query_enricher_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_neural_query_enricher_processors = false;
                let mut f_processors_search_neural_sparse_two_phase_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_neural_sparse_two_phase_processors = false;
                let mut f_processors_search_rerank_by_field_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_rerank_by_field_processors = false;
                let mut f_processors_search_rerank_ml_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_processors_search_rerank_ml_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_version" => {
                            if seen_cluster_version {
                                return Err(serde::de::Error::duplicate_field("cluster_version"));
                            }
                            seen_cluster_version = true;
                            f_cluster_version = map
                                .next_value::<Option<crate::types::neural_common::InfoStringStat>>(
                                )?;
                        }
                        "processors.ingest.skip_existing_processors" => {
                            if seen_processors_ingest_skip_existing_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.skip_existing_processors",
                                ));
                            }
                            seen_processors_ingest_skip_existing_processors = true;
                            f_processors_ingest_skip_existing_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.ingest.sparse_encoding_processors" => {
                            if seen_processors_ingest_sparse_encoding_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.sparse_encoding_processors",
                                ));
                            }
                            seen_processors_ingest_sparse_encoding_processors = true;
                            f_processors_ingest_sparse_encoding_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.ingest.text_chunking_delimiter_processors" => {
                            if seen_processors_ingest_text_chunking_delimiter_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_chunking_delimiter_processors",
                                ));
                            }
                            seen_processors_ingest_text_chunking_delimiter_processors = true;
                            f_processors_ingest_text_chunking_delimiter_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.ingest.text_chunking_fixed_char_length_processors" => {
                            if seen_processors_ingest_text_chunking_fixed_char_length_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_chunking_fixed_char_length_processors",
                                ));
                            }
                            seen_processors_ingest_text_chunking_fixed_char_length_processors =
                                true;
                            f_processors_ingest_text_chunking_fixed_char_length_processors = map.next_value::<Option<crate::types::neural_common::InfoCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_fixed_token_length_processors" => {
                            if seen_processors_ingest_text_chunking_fixed_token_length_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_chunking_fixed_token_length_processors",
                                ));
                            }
                            seen_processors_ingest_text_chunking_fixed_token_length_processors =
                                true;
                            f_processors_ingest_text_chunking_fixed_token_length_processors = map.next_value::<Option<crate::types::neural_common::InfoCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_processors" => {
                            if seen_processors_ingest_text_chunking_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_chunking_processors",
                                ));
                            }
                            seen_processors_ingest_text_chunking_processors = true;
                            f_processors_ingest_text_chunking_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.ingest.text_embedding_processors_in_pipelines" => {
                            if seen_processors_ingest_text_embedding_processors_in_pipelines {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_embedding_processors_in_pipelines",
                                ));
                            }
                            seen_processors_ingest_text_embedding_processors_in_pipelines = true;
                            f_processors_ingest_text_embedding_processors_in_pipelines = map
                                .next_value::<Option<
                                crate::types::neural_common::InfoCounterStat,
                            >>(
                            )?;
                        }
                        "processors.ingest.text_image_embedding_processors" => {
                            if seen_processors_ingest_text_image_embedding_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.ingest.text_image_embedding_processors",
                                ));
                            }
                            seen_processors_ingest_text_image_embedding_processors = true;
                            f_processors_ingest_text_image_embedding_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.comb_arithmetic_processors" => {
                            if seen_processors_search_hybrid_comb_arithmetic_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.comb_arithmetic_processors",
                                ));
                            }
                            seen_processors_search_hybrid_comb_arithmetic_processors = true;
                            f_processors_search_hybrid_comb_arithmetic_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.comb_geometric_processors" => {
                            if seen_processors_search_hybrid_comb_geometric_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.comb_geometric_processors",
                                ));
                            }
                            seen_processors_search_hybrid_comb_geometric_processors = true;
                            f_processors_search_hybrid_comb_geometric_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.comb_harmonic_processors" => {
                            if seen_processors_search_hybrid_comb_harmonic_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.comb_harmonic_processors",
                                ));
                            }
                            seen_processors_search_hybrid_comb_harmonic_processors = true;
                            f_processors_search_hybrid_comb_harmonic_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.comb_rrf_processors" => {
                            if seen_processors_search_hybrid_comb_rrf_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.comb_rrf_processors",
                                ));
                            }
                            seen_processors_search_hybrid_comb_rrf_processors = true;
                            f_processors_search_hybrid_comb_rrf_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.norm_l2_processors" => {
                            if seen_processors_search_hybrid_norm_l_2_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.norm_l2_processors",
                                ));
                            }
                            seen_processors_search_hybrid_norm_l_2_processors = true;
                            f_processors_search_hybrid_norm_l_2_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.norm_minmax_processors" => {
                            if seen_processors_search_hybrid_norm_minmax_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.norm_minmax_processors",
                                ));
                            }
                            seen_processors_search_hybrid_norm_minmax_processors = true;
                            f_processors_search_hybrid_norm_minmax_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.norm_zscore_processors" => {
                            if seen_processors_search_hybrid_norm_zscore_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.norm_zscore_processors",
                                ));
                            }
                            seen_processors_search_hybrid_norm_zscore_processors = true;
                            f_processors_search_hybrid_norm_zscore_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.normalization_processors" => {
                            if seen_processors_search_hybrid_normalization_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.normalization_processors",
                                ));
                            }
                            seen_processors_search_hybrid_normalization_processors = true;
                            f_processors_search_hybrid_normalization_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.hybrid.rank_based_normalization_processors" => {
                            if seen_processors_search_hybrid_rank_based_normalization_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.hybrid.rank_based_normalization_processors",
                                ));
                            }
                            seen_processors_search_hybrid_rank_based_normalization_processors =
                                true;
                            f_processors_search_hybrid_rank_based_normalization_processors = map.next_value::<Option<crate::types::neural_common::InfoCounterStat>>()?;
                        }
                        "processors.search.neural_query_enricher_processors" => {
                            if seen_processors_search_neural_query_enricher_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.neural_query_enricher_processors",
                                ));
                            }
                            seen_processors_search_neural_query_enricher_processors = true;
                            f_processors_search_neural_query_enricher_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.neural_sparse_two_phase_processors" => {
                            if seen_processors_search_neural_sparse_two_phase_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.neural_sparse_two_phase_processors",
                                ));
                            }
                            seen_processors_search_neural_sparse_two_phase_processors = true;
                            f_processors_search_neural_sparse_two_phase_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.rerank_by_field_processors" => {
                            if seen_processors_search_rerank_by_field_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.rerank_by_field_processors",
                                ));
                            }
                            seen_processors_search_rerank_by_field_processors = true;
                            f_processors_search_rerank_by_field_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "processors.search.rerank_ml_processors" => {
                            if seen_processors_search_rerank_ml_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "processors.search.rerank_ml_processors",
                                ));
                            }
                            seen_processors_search_rerank_ml_processors = true;
                            f_processors_search_rerank_ml_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FlatNeuralInfoStats {
                    cluster_version: f_cluster_version,
                    processors_ingest_skip_existing_processors:
                        f_processors_ingest_skip_existing_processors,
                    processors_ingest_sparse_encoding_processors:
                        f_processors_ingest_sparse_encoding_processors,
                    processors_ingest_text_chunking_delimiter_processors:
                        f_processors_ingest_text_chunking_delimiter_processors,
                    processors_ingest_text_chunking_fixed_char_length_processors:
                        f_processors_ingest_text_chunking_fixed_char_length_processors,
                    processors_ingest_text_chunking_fixed_token_length_processors:
                        f_processors_ingest_text_chunking_fixed_token_length_processors,
                    processors_ingest_text_chunking_processors:
                        f_processors_ingest_text_chunking_processors,
                    processors_ingest_text_embedding_processors_in_pipelines:
                        f_processors_ingest_text_embedding_processors_in_pipelines,
                    processors_ingest_text_image_embedding_processors:
                        f_processors_ingest_text_image_embedding_processors,
                    processors_search_hybrid_comb_arithmetic_processors:
                        f_processors_search_hybrid_comb_arithmetic_processors,
                    processors_search_hybrid_comb_geometric_processors:
                        f_processors_search_hybrid_comb_geometric_processors,
                    processors_search_hybrid_comb_harmonic_processors:
                        f_processors_search_hybrid_comb_harmonic_processors,
                    processors_search_hybrid_comb_rrf_processors:
                        f_processors_search_hybrid_comb_rrf_processors,
                    processors_search_hybrid_norm_l_2_processors:
                        f_processors_search_hybrid_norm_l_2_processors,
                    processors_search_hybrid_norm_minmax_processors:
                        f_processors_search_hybrid_norm_minmax_processors,
                    processors_search_hybrid_norm_zscore_processors:
                        f_processors_search_hybrid_norm_zscore_processors,
                    processors_search_hybrid_normalization_processors:
                        f_processors_search_hybrid_normalization_processors,
                    processors_search_hybrid_rank_based_normalization_processors:
                        f_processors_search_hybrid_rank_based_normalization_processors,
                    processors_search_neural_query_enricher_processors:
                        f_processors_search_neural_query_enricher_processors,
                    processors_search_neural_sparse_two_phase_processors:
                        f_processors_search_neural_sparse_two_phase_processors,
                    processors_search_rerank_by_field_processors:
                        f_processors_search_rerank_by_field_processors,
                    processors_search_rerank_ml_processors:
                        f_processors_search_rerank_ml_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlatNeuralInfoStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cluster_version {
            map.serialize_entry("cluster_version", v)?;
        }
        if let Some(v) = &self.processors_ingest_skip_existing_processors {
            map.serialize_entry("processors.ingest.skip_existing_processors", v)?;
        }
        if let Some(v) = &self.processors_ingest_sparse_encoding_processors {
            map.serialize_entry("processors.ingest.sparse_encoding_processors", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_delimiter_processors {
            map.serialize_entry("processors.ingest.text_chunking_delimiter_processors", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_fixed_char_length_processors {
            map.serialize_entry(
                "processors.ingest.text_chunking_fixed_char_length_processors",
                v,
            )?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_fixed_token_length_processors {
            map.serialize_entry(
                "processors.ingest.text_chunking_fixed_token_length_processors",
                v,
            )?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_processors {
            map.serialize_entry("processors.ingest.text_chunking_processors", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_embedding_processors_in_pipelines {
            map.serialize_entry(
                "processors.ingest.text_embedding_processors_in_pipelines",
                v,
            )?;
        }
        if let Some(v) = &self.processors_ingest_text_image_embedding_processors {
            map.serialize_entry("processors.ingest.text_image_embedding_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_arithmetic_processors {
            map.serialize_entry("processors.search.hybrid.comb_arithmetic_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_geometric_processors {
            map.serialize_entry("processors.search.hybrid.comb_geometric_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_harmonic_processors {
            map.serialize_entry("processors.search.hybrid.comb_harmonic_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_rrf_processors {
            map.serialize_entry("processors.search.hybrid.comb_rrf_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_l_2_processors {
            map.serialize_entry("processors.search.hybrid.norm_l2_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_minmax_processors {
            map.serialize_entry("processors.search.hybrid.norm_minmax_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_zscore_processors {
            map.serialize_entry("processors.search.hybrid.norm_zscore_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_normalization_processors {
            map.serialize_entry("processors.search.hybrid.normalization_processors", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_rank_based_normalization_processors {
            map.serialize_entry(
                "processors.search.hybrid.rank_based_normalization_processors",
                v,
            )?;
        }
        if let Some(v) = &self.processors_search_neural_query_enricher_processors {
            map.serialize_entry("processors.search.neural_query_enricher_processors", v)?;
        }
        if let Some(v) = &self.processors_search_neural_sparse_two_phase_processors {
            map.serialize_entry("processors.search.neural_sparse_two_phase_processors", v)?;
        }
        if let Some(v) = &self.processors_search_rerank_by_field_processors {
            map.serialize_entry("processors.search.rerank_by_field_processors", v)?;
        }
        if let Some(v) = &self.processors_search_rerank_ml_processors {
            map.serialize_entry("processors.search.rerank_ml_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FlatNeuralNodeStats {
    pub processors_ingest_semantic_field_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_semantic_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_skip_existing_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_sparse_encoding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_chunking_delimiter_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_chunking_fixed_char_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_chunking_fixed_token_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_ingest_text_image_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_comb_arithmetic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_comb_geometric_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_comb_harmonic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_comb_rrf_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_norm_l_2_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_norm_minmax_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_norm_zscore_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_hybrid_rank_based_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_neural_query_enricher_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_neural_sparse_two_phase_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_rerank_by_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub processors_search_rerank_ml_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_hybrid_hybrid_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_hybrid_hybrid_query_with_filter_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_hybrid_hybrid_query_with_inner_hits_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_hybrid_hybrid_query_with_pagination_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_neural_neural_query_against_knn_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_neural_neural_query_against_semantic_dense_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_neural_neural_query_against_semantic_sparse_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_neural_neural_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub query_neural_sparse_neural_sparse_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub semantic_highlighting_semantic_highlighting_request_count:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlatNeuralNodeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlatNeuralNodeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlatNeuralNodeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_processors_ingest_semantic_field_chunking_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_semantic_field_chunking_executions = false;
                let mut f_processors_ingest_semantic_field_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_semantic_field_executions = false;
                let mut f_processors_ingest_skip_existing_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_skip_existing_executions = false;
                let mut f_processors_ingest_sparse_encoding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_sparse_encoding_executions = false;
                let mut f_processors_ingest_text_chunking_delimiter_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_delimiter_executions = false;
                let mut f_processors_ingest_text_chunking_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_executions = false;
                let mut f_processors_ingest_text_chunking_fixed_char_length_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_fixed_char_length_executions = false;
                let mut f_processors_ingest_text_chunking_fixed_token_length_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_chunking_fixed_token_length_executions = false;
                let mut f_processors_ingest_text_embedding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_embedding_executions = false;
                let mut f_processors_ingest_text_image_embedding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_ingest_text_image_embedding_executions = false;
                let mut f_processors_search_hybrid_comb_arithmetic_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_arithmetic_executions = false;
                let mut f_processors_search_hybrid_comb_geometric_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_geometric_executions = false;
                let mut f_processors_search_hybrid_comb_harmonic_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_harmonic_executions = false;
                let mut f_processors_search_hybrid_comb_rrf_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_comb_rrf_executions = false;
                let mut f_processors_search_hybrid_norm_l_2_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_l_2_executions = false;
                let mut f_processors_search_hybrid_norm_minmax_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_minmax_executions = false;
                let mut f_processors_search_hybrid_norm_zscore_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_norm_zscore_executions = false;
                let mut f_processors_search_hybrid_normalization_processor_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_hybrid_normalization_processor_executions = false;
                let mut f_processors_search_hybrid_rank_based_normalization_processor_executions: Option<crate::types::neural_common::TimestampedEventCounterStat> = None;
                let mut seen_processors_search_hybrid_rank_based_normalization_processor_executions =
                    false;
                let mut f_processors_search_neural_query_enricher_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_neural_query_enricher_executions = false;
                let mut f_processors_search_neural_sparse_two_phase_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_neural_sparse_two_phase_executions = false;
                let mut f_processors_search_rerank_by_field_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_rerank_by_field_executions = false;
                let mut f_processors_search_rerank_ml_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_processors_search_rerank_ml_executions = false;
                let mut f_query_hybrid_hybrid_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_hybrid_hybrid_query_requests = false;
                let mut f_query_hybrid_hybrid_query_with_filter_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_hybrid_hybrid_query_with_filter_requests = false;
                let mut f_query_hybrid_hybrid_query_with_inner_hits_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_hybrid_hybrid_query_with_inner_hits_requests = false;
                let mut f_query_hybrid_hybrid_query_with_pagination_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_hybrid_hybrid_query_with_pagination_requests = false;
                let mut f_query_neural_neural_query_against_knn_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_neural_neural_query_against_knn_requests = false;
                let mut f_query_neural_neural_query_against_semantic_dense_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_neural_neural_query_against_semantic_dense_requests = false;
                let mut f_query_neural_neural_query_against_semantic_sparse_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_neural_neural_query_against_semantic_sparse_requests = false;
                let mut f_query_neural_neural_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_neural_neural_query_requests = false;
                let mut f_query_neural_sparse_neural_sparse_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_query_neural_sparse_neural_sparse_query_requests = false;
                let mut f_semantic_highlighting_semantic_highlighting_request_count: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_semantic_highlighting_semantic_highlighting_request_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "processors.ingest.semantic_field_chunking_executions" => {
                            if seen_processors_ingest_semantic_field_chunking_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.semantic_field_chunking_executions"));
                            }
                            seen_processors_ingest_semantic_field_chunking_executions = true;
                            f_processors_ingest_semantic_field_chunking_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.semantic_field_executions" => {
                            if seen_processors_ingest_semantic_field_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.semantic_field_executions"));
                            }
                            seen_processors_ingest_semantic_field_executions = true;
                            f_processors_ingest_semantic_field_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.skip_existing_executions" => {
                            if seen_processors_ingest_skip_existing_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.skip_existing_executions"));
                            }
                            seen_processors_ingest_skip_existing_executions = true;
                            f_processors_ingest_skip_existing_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.sparse_encoding_executions" => {
                            if seen_processors_ingest_sparse_encoding_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.sparse_encoding_executions"));
                            }
                            seen_processors_ingest_sparse_encoding_executions = true;
                            f_processors_ingest_sparse_encoding_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_delimiter_executions" => {
                            if seen_processors_ingest_text_chunking_delimiter_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_chunking_delimiter_executions"));
                            }
                            seen_processors_ingest_text_chunking_delimiter_executions = true;
                            f_processors_ingest_text_chunking_delimiter_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_executions" => {
                            if seen_processors_ingest_text_chunking_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_chunking_executions"));
                            }
                            seen_processors_ingest_text_chunking_executions = true;
                            f_processors_ingest_text_chunking_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_fixed_char_length_executions" => {
                            if seen_processors_ingest_text_chunking_fixed_char_length_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_chunking_fixed_char_length_executions"));
                            }
                            seen_processors_ingest_text_chunking_fixed_char_length_executions = true;
                            f_processors_ingest_text_chunking_fixed_char_length_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_chunking_fixed_token_length_executions" => {
                            if seen_processors_ingest_text_chunking_fixed_token_length_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_chunking_fixed_token_length_executions"));
                            }
                            seen_processors_ingest_text_chunking_fixed_token_length_executions = true;
                            f_processors_ingest_text_chunking_fixed_token_length_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_embedding_executions" => {
                            if seen_processors_ingest_text_embedding_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_embedding_executions"));
                            }
                            seen_processors_ingest_text_embedding_executions = true;
                            f_processors_ingest_text_embedding_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.ingest.text_image_embedding_executions" => {
                            if seen_processors_ingest_text_image_embedding_executions {
                                return Err(serde::de::Error::duplicate_field("processors.ingest.text_image_embedding_executions"));
                            }
                            seen_processors_ingest_text_image_embedding_executions = true;
                            f_processors_ingest_text_image_embedding_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.comb_arithmetic_executions" => {
                            if seen_processors_search_hybrid_comb_arithmetic_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.comb_arithmetic_executions"));
                            }
                            seen_processors_search_hybrid_comb_arithmetic_executions = true;
                            f_processors_search_hybrid_comb_arithmetic_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.comb_geometric_executions" => {
                            if seen_processors_search_hybrid_comb_geometric_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.comb_geometric_executions"));
                            }
                            seen_processors_search_hybrid_comb_geometric_executions = true;
                            f_processors_search_hybrid_comb_geometric_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.comb_harmonic_executions" => {
                            if seen_processors_search_hybrid_comb_harmonic_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.comb_harmonic_executions"));
                            }
                            seen_processors_search_hybrid_comb_harmonic_executions = true;
                            f_processors_search_hybrid_comb_harmonic_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.comb_rrf_executions" => {
                            if seen_processors_search_hybrid_comb_rrf_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.comb_rrf_executions"));
                            }
                            seen_processors_search_hybrid_comb_rrf_executions = true;
                            f_processors_search_hybrid_comb_rrf_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.norm_l2_executions" => {
                            if seen_processors_search_hybrid_norm_l_2_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.norm_l2_executions"));
                            }
                            seen_processors_search_hybrid_norm_l_2_executions = true;
                            f_processors_search_hybrid_norm_l_2_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.norm_minmax_executions" => {
                            if seen_processors_search_hybrid_norm_minmax_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.norm_minmax_executions"));
                            }
                            seen_processors_search_hybrid_norm_minmax_executions = true;
                            f_processors_search_hybrid_norm_minmax_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.norm_zscore_executions" => {
                            if seen_processors_search_hybrid_norm_zscore_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.norm_zscore_executions"));
                            }
                            seen_processors_search_hybrid_norm_zscore_executions = true;
                            f_processors_search_hybrid_norm_zscore_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.normalization_processor_executions" => {
                            if seen_processors_search_hybrid_normalization_processor_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.normalization_processor_executions"));
                            }
                            seen_processors_search_hybrid_normalization_processor_executions = true;
                            f_processors_search_hybrid_normalization_processor_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.hybrid.rank_based_normalization_processor_executions" => {
                            if seen_processors_search_hybrid_rank_based_normalization_processor_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.hybrid.rank_based_normalization_processor_executions"));
                            }
                            seen_processors_search_hybrid_rank_based_normalization_processor_executions = true;
                            f_processors_search_hybrid_rank_based_normalization_processor_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.neural_query_enricher_executions" => {
                            if seen_processors_search_neural_query_enricher_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.neural_query_enricher_executions"));
                            }
                            seen_processors_search_neural_query_enricher_executions = true;
                            f_processors_search_neural_query_enricher_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.neural_sparse_two_phase_executions" => {
                            if seen_processors_search_neural_sparse_two_phase_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.neural_sparse_two_phase_executions"));
                            }
                            seen_processors_search_neural_sparse_two_phase_executions = true;
                            f_processors_search_neural_sparse_two_phase_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.rerank_by_field_executions" => {
                            if seen_processors_search_rerank_by_field_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.rerank_by_field_executions"));
                            }
                            seen_processors_search_rerank_by_field_executions = true;
                            f_processors_search_rerank_by_field_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "processors.search.rerank_ml_executions" => {
                            if seen_processors_search_rerank_ml_executions {
                                return Err(serde::de::Error::duplicate_field("processors.search.rerank_ml_executions"));
                            }
                            seen_processors_search_rerank_ml_executions = true;
                            f_processors_search_rerank_ml_executions = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.hybrid.hybrid_query_requests" => {
                            if seen_query_hybrid_hybrid_query_requests {
                                return Err(serde::de::Error::duplicate_field("query.hybrid.hybrid_query_requests"));
                            }
                            seen_query_hybrid_hybrid_query_requests = true;
                            f_query_hybrid_hybrid_query_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.hybrid.hybrid_query_with_filter_requests" => {
                            if seen_query_hybrid_hybrid_query_with_filter_requests {
                                return Err(serde::de::Error::duplicate_field("query.hybrid.hybrid_query_with_filter_requests"));
                            }
                            seen_query_hybrid_hybrid_query_with_filter_requests = true;
                            f_query_hybrid_hybrid_query_with_filter_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.hybrid.hybrid_query_with_inner_hits_requests" => {
                            if seen_query_hybrid_hybrid_query_with_inner_hits_requests {
                                return Err(serde::de::Error::duplicate_field("query.hybrid.hybrid_query_with_inner_hits_requests"));
                            }
                            seen_query_hybrid_hybrid_query_with_inner_hits_requests = true;
                            f_query_hybrid_hybrid_query_with_inner_hits_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.hybrid.hybrid_query_with_pagination_requests" => {
                            if seen_query_hybrid_hybrid_query_with_pagination_requests {
                                return Err(serde::de::Error::duplicate_field("query.hybrid.hybrid_query_with_pagination_requests"));
                            }
                            seen_query_hybrid_hybrid_query_with_pagination_requests = true;
                            f_query_hybrid_hybrid_query_with_pagination_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.neural.neural_query_against_knn_requests" => {
                            if seen_query_neural_neural_query_against_knn_requests {
                                return Err(serde::de::Error::duplicate_field("query.neural.neural_query_against_knn_requests"));
                            }
                            seen_query_neural_neural_query_against_knn_requests = true;
                            f_query_neural_neural_query_against_knn_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.neural.neural_query_against_semantic_dense_requests" => {
                            if seen_query_neural_neural_query_against_semantic_dense_requests {
                                return Err(serde::de::Error::duplicate_field("query.neural.neural_query_against_semantic_dense_requests"));
                            }
                            seen_query_neural_neural_query_against_semantic_dense_requests = true;
                            f_query_neural_neural_query_against_semantic_dense_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.neural.neural_query_against_semantic_sparse_requests" => {
                            if seen_query_neural_neural_query_against_semantic_sparse_requests {
                                return Err(serde::de::Error::duplicate_field("query.neural.neural_query_against_semantic_sparse_requests"));
                            }
                            seen_query_neural_neural_query_against_semantic_sparse_requests = true;
                            f_query_neural_neural_query_against_semantic_sparse_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.neural.neural_query_requests" => {
                            if seen_query_neural_neural_query_requests {
                                return Err(serde::de::Error::duplicate_field("query.neural.neural_query_requests"));
                            }
                            seen_query_neural_neural_query_requests = true;
                            f_query_neural_neural_query_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "query.neural_sparse.neural_sparse_query_requests" => {
                            if seen_query_neural_sparse_neural_sparse_query_requests {
                                return Err(serde::de::Error::duplicate_field("query.neural_sparse.neural_sparse_query_requests"));
                            }
                            seen_query_neural_sparse_neural_sparse_query_requests = true;
                            f_query_neural_sparse_neural_sparse_query_requests = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        "semantic_highlighting.semantic_highlighting_request_count" => {
                            if seen_semantic_highlighting_semantic_highlighting_request_count {
                                return Err(serde::de::Error::duplicate_field("semantic_highlighting.semantic_highlighting_request_count"));
                            }
                            seen_semantic_highlighting_semantic_highlighting_request_count = true;
                            f_semantic_highlighting_semantic_highlighting_request_count = map.next_value::<Option<crate::types::neural_common::TimestampedEventCounterStat>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FlatNeuralNodeStats {
                    processors_ingest_semantic_field_chunking_executions:
                        f_processors_ingest_semantic_field_chunking_executions,
                    processors_ingest_semantic_field_executions:
                        f_processors_ingest_semantic_field_executions,
                    processors_ingest_skip_existing_executions:
                        f_processors_ingest_skip_existing_executions,
                    processors_ingest_sparse_encoding_executions:
                        f_processors_ingest_sparse_encoding_executions,
                    processors_ingest_text_chunking_delimiter_executions:
                        f_processors_ingest_text_chunking_delimiter_executions,
                    processors_ingest_text_chunking_executions:
                        f_processors_ingest_text_chunking_executions,
                    processors_ingest_text_chunking_fixed_char_length_executions:
                        f_processors_ingest_text_chunking_fixed_char_length_executions,
                    processors_ingest_text_chunking_fixed_token_length_executions:
                        f_processors_ingest_text_chunking_fixed_token_length_executions,
                    processors_ingest_text_embedding_executions:
                        f_processors_ingest_text_embedding_executions,
                    processors_ingest_text_image_embedding_executions:
                        f_processors_ingest_text_image_embedding_executions,
                    processors_search_hybrid_comb_arithmetic_executions:
                        f_processors_search_hybrid_comb_arithmetic_executions,
                    processors_search_hybrid_comb_geometric_executions:
                        f_processors_search_hybrid_comb_geometric_executions,
                    processors_search_hybrid_comb_harmonic_executions:
                        f_processors_search_hybrid_comb_harmonic_executions,
                    processors_search_hybrid_comb_rrf_executions:
                        f_processors_search_hybrid_comb_rrf_executions,
                    processors_search_hybrid_norm_l_2_executions:
                        f_processors_search_hybrid_norm_l_2_executions,
                    processors_search_hybrid_norm_minmax_executions:
                        f_processors_search_hybrid_norm_minmax_executions,
                    processors_search_hybrid_norm_zscore_executions:
                        f_processors_search_hybrid_norm_zscore_executions,
                    processors_search_hybrid_normalization_processor_executions:
                        f_processors_search_hybrid_normalization_processor_executions,
                    processors_search_hybrid_rank_based_normalization_processor_executions:
                        f_processors_search_hybrid_rank_based_normalization_processor_executions,
                    processors_search_neural_query_enricher_executions:
                        f_processors_search_neural_query_enricher_executions,
                    processors_search_neural_sparse_two_phase_executions:
                        f_processors_search_neural_sparse_two_phase_executions,
                    processors_search_rerank_by_field_executions:
                        f_processors_search_rerank_by_field_executions,
                    processors_search_rerank_ml_executions:
                        f_processors_search_rerank_ml_executions,
                    query_hybrid_hybrid_query_requests: f_query_hybrid_hybrid_query_requests,
                    query_hybrid_hybrid_query_with_filter_requests:
                        f_query_hybrid_hybrid_query_with_filter_requests,
                    query_hybrid_hybrid_query_with_inner_hits_requests:
                        f_query_hybrid_hybrid_query_with_inner_hits_requests,
                    query_hybrid_hybrid_query_with_pagination_requests:
                        f_query_hybrid_hybrid_query_with_pagination_requests,
                    query_neural_neural_query_against_knn_requests:
                        f_query_neural_neural_query_against_knn_requests,
                    query_neural_neural_query_against_semantic_dense_requests:
                        f_query_neural_neural_query_against_semantic_dense_requests,
                    query_neural_neural_query_against_semantic_sparse_requests:
                        f_query_neural_neural_query_against_semantic_sparse_requests,
                    query_neural_neural_query_requests: f_query_neural_neural_query_requests,
                    query_neural_sparse_neural_sparse_query_requests:
                        f_query_neural_sparse_neural_sparse_query_requests,
                    semantic_highlighting_semantic_highlighting_request_count:
                        f_semantic_highlighting_semantic_highlighting_request_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlatNeuralNodeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.processors_ingest_semantic_field_chunking_executions {
            map.serialize_entry("processors.ingest.semantic_field_chunking_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_semantic_field_executions {
            map.serialize_entry("processors.ingest.semantic_field_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_skip_existing_executions {
            map.serialize_entry("processors.ingest.skip_existing_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_sparse_encoding_executions {
            map.serialize_entry("processors.ingest.sparse_encoding_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_delimiter_executions {
            map.serialize_entry("processors.ingest.text_chunking_delimiter_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_executions {
            map.serialize_entry("processors.ingest.text_chunking_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_fixed_char_length_executions {
            map.serialize_entry(
                "processors.ingest.text_chunking_fixed_char_length_executions",
                v,
            )?;
        }
        if let Some(v) = &self.processors_ingest_text_chunking_fixed_token_length_executions {
            map.serialize_entry(
                "processors.ingest.text_chunking_fixed_token_length_executions",
                v,
            )?;
        }
        if let Some(v) = &self.processors_ingest_text_embedding_executions {
            map.serialize_entry("processors.ingest.text_embedding_executions", v)?;
        }
        if let Some(v) = &self.processors_ingest_text_image_embedding_executions {
            map.serialize_entry("processors.ingest.text_image_embedding_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_arithmetic_executions {
            map.serialize_entry("processors.search.hybrid.comb_arithmetic_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_geometric_executions {
            map.serialize_entry("processors.search.hybrid.comb_geometric_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_harmonic_executions {
            map.serialize_entry("processors.search.hybrid.comb_harmonic_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_comb_rrf_executions {
            map.serialize_entry("processors.search.hybrid.comb_rrf_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_l_2_executions {
            map.serialize_entry("processors.search.hybrid.norm_l2_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_minmax_executions {
            map.serialize_entry("processors.search.hybrid.norm_minmax_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_norm_zscore_executions {
            map.serialize_entry("processors.search.hybrid.norm_zscore_executions", v)?;
        }
        if let Some(v) = &self.processors_search_hybrid_normalization_processor_executions {
            map.serialize_entry(
                "processors.search.hybrid.normalization_processor_executions",
                v,
            )?;
        }
        if let Some(v) =
            &self.processors_search_hybrid_rank_based_normalization_processor_executions
        {
            map.serialize_entry(
                "processors.search.hybrid.rank_based_normalization_processor_executions",
                v,
            )?;
        }
        if let Some(v) = &self.processors_search_neural_query_enricher_executions {
            map.serialize_entry("processors.search.neural_query_enricher_executions", v)?;
        }
        if let Some(v) = &self.processors_search_neural_sparse_two_phase_executions {
            map.serialize_entry("processors.search.neural_sparse_two_phase_executions", v)?;
        }
        if let Some(v) = &self.processors_search_rerank_by_field_executions {
            map.serialize_entry("processors.search.rerank_by_field_executions", v)?;
        }
        if let Some(v) = &self.processors_search_rerank_ml_executions {
            map.serialize_entry("processors.search.rerank_ml_executions", v)?;
        }
        if let Some(v) = &self.query_hybrid_hybrid_query_requests {
            map.serialize_entry("query.hybrid.hybrid_query_requests", v)?;
        }
        if let Some(v) = &self.query_hybrid_hybrid_query_with_filter_requests {
            map.serialize_entry("query.hybrid.hybrid_query_with_filter_requests", v)?;
        }
        if let Some(v) = &self.query_hybrid_hybrid_query_with_inner_hits_requests {
            map.serialize_entry("query.hybrid.hybrid_query_with_inner_hits_requests", v)?;
        }
        if let Some(v) = &self.query_hybrid_hybrid_query_with_pagination_requests {
            map.serialize_entry("query.hybrid.hybrid_query_with_pagination_requests", v)?;
        }
        if let Some(v) = &self.query_neural_neural_query_against_knn_requests {
            map.serialize_entry("query.neural.neural_query_against_knn_requests", v)?;
        }
        if let Some(v) = &self.query_neural_neural_query_against_semantic_dense_requests {
            map.serialize_entry(
                "query.neural.neural_query_against_semantic_dense_requests",
                v,
            )?;
        }
        if let Some(v) = &self.query_neural_neural_query_against_semantic_sparse_requests {
            map.serialize_entry(
                "query.neural.neural_query_against_semantic_sparse_requests",
                v,
            )?;
        }
        if let Some(v) = &self.query_neural_neural_query_requests {
            map.serialize_entry("query.neural.neural_query_requests", v)?;
        }
        if let Some(v) = &self.query_neural_sparse_neural_sparse_query_requests {
            map.serialize_entry("query.neural_sparse.neural_sparse_query_requests", v)?;
        }
        if let Some(v) = &self.semantic_highlighting_semantic_highlighting_request_count {
            map.serialize_entry(
                "semantic_highlighting.semantic_highlighting_request_count",
                v,
            )?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FlatNeuralStats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub all_nodes: Option<crate::types::neural_common::FlatNeuralNodeStats>,
    pub cluster_name: Option<String>,
    pub info: Option<crate::types::neural_common::FlatNeuralInfoStats>,
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::neural_common::FlatNeuralNodeStats>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FlatNeuralStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FlatNeuralStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FlatNeuralStats object")
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
                let mut f_all_nodes: Option<crate::types::neural_common::FlatNeuralNodeStats> =
                    None;
                let mut seen_all_nodes = false;
                let mut f_cluster_name: Option<String> = None;
                let mut seen_cluster_name = false;
                let mut f_info: Option<crate::types::neural_common::FlatNeuralInfoStats> = None;
                let mut seen_info = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::neural_common::FlatNeuralNodeStats,
                    >,
                > = None;
                let mut seen__nodes = false;
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
                        "all_nodes" => {
                            if seen_all_nodes {
                                return Err(serde::de::Error::duplicate_field("all_nodes"));
                            }
                            seen_all_nodes = true;
                            f_all_nodes = map.next_value::<Option<crate::types::neural_common::FlatNeuralNodeStats>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name = map.next_value::<Option<String>>()?;
                        }
                        "info" => {
                            if seen_info {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            seen_info = true;
                            f_info = map.next_value::<Option<crate::types::neural_common::FlatNeuralInfoStats>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::neural_common::FlatNeuralNodeStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FlatNeuralStats {
                    nodes: f_nodes,
                    all_nodes: f_all_nodes,
                    cluster_name: f_cluster_name,
                    info: f_info,
                    _nodes: f__nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FlatNeuralStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.all_nodes {
            map.serialize_entry("all_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self.info {
            map.serialize_entry("info", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoCounterStat {
    Integer(i64),
    Object(crate::types::neural_common::InfoCounterStatObject),
}
impl<'de> serde::Deserialize<'de> for InfoCounterStat {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoCounterStat::Integer)
            }
            Some(b'{') => serde_json::from_str(text).map(InfoCounterStat::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a InfoCounterStat value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct InfoCounterStatObject {
    pub stat_type: crate::types::neural_common::InfoCounterStatObjectStatType,
    pub value: crate::types::neural_common::InfoCounterStatObjectValue,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InfoCounterStatObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InfoCounterStatObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InfoCounterStatObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stat_type: Option<
                    crate::types::neural_common::InfoCounterStatObjectStatType,
                > = None;
                let mut f_value: Option<crate::types::neural_common::InfoCounterStatObjectValue> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stat_type" => {
                            if f_stat_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("stat_type"));
                            }
                            f_stat_type = Some(map.next_value::<crate::types::neural_common::InfoCounterStatObjectStatType>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<crate::types::neural_common::InfoCounterStatObjectValue>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InfoCounterStatObject {
                    stat_type: f_stat_type
                        .ok_or_else(|| serde::de::Error::missing_field("stat_type"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InfoCounterStatObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("stat_type", &self.stat_type)?;
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfoCounterStatObjectStatType {
    #[doc = "`info_counter`"]
    InfoCounter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl InfoCounterStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            InfoCounterStatObjectStatType::InfoCounter => "info_counter",
            InfoCounterStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for InfoCounterStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InfoCounterStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info_counter" => InfoCounterStatObjectStatType::InfoCounter,
            _ => InfoCounterStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoCounterStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for InfoCounterStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoCounterStatObjectValue::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(InfoCounterStatObjectValue::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a InfoCounterStatObjectValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoStringStat {
    String(String),
    Object(crate::types::neural_common::InfoStringStatObject),
}
impl<'de> serde::Deserialize<'de> for InfoStringStat {
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
            Some(b'"') => serde_json::from_str(text).map(InfoStringStat::String),
            Some(b'{') => serde_json::from_str(text).map(InfoStringStat::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a InfoStringStat value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct InfoStringStatObject {
    pub stat_type: crate::types::neural_common::InfoStringStatObjectStatType,
    pub value: crate::types::neural_common::InfoStringStatObjectValue,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for InfoStringStatObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InfoStringStatObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InfoStringStatObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stat_type: Option<
                    crate::types::neural_common::InfoStringStatObjectStatType,
                > = None;
                let mut f_value: Option<crate::types::neural_common::InfoStringStatObjectValue> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stat_type" => {
                            if f_stat_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("stat_type"));
                            }
                            f_stat_type = Some(map.next_value::<crate::types::neural_common::InfoStringStatObjectStatType>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<crate::types::neural_common::InfoStringStatObjectValue>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InfoStringStatObject {
                    stat_type: f_stat_type
                        .ok_or_else(|| serde::de::Error::missing_field("stat_type"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InfoStringStatObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("stat_type", &self.stat_type)?;
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfoStringStatObjectStatType {
    #[doc = "`info_string`"]
    InfoString,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl InfoStringStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            InfoStringStatObjectStatType::InfoString => "info_string",
            InfoStringStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for InfoStringStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InfoStringStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info_string" => InfoStringStatObjectStatType::InfoString,
            _ => InfoStringStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoStringStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for InfoStringStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoStringStatObjectValue::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(InfoStringStatObjectValue::String),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a InfoStringStatObjectValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralInfoStats {
    pub cluster_version: Option<crate::types::neural_common::InfoStringStat>,
    pub processors: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessors>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralInfoStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralInfoStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralInfoStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cluster_version: Option<crate::types::neural_common::InfoStringStat> =
                    None;
                let mut seen_cluster_version = false;
                let mut f_processors: Option<
                    crate::types::neural_common::NestedNeuralInfoStatsProcessors,
                > = None;
                let mut seen_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "cluster_version" => {
                            if seen_cluster_version {
                                return Err(serde::de::Error::duplicate_field("cluster_version"));
                            }
                            seen_cluster_version = true;
                            f_cluster_version = map
                                .next_value::<Option<crate::types::neural_common::InfoStringStat>>(
                                )?;
                        }
                        "processors" => {
                            if seen_processors {
                                return Err(serde::de::Error::duplicate_field("processors"));
                            }
                            seen_processors = true;
                            f_processors = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralInfoStatsProcessors,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralInfoStats {
                    cluster_version: f_cluster_version,
                    processors: f_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralInfoStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cluster_version {
            map.serialize_entry("cluster_version", v)?;
        }
        if let Some(v) = &self.processors {
            map.serialize_entry("processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralInfoStatsProcessors {
    pub ingest: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsIngest>,
    pub search: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearch>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralInfoStatsProcessors {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralInfoStatsProcessors;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralInfoStatsProcessors object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ingest: Option<
                    crate::types::neural_common::NestedNeuralInfoStatsProcessorsIngest,
                > = None;
                let mut seen_ingest = false;
                let mut f_search: Option<
                    crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearch,
                > = None;
                let mut seen_search = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralInfoStatsProcessorsIngest,
                            >>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearch,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralInfoStatsProcessors {
                    ingest: f_ingest,
                    search: f_search,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralInfoStatsProcessors {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralInfoStatsProcessorsIngest {
    pub skip_existing_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub sparse_encoding_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub text_chunking_delimiter_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub text_chunking_fixed_char_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub text_chunking_fixed_token_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub text_chunking_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub text_embedding_processors_in_pipelines:
        Option<crate::types::neural_common::InfoCounterStat>,
    pub text_image_embedding_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralInfoStatsProcessorsIngest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralInfoStatsProcessorsIngest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralInfoStatsProcessorsIngest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_skip_existing_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_skip_existing_processors = false;
                let mut f_sparse_encoding_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_sparse_encoding_processors = false;
                let mut f_text_chunking_delimiter_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_chunking_delimiter_processors = false;
                let mut f_text_chunking_fixed_char_length_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_chunking_fixed_char_length_processors = false;
                let mut f_text_chunking_fixed_token_length_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_chunking_fixed_token_length_processors = false;
                let mut f_text_chunking_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_chunking_processors = false;
                let mut f_text_embedding_processors_in_pipelines: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_embedding_processors_in_pipelines = false;
                let mut f_text_image_embedding_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_text_image_embedding_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "skip_existing_processors" => {
                            if seen_skip_existing_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "skip_existing_processors",
                                ));
                            }
                            seen_skip_existing_processors = true;
                            f_skip_existing_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "sparse_encoding_processors" => {
                            if seen_sparse_encoding_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "sparse_encoding_processors",
                                ));
                            }
                            seen_sparse_encoding_processors = true;
                            f_sparse_encoding_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_chunking_delimiter_processors" => {
                            if seen_text_chunking_delimiter_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_delimiter_processors",
                                ));
                            }
                            seen_text_chunking_delimiter_processors = true;
                            f_text_chunking_delimiter_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_chunking_fixed_char_length_processors" => {
                            if seen_text_chunking_fixed_char_length_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_fixed_char_length_processors",
                                ));
                            }
                            seen_text_chunking_fixed_char_length_processors = true;
                            f_text_chunking_fixed_char_length_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_chunking_fixed_token_length_processors" => {
                            if seen_text_chunking_fixed_token_length_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_fixed_token_length_processors",
                                ));
                            }
                            seen_text_chunking_fixed_token_length_processors = true;
                            f_text_chunking_fixed_token_length_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_chunking_processors" => {
                            if seen_text_chunking_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_processors",
                                ));
                            }
                            seen_text_chunking_processors = true;
                            f_text_chunking_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_embedding_processors_in_pipelines" => {
                            if seen_text_embedding_processors_in_pipelines {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_embedding_processors_in_pipelines",
                                ));
                            }
                            seen_text_embedding_processors_in_pipelines = true;
                            f_text_embedding_processors_in_pipelines = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "text_image_embedding_processors" => {
                            if seen_text_image_embedding_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_image_embedding_processors",
                                ));
                            }
                            seen_text_image_embedding_processors = true;
                            f_text_image_embedding_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralInfoStatsProcessorsIngest {
                    skip_existing_processors: f_skip_existing_processors,
                    sparse_encoding_processors: f_sparse_encoding_processors,
                    text_chunking_delimiter_processors: f_text_chunking_delimiter_processors,
                    text_chunking_fixed_char_length_processors:
                        f_text_chunking_fixed_char_length_processors,
                    text_chunking_fixed_token_length_processors:
                        f_text_chunking_fixed_token_length_processors,
                    text_chunking_processors: f_text_chunking_processors,
                    text_embedding_processors_in_pipelines:
                        f_text_embedding_processors_in_pipelines,
                    text_image_embedding_processors: f_text_image_embedding_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralInfoStatsProcessorsIngest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.skip_existing_processors {
            map.serialize_entry("skip_existing_processors", v)?;
        }
        if let Some(v) = &self.sparse_encoding_processors {
            map.serialize_entry("sparse_encoding_processors", v)?;
        }
        if let Some(v) = &self.text_chunking_delimiter_processors {
            map.serialize_entry("text_chunking_delimiter_processors", v)?;
        }
        if let Some(v) = &self.text_chunking_fixed_char_length_processors {
            map.serialize_entry("text_chunking_fixed_char_length_processors", v)?;
        }
        if let Some(v) = &self.text_chunking_fixed_token_length_processors {
            map.serialize_entry("text_chunking_fixed_token_length_processors", v)?;
        }
        if let Some(v) = &self.text_chunking_processors {
            map.serialize_entry("text_chunking_processors", v)?;
        }
        if let Some(v) = &self.text_embedding_processors_in_pipelines {
            map.serialize_entry("text_embedding_processors_in_pipelines", v)?;
        }
        if let Some(v) = &self.text_image_embedding_processors {
            map.serialize_entry("text_image_embedding_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralInfoStatsProcessorsSearch {
    pub hybrid: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearchHybrid>,
    pub neural_query_enricher_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub neural_sparse_two_phase_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub rerank_by_field_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub rerank_ml_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralInfoStatsProcessorsSearch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralInfoStatsProcessorsSearch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralInfoStatsProcessorsSearch object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hybrid: Option<
                    crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearchHybrid,
                > = None;
                let mut seen_hybrid = false;
                let mut f_neural_query_enricher_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_neural_query_enricher_processors = false;
                let mut f_neural_sparse_two_phase_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_neural_sparse_two_phase_processors = false;
                let mut f_rerank_by_field_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_rerank_by_field_processors = false;
                let mut f_rerank_ml_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_rerank_ml_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearchHybrid>>()?;
                        }
                        "neural_query_enricher_processors" => {
                            if seen_neural_query_enricher_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_enricher_processors",
                                ));
                            }
                            seen_neural_query_enricher_processors = true;
                            f_neural_query_enricher_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "neural_sparse_two_phase_processors" => {
                            if seen_neural_sparse_two_phase_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_sparse_two_phase_processors",
                                ));
                            }
                            seen_neural_sparse_two_phase_processors = true;
                            f_neural_sparse_two_phase_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "rerank_by_field_processors" => {
                            if seen_rerank_by_field_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "rerank_by_field_processors",
                                ));
                            }
                            seen_rerank_by_field_processors = true;
                            f_rerank_by_field_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "rerank_ml_processors" => {
                            if seen_rerank_ml_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "rerank_ml_processors",
                                ));
                            }
                            seen_rerank_ml_processors = true;
                            f_rerank_ml_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralInfoStatsProcessorsSearch {
                    hybrid: f_hybrid,
                    neural_query_enricher_processors: f_neural_query_enricher_processors,
                    neural_sparse_two_phase_processors: f_neural_sparse_two_phase_processors,
                    rerank_by_field_processors: f_rerank_by_field_processors,
                    rerank_ml_processors: f_rerank_ml_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralInfoStatsProcessorsSearch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        if let Some(v) = &self.neural_query_enricher_processors {
            map.serialize_entry("neural_query_enricher_processors", v)?;
        }
        if let Some(v) = &self.neural_sparse_two_phase_processors {
            map.serialize_entry("neural_sparse_two_phase_processors", v)?;
        }
        if let Some(v) = &self.rerank_by_field_processors {
            map.serialize_entry("rerank_by_field_processors", v)?;
        }
        if let Some(v) = &self.rerank_ml_processors {
            map.serialize_entry("rerank_ml_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralInfoStatsProcessorsSearchHybrid {
    pub comb_arithmetic_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub comb_geometric_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub comb_harmonic_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub comb_rrf_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub norm_l_2_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub norm_minmax_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub norm_zscore_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub normalization_processors: Option<crate::types::neural_common::InfoCounterStat>,
    pub rank_based_normalization_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralInfoStatsProcessorsSearchHybrid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralInfoStatsProcessorsSearchHybrid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralInfoStatsProcessorsSearchHybrid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_comb_arithmetic_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_comb_arithmetic_processors = false;
                let mut f_comb_geometric_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_comb_geometric_processors = false;
                let mut f_comb_harmonic_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_comb_harmonic_processors = false;
                let mut f_comb_rrf_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_comb_rrf_processors = false;
                let mut f_norm_l_2_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_norm_l_2_processors = false;
                let mut f_norm_minmax_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_norm_minmax_processors = false;
                let mut f_norm_zscore_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_norm_zscore_processors = false;
                let mut f_normalization_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_normalization_processors = false;
                let mut f_rank_based_normalization_processors: Option<
                    crate::types::neural_common::InfoCounterStat,
                > = None;
                let mut seen_rank_based_normalization_processors = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "comb_arithmetic_processors" => {
                            if seen_comb_arithmetic_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_arithmetic_processors",
                                ));
                            }
                            seen_comb_arithmetic_processors = true;
                            f_comb_arithmetic_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "comb_geometric_processors" => {
                            if seen_comb_geometric_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_geometric_processors",
                                ));
                            }
                            seen_comb_geometric_processors = true;
                            f_comb_geometric_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "comb_harmonic_processors" => {
                            if seen_comb_harmonic_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_harmonic_processors",
                                ));
                            }
                            seen_comb_harmonic_processors = true;
                            f_comb_harmonic_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "comb_rrf_processors" => {
                            if seen_comb_rrf_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_rrf_processors",
                                ));
                            }
                            seen_comb_rrf_processors = true;
                            f_comb_rrf_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "norm_l2_processors" => {
                            if seen_norm_l_2_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_l2_processors",
                                ));
                            }
                            seen_norm_l_2_processors = true;
                            f_norm_l_2_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "norm_minmax_processors" => {
                            if seen_norm_minmax_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_minmax_processors",
                                ));
                            }
                            seen_norm_minmax_processors = true;
                            f_norm_minmax_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "norm_zscore_processors" => {
                            if seen_norm_zscore_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_zscore_processors",
                                ));
                            }
                            seen_norm_zscore_processors = true;
                            f_norm_zscore_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "normalization_processors" => {
                            if seen_normalization_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalization_processors",
                                ));
                            }
                            seen_normalization_processors = true;
                            f_normalization_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        "rank_based_normalization_processors" => {
                            if seen_rank_based_normalization_processors {
                                return Err(serde::de::Error::duplicate_field(
                                    "rank_based_normalization_processors",
                                ));
                            }
                            seen_rank_based_normalization_processors = true;
                            f_rank_based_normalization_processors = map
                                .next_value::<Option<crate::types::neural_common::InfoCounterStat>>(
                                )?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralInfoStatsProcessorsSearchHybrid {
                    comb_arithmetic_processors: f_comb_arithmetic_processors,
                    comb_geometric_processors: f_comb_geometric_processors,
                    comb_harmonic_processors: f_comb_harmonic_processors,
                    comb_rrf_processors: f_comb_rrf_processors,
                    norm_l_2_processors: f_norm_l_2_processors,
                    norm_minmax_processors: f_norm_minmax_processors,
                    norm_zscore_processors: f_norm_zscore_processors,
                    normalization_processors: f_normalization_processors,
                    rank_based_normalization_processors: f_rank_based_normalization_processors,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralInfoStatsProcessorsSearchHybrid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.comb_arithmetic_processors {
            map.serialize_entry("comb_arithmetic_processors", v)?;
        }
        if let Some(v) = &self.comb_geometric_processors {
            map.serialize_entry("comb_geometric_processors", v)?;
        }
        if let Some(v) = &self.comb_harmonic_processors {
            map.serialize_entry("comb_harmonic_processors", v)?;
        }
        if let Some(v) = &self.comb_rrf_processors {
            map.serialize_entry("comb_rrf_processors", v)?;
        }
        if let Some(v) = &self.norm_l_2_processors {
            map.serialize_entry("norm_l2_processors", v)?;
        }
        if let Some(v) = &self.norm_minmax_processors {
            map.serialize_entry("norm_minmax_processors", v)?;
        }
        if let Some(v) = &self.norm_zscore_processors {
            map.serialize_entry("norm_zscore_processors", v)?;
        }
        if let Some(v) = &self.normalization_processors {
            map.serialize_entry("normalization_processors", v)?;
        }
        if let Some(v) = &self.rank_based_normalization_processors {
            map.serialize_entry("rank_based_normalization_processors", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStats {
    pub processors: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessors>,
    pub query: Option<crate::types::neural_common::NestedNeuralNodeStatsQuery>,
    pub semantic_highlighting:
        Option<crate::types::neural_common::NestedNeuralNodeStatsSemanticHighlighting>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStats object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_processors: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsProcessors,
                > = None;
                let mut seen_processors = false;
                let mut f_query: Option<crate::types::neural_common::NestedNeuralNodeStatsQuery> =
                    None;
                let mut seen_query = false;
                let mut f_semantic_highlighting: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsSemanticHighlighting,
                > = None;
                let mut seen_semantic_highlighting = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "processors" => {
                            if seen_processors {
                                return Err(serde::de::Error::duplicate_field("processors"));
                            }
                            seen_processors = true;
                            f_processors = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsProcessors,
                            >>()?;
                        }
                        "query" => {
                            if seen_query {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            seen_query = true;
                            f_query = map.next_value::<Option<crate::types::neural_common::NestedNeuralNodeStatsQuery>>()?;
                        }
                        "semantic_highlighting" => {
                            if seen_semantic_highlighting {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_highlighting",
                                ));
                            }
                            seen_semantic_highlighting = true;
                            f_semantic_highlighting = map.next_value::<Option<crate::types::neural_common::NestedNeuralNodeStatsSemanticHighlighting>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStats {
                    processors: f_processors,
                    query: f_query,
                    semantic_highlighting: f_semantic_highlighting,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.processors {
            map.serialize_entry("processors", v)?;
        }
        if let Some(v) = &self.query {
            map.serialize_entry("query", v)?;
        }
        if let Some(v) = &self.semantic_highlighting {
            map.serialize_entry("semantic_highlighting", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsProcessors {
    pub ingest: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsIngest>,
    pub search: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearch>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsProcessors {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsProcessors;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsProcessors object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_ingest: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsProcessorsIngest,
                > = None;
                let mut seen_ingest = false;
                let mut f_search: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearch,
                > = None;
                let mut seen_search = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "ingest" => {
                            if seen_ingest {
                                return Err(serde::de::Error::duplicate_field("ingest"));
                            }
                            seen_ingest = true;
                            f_ingest = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsProcessorsIngest,
                            >>()?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearch,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsProcessors {
                    ingest: f_ingest,
                    search: f_search,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsProcessors {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.ingest {
            map.serialize_entry("ingest", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsProcessorsIngest {
    pub semantic_field_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub semantic_field_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub skip_existing_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub sparse_encoding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_chunking_delimiter_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_chunking_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_chunking_fixed_char_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_chunking_fixed_token_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_embedding_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub text_image_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsProcessorsIngest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsProcessorsIngest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsProcessorsIngest object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_semantic_field_chunking_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_semantic_field_chunking_executions = false;
                let mut f_semantic_field_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_semantic_field_executions = false;
                let mut f_skip_existing_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_skip_existing_executions = false;
                let mut f_sparse_encoding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_sparse_encoding_executions = false;
                let mut f_text_chunking_delimiter_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_chunking_delimiter_executions = false;
                let mut f_text_chunking_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_chunking_executions = false;
                let mut f_text_chunking_fixed_char_length_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_chunking_fixed_char_length_executions = false;
                let mut f_text_chunking_fixed_token_length_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_chunking_fixed_token_length_executions = false;
                let mut f_text_embedding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_embedding_executions = false;
                let mut f_text_image_embedding_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_text_image_embedding_executions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "semantic_field_chunking_executions" => {
                            if seen_semantic_field_chunking_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_field_chunking_executions",
                                ));
                            }
                            seen_semantic_field_chunking_executions = true;
                            f_semantic_field_chunking_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "semantic_field_executions" => {
                            if seen_semantic_field_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_field_executions",
                                ));
                            }
                            seen_semantic_field_executions = true;
                            f_semantic_field_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "skip_existing_executions" => {
                            if seen_skip_existing_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "skip_existing_executions",
                                ));
                            }
                            seen_skip_existing_executions = true;
                            f_skip_existing_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "sparse_encoding_executions" => {
                            if seen_sparse_encoding_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "sparse_encoding_executions",
                                ));
                            }
                            seen_sparse_encoding_executions = true;
                            f_sparse_encoding_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_chunking_delimiter_executions" => {
                            if seen_text_chunking_delimiter_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_delimiter_executions",
                                ));
                            }
                            seen_text_chunking_delimiter_executions = true;
                            f_text_chunking_delimiter_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_chunking_executions" => {
                            if seen_text_chunking_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_executions",
                                ));
                            }
                            seen_text_chunking_executions = true;
                            f_text_chunking_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_chunking_fixed_char_length_executions" => {
                            if seen_text_chunking_fixed_char_length_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_fixed_char_length_executions",
                                ));
                            }
                            seen_text_chunking_fixed_char_length_executions = true;
                            f_text_chunking_fixed_char_length_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_chunking_fixed_token_length_executions" => {
                            if seen_text_chunking_fixed_token_length_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_chunking_fixed_token_length_executions",
                                ));
                            }
                            seen_text_chunking_fixed_token_length_executions = true;
                            f_text_chunking_fixed_token_length_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_embedding_executions" => {
                            if seen_text_embedding_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_embedding_executions",
                                ));
                            }
                            seen_text_embedding_executions = true;
                            f_text_embedding_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "text_image_embedding_executions" => {
                            if seen_text_image_embedding_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "text_image_embedding_executions",
                                ));
                            }
                            seen_text_image_embedding_executions = true;
                            f_text_image_embedding_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsProcessorsIngest {
                    semantic_field_chunking_executions: f_semantic_field_chunking_executions,
                    semantic_field_executions: f_semantic_field_executions,
                    skip_existing_executions: f_skip_existing_executions,
                    sparse_encoding_executions: f_sparse_encoding_executions,
                    text_chunking_delimiter_executions: f_text_chunking_delimiter_executions,
                    text_chunking_executions: f_text_chunking_executions,
                    text_chunking_fixed_char_length_executions:
                        f_text_chunking_fixed_char_length_executions,
                    text_chunking_fixed_token_length_executions:
                        f_text_chunking_fixed_token_length_executions,
                    text_embedding_executions: f_text_embedding_executions,
                    text_image_embedding_executions: f_text_image_embedding_executions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsProcessorsIngest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.semantic_field_chunking_executions {
            map.serialize_entry("semantic_field_chunking_executions", v)?;
        }
        if let Some(v) = &self.semantic_field_executions {
            map.serialize_entry("semantic_field_executions", v)?;
        }
        if let Some(v) = &self.skip_existing_executions {
            map.serialize_entry("skip_existing_executions", v)?;
        }
        if let Some(v) = &self.sparse_encoding_executions {
            map.serialize_entry("sparse_encoding_executions", v)?;
        }
        if let Some(v) = &self.text_chunking_delimiter_executions {
            map.serialize_entry("text_chunking_delimiter_executions", v)?;
        }
        if let Some(v) = &self.text_chunking_executions {
            map.serialize_entry("text_chunking_executions", v)?;
        }
        if let Some(v) = &self.text_chunking_fixed_char_length_executions {
            map.serialize_entry("text_chunking_fixed_char_length_executions", v)?;
        }
        if let Some(v) = &self.text_chunking_fixed_token_length_executions {
            map.serialize_entry("text_chunking_fixed_token_length_executions", v)?;
        }
        if let Some(v) = &self.text_embedding_executions {
            map.serialize_entry("text_embedding_executions", v)?;
        }
        if let Some(v) = &self.text_image_embedding_executions {
            map.serialize_entry("text_image_embedding_executions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsProcessorsSearch {
    pub hybrid: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearchHybrid>,
    pub neural_query_enricher_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub neural_sparse_two_phase_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub rerank_by_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub rerank_ml_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsProcessorsSearch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsProcessorsSearch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsProcessorsSearch object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hybrid: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearchHybrid,
                > = None;
                let mut seen_hybrid = false;
                let mut f_neural_query_enricher_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_query_enricher_executions = false;
                let mut f_neural_sparse_two_phase_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_sparse_two_phase_executions = false;
                let mut f_rerank_by_field_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_rerank_by_field_executions = false;
                let mut f_rerank_ml_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_rerank_ml_executions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearchHybrid>>()?;
                        }
                        "neural_query_enricher_executions" => {
                            if seen_neural_query_enricher_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_enricher_executions",
                                ));
                            }
                            seen_neural_query_enricher_executions = true;
                            f_neural_query_enricher_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "neural_sparse_two_phase_executions" => {
                            if seen_neural_sparse_two_phase_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_sparse_two_phase_executions",
                                ));
                            }
                            seen_neural_sparse_two_phase_executions = true;
                            f_neural_sparse_two_phase_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "rerank_by_field_executions" => {
                            if seen_rerank_by_field_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "rerank_by_field_executions",
                                ));
                            }
                            seen_rerank_by_field_executions = true;
                            f_rerank_by_field_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "rerank_ml_executions" => {
                            if seen_rerank_ml_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "rerank_ml_executions",
                                ));
                            }
                            seen_rerank_ml_executions = true;
                            f_rerank_ml_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsProcessorsSearch {
                    hybrid: f_hybrid,
                    neural_query_enricher_executions: f_neural_query_enricher_executions,
                    neural_sparse_two_phase_executions: f_neural_sparse_two_phase_executions,
                    rerank_by_field_executions: f_rerank_by_field_executions,
                    rerank_ml_executions: f_rerank_ml_executions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsProcessorsSearch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        if let Some(v) = &self.neural_query_enricher_executions {
            map.serialize_entry("neural_query_enricher_executions", v)?;
        }
        if let Some(v) = &self.neural_sparse_two_phase_executions {
            map.serialize_entry("neural_sparse_two_phase_executions", v)?;
        }
        if let Some(v) = &self.rerank_by_field_executions {
            map.serialize_entry("rerank_by_field_executions", v)?;
        }
        if let Some(v) = &self.rerank_ml_executions {
            map.serialize_entry("rerank_ml_executions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsProcessorsSearchHybrid {
    pub comb_arithmetic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub comb_geometric_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub comb_harmonic_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub comb_rrf_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub norm_l_2_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub norm_minmax_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub norm_zscore_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub rank_based_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsProcessorsSearchHybrid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsProcessorsSearchHybrid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsProcessorsSearchHybrid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_comb_arithmetic_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_comb_arithmetic_executions = false;
                let mut f_comb_geometric_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_comb_geometric_executions = false;
                let mut f_comb_harmonic_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_comb_harmonic_executions = false;
                let mut f_comb_rrf_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_comb_rrf_executions = false;
                let mut f_norm_l_2_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_norm_l_2_executions = false;
                let mut f_norm_minmax_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_norm_minmax_executions = false;
                let mut f_norm_zscore_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_norm_zscore_executions = false;
                let mut f_normalization_processor_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_normalization_processor_executions = false;
                let mut f_rank_based_normalization_processor_executions: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_rank_based_normalization_processor_executions = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "comb_arithmetic_executions" => {
                            if seen_comb_arithmetic_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_arithmetic_executions",
                                ));
                            }
                            seen_comb_arithmetic_executions = true;
                            f_comb_arithmetic_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "comb_geometric_executions" => {
                            if seen_comb_geometric_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_geometric_executions",
                                ));
                            }
                            seen_comb_geometric_executions = true;
                            f_comb_geometric_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "comb_harmonic_executions" => {
                            if seen_comb_harmonic_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_harmonic_executions",
                                ));
                            }
                            seen_comb_harmonic_executions = true;
                            f_comb_harmonic_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "comb_rrf_executions" => {
                            if seen_comb_rrf_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "comb_rrf_executions",
                                ));
                            }
                            seen_comb_rrf_executions = true;
                            f_comb_rrf_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "norm_l2_executions" => {
                            if seen_norm_l_2_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_l2_executions",
                                ));
                            }
                            seen_norm_l_2_executions = true;
                            f_norm_l_2_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "norm_minmax_executions" => {
                            if seen_norm_minmax_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_minmax_executions",
                                ));
                            }
                            seen_norm_minmax_executions = true;
                            f_norm_minmax_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "norm_zscore_executions" => {
                            if seen_norm_zscore_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "norm_zscore_executions",
                                ));
                            }
                            seen_norm_zscore_executions = true;
                            f_norm_zscore_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "normalization_processor_executions" => {
                            if seen_normalization_processor_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalization_processor_executions",
                                ));
                            }
                            seen_normalization_processor_executions = true;
                            f_normalization_processor_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "rank_based_normalization_processor_executions" => {
                            if seen_rank_based_normalization_processor_executions {
                                return Err(serde::de::Error::duplicate_field(
                                    "rank_based_normalization_processor_executions",
                                ));
                            }
                            seen_rank_based_normalization_processor_executions = true;
                            f_rank_based_normalization_processor_executions =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsProcessorsSearchHybrid {
                    comb_arithmetic_executions: f_comb_arithmetic_executions,
                    comb_geometric_executions: f_comb_geometric_executions,
                    comb_harmonic_executions: f_comb_harmonic_executions,
                    comb_rrf_executions: f_comb_rrf_executions,
                    norm_l_2_executions: f_norm_l_2_executions,
                    norm_minmax_executions: f_norm_minmax_executions,
                    norm_zscore_executions: f_norm_zscore_executions,
                    normalization_processor_executions: f_normalization_processor_executions,
                    rank_based_normalization_processor_executions:
                        f_rank_based_normalization_processor_executions,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsProcessorsSearchHybrid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.comb_arithmetic_executions {
            map.serialize_entry("comb_arithmetic_executions", v)?;
        }
        if let Some(v) = &self.comb_geometric_executions {
            map.serialize_entry("comb_geometric_executions", v)?;
        }
        if let Some(v) = &self.comb_harmonic_executions {
            map.serialize_entry("comb_harmonic_executions", v)?;
        }
        if let Some(v) = &self.comb_rrf_executions {
            map.serialize_entry("comb_rrf_executions", v)?;
        }
        if let Some(v) = &self.norm_l_2_executions {
            map.serialize_entry("norm_l2_executions", v)?;
        }
        if let Some(v) = &self.norm_minmax_executions {
            map.serialize_entry("norm_minmax_executions", v)?;
        }
        if let Some(v) = &self.norm_zscore_executions {
            map.serialize_entry("norm_zscore_executions", v)?;
        }
        if let Some(v) = &self.normalization_processor_executions {
            map.serialize_entry("normalization_processor_executions", v)?;
        }
        if let Some(v) = &self.rank_based_normalization_processor_executions {
            map.serialize_entry("rank_based_normalization_processor_executions", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsQuery {
    pub hybrid: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryHybrid>,
    pub neural: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryNeural>,
    pub neural_sparse: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryNeuralSparse>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsQuery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsQuery object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hybrid: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsQueryHybrid,
                > = None;
                let mut seen_hybrid = false;
                let mut f_neural: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsQueryNeural,
                > = None;
                let mut seen_neural = false;
                let mut f_neural_sparse: Option<
                    crate::types::neural_common::NestedNeuralNodeStatsQueryNeuralSparse,
                > = None;
                let mut seen_neural_sparse = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hybrid" => {
                            if seen_hybrid {
                                return Err(serde::de::Error::duplicate_field("hybrid"));
                            }
                            seen_hybrid = true;
                            f_hybrid = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsQueryHybrid,
                            >>()?;
                        }
                        "neural" => {
                            if seen_neural {
                                return Err(serde::de::Error::duplicate_field("neural"));
                            }
                            seen_neural = true;
                            f_neural = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsQueryNeural,
                            >>()?;
                        }
                        "neural_sparse" => {
                            if seen_neural_sparse {
                                return Err(serde::de::Error::duplicate_field("neural_sparse"));
                            }
                            seen_neural_sparse = true;
                            f_neural_sparse = map.next_value::<Option<
                                crate::types::neural_common::NestedNeuralNodeStatsQueryNeuralSparse,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsQuery {
                    hybrid: f_hybrid,
                    neural: f_neural,
                    neural_sparse: f_neural_sparse,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsQuery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hybrid {
            map.serialize_entry("hybrid", v)?;
        }
        if let Some(v) = &self.neural {
            map.serialize_entry("neural", v)?;
        }
        if let Some(v) = &self.neural_sparse {
            map.serialize_entry("neural_sparse", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsQueryHybrid {
    pub hybrid_query_requests: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub hybrid_query_with_filter_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub hybrid_query_with_inner_hits_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub hybrid_query_with_pagination_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsQueryHybrid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsQueryHybrid;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsQueryHybrid object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_hybrid_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_hybrid_query_requests = false;
                let mut f_hybrid_query_with_filter_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_hybrid_query_with_filter_requests = false;
                let mut f_hybrid_query_with_inner_hits_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_hybrid_query_with_inner_hits_requests = false;
                let mut f_hybrid_query_with_pagination_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_hybrid_query_with_pagination_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "hybrid_query_requests" => {
                            if seen_hybrid_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "hybrid_query_requests",
                                ));
                            }
                            seen_hybrid_query_requests = true;
                            f_hybrid_query_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "hybrid_query_with_filter_requests" => {
                            if seen_hybrid_query_with_filter_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "hybrid_query_with_filter_requests",
                                ));
                            }
                            seen_hybrid_query_with_filter_requests = true;
                            f_hybrid_query_with_filter_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "hybrid_query_with_inner_hits_requests" => {
                            if seen_hybrid_query_with_inner_hits_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "hybrid_query_with_inner_hits_requests",
                                ));
                            }
                            seen_hybrid_query_with_inner_hits_requests = true;
                            f_hybrid_query_with_inner_hits_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "hybrid_query_with_pagination_requests" => {
                            if seen_hybrid_query_with_pagination_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "hybrid_query_with_pagination_requests",
                                ));
                            }
                            seen_hybrid_query_with_pagination_requests = true;
                            f_hybrid_query_with_pagination_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsQueryHybrid {
                    hybrid_query_requests: f_hybrid_query_requests,
                    hybrid_query_with_filter_requests: f_hybrid_query_with_filter_requests,
                    hybrid_query_with_inner_hits_requests: f_hybrid_query_with_inner_hits_requests,
                    hybrid_query_with_pagination_requests: f_hybrid_query_with_pagination_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsQueryHybrid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.hybrid_query_requests {
            map.serialize_entry("hybrid_query_requests", v)?;
        }
        if let Some(v) = &self.hybrid_query_with_filter_requests {
            map.serialize_entry("hybrid_query_with_filter_requests", v)?;
        }
        if let Some(v) = &self.hybrid_query_with_inner_hits_requests {
            map.serialize_entry("hybrid_query_with_inner_hits_requests", v)?;
        }
        if let Some(v) = &self.hybrid_query_with_pagination_requests {
            map.serialize_entry("hybrid_query_with_pagination_requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsQueryNeural {
    pub neural_query_against_knn_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub neural_query_against_semantic_dense_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub neural_query_against_semantic_sparse_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    pub neural_query_requests: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsQueryNeural {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsQueryNeural;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsQueryNeural object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_neural_query_against_knn_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_query_against_knn_requests = false;
                let mut f_neural_query_against_semantic_dense_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_query_against_semantic_dense_requests = false;
                let mut f_neural_query_against_semantic_sparse_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_query_against_semantic_sparse_requests = false;
                let mut f_neural_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_query_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "neural_query_against_knn_requests" => {
                            if seen_neural_query_against_knn_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_against_knn_requests",
                                ));
                            }
                            seen_neural_query_against_knn_requests = true;
                            f_neural_query_against_knn_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "neural_query_against_semantic_dense_requests" => {
                            if seen_neural_query_against_semantic_dense_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_against_semantic_dense_requests",
                                ));
                            }
                            seen_neural_query_against_semantic_dense_requests = true;
                            f_neural_query_against_semantic_dense_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "neural_query_against_semantic_sparse_requests" => {
                            if seen_neural_query_against_semantic_sparse_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_against_semantic_sparse_requests",
                                ));
                            }
                            seen_neural_query_against_semantic_sparse_requests = true;
                            f_neural_query_against_semantic_sparse_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        "neural_query_requests" => {
                            if seen_neural_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_query_requests",
                                ));
                            }
                            seen_neural_query_requests = true;
                            f_neural_query_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsQueryNeural {
                    neural_query_against_knn_requests: f_neural_query_against_knn_requests,
                    neural_query_against_semantic_dense_requests:
                        f_neural_query_against_semantic_dense_requests,
                    neural_query_against_semantic_sparse_requests:
                        f_neural_query_against_semantic_sparse_requests,
                    neural_query_requests: f_neural_query_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsQueryNeural {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.neural_query_against_knn_requests {
            map.serialize_entry("neural_query_against_knn_requests", v)?;
        }
        if let Some(v) = &self.neural_query_against_semantic_dense_requests {
            map.serialize_entry("neural_query_against_semantic_dense_requests", v)?;
        }
        if let Some(v) = &self.neural_query_against_semantic_sparse_requests {
            map.serialize_entry("neural_query_against_semantic_sparse_requests", v)?;
        }
        if let Some(v) = &self.neural_query_requests {
            map.serialize_entry("neural_query_requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsQueryNeuralSparse {
    pub neural_sparse_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsQueryNeuralSparse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsQueryNeuralSparse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsQueryNeuralSparse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_neural_sparse_query_requests: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_neural_sparse_query_requests = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "neural_sparse_query_requests" => {
                            if seen_neural_sparse_query_requests {
                                return Err(serde::de::Error::duplicate_field(
                                    "neural_sparse_query_requests",
                                ));
                            }
                            seen_neural_sparse_query_requests = true;
                            f_neural_sparse_query_requests =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsQueryNeuralSparse {
                    neural_sparse_query_requests: f_neural_sparse_query_requests,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsQueryNeuralSparse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.neural_sparse_query_requests {
            map.serialize_entry("neural_sparse_query_requests", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralNodeStatsSemanticHighlighting {
    pub semantic_highlighting_request_count:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralNodeStatsSemanticHighlighting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralNodeStatsSemanticHighlighting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralNodeStatsSemanticHighlighting object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_semantic_highlighting_request_count: Option<
                    crate::types::neural_common::TimestampedEventCounterStat,
                > = None;
                let mut seen_semantic_highlighting_request_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "semantic_highlighting_request_count" => {
                            if seen_semantic_highlighting_request_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "semantic_highlighting_request_count",
                                ));
                            }
                            seen_semantic_highlighting_request_count = true;
                            f_semantic_highlighting_request_count =
                                map.next_value::<Option<
                                    crate::types::neural_common::TimestampedEventCounterStat,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralNodeStatsSemanticHighlighting {
                    semantic_highlighting_request_count: f_semantic_highlighting_request_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralNodeStatsSemanticHighlighting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.semantic_highlighting_request_count {
            map.serialize_entry("semantic_highlighting_request_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct NestedNeuralStats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub all_nodes: Option<crate::types::neural_common::NestedNeuralNodeStats>,
    pub cluster_name: Option<String>,
    pub info: Option<crate::types::neural_common::NestedNeuralInfoStats>,
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::neural_common::NestedNeuralNodeStats>,
    >,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedNeuralStats {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedNeuralStats;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedNeuralStats object")
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
                let mut f_all_nodes: Option<crate::types::neural_common::NestedNeuralNodeStats> =
                    None;
                let mut seen_all_nodes = false;
                let mut f_cluster_name: Option<String> = None;
                let mut seen_cluster_name = false;
                let mut f_info: Option<crate::types::neural_common::NestedNeuralInfoStats> = None;
                let mut seen_info = false;
                let mut f__nodes: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::types::neural_common::NestedNeuralNodeStats,
                    >,
                > = None;
                let mut seen__nodes = false;
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
                        "all_nodes" => {
                            if seen_all_nodes {
                                return Err(serde::de::Error::duplicate_field("all_nodes"));
                            }
                            seen_all_nodes = true;
                            f_all_nodes = map.next_value::<Option<crate::types::neural_common::NestedNeuralNodeStats>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name = map.next_value::<Option<String>>()?;
                        }
                        "info" => {
                            if seen_info {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            seen_info = true;
                            f_info = map.next_value::<Option<crate::types::neural_common::NestedNeuralInfoStats>>()?;
                        }
                        "nodes" => {
                            if seen__nodes {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            seen__nodes = true;
                            f__nodes = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::neural_common::NestedNeuralNodeStats,
                                >,
                            >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(NestedNeuralStats {
                    nodes: f_nodes,
                    all_nodes: f_all_nodes,
                    cluster_name: f_cluster_name,
                    info: f_info,
                    _nodes: f__nodes,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedNeuralStats {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.all_nodes {
            map.serialize_entry("all_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        if let Some(v) = &self.info {
            map.serialize_entry("info", v)?;
        }
        if let Some(v) = &self._nodes {
            map.serialize_entry("nodes", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NeuralStats {
    raw: Box<serde_json::value::RawValue>,
}
impl NeuralStats {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `nested_neural_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nested_neural_stats(
        &self,
    ) -> std::result::Result<
        crate::types::neural_common::NestedNeuralStats,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nested_neural_stats")
    }
    #[doc = "Parses the value as the `flat_neural_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_flat_neural_stats(
        &self,
    ) -> std::result::Result<
        crate::types::neural_common::FlatNeuralStats,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "flat_neural_stats")
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TimestampedEventCounterStat {
    Integer(i64),
    Object(crate::types::neural_common::TimestampedEventCounterStatObject),
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStat {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(TimestampedEventCounterStat::Integer)
            }
            Some(b'{') => serde_json::from_str(text).map(TimestampedEventCounterStat::Object),
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TimestampedEventCounterStat value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct TimestampedEventCounterStatObject {
    pub stat_type: crate::types::neural_common::TimestampedEventCounterStatObjectStatType,
    pub value: crate::types::neural_common::TimestampedEventCounterStatObjectValue,
    pub minutes_since_last_event: Option<i64>,
    pub trailing_interval_value: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStatObject {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TimestampedEventCounterStatObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TimestampedEventCounterStatObject object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_stat_type: Option<
                    crate::types::neural_common::TimestampedEventCounterStatObjectStatType,
                > = None;
                let mut f_value: Option<
                    crate::types::neural_common::TimestampedEventCounterStatObjectValue,
                > = None;
                let mut f_minutes_since_last_event: Option<i64> = None;
                let mut seen_minutes_since_last_event = false;
                let mut f_trailing_interval_value: Option<i64> = None;
                let mut seen_trailing_interval_value = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "stat_type" => {
                            if f_stat_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("stat_type"));
                            }
                            f_stat_type = Some(map.next_value::<crate::types::neural_common::TimestampedEventCounterStatObjectStatType>()?);
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<crate::types::neural_common::TimestampedEventCounterStatObjectValue>()?);
                        }
                        "minutes_since_last_event" => {
                            if seen_minutes_since_last_event {
                                return Err(serde::de::Error::duplicate_field(
                                    "minutes_since_last_event",
                                ));
                            }
                            seen_minutes_since_last_event = true;
                            f_minutes_since_last_event = map.next_value::<Option<i64>>()?;
                        }
                        "trailing_interval_value" => {
                            if seen_trailing_interval_value {
                                return Err(serde::de::Error::duplicate_field(
                                    "trailing_interval_value",
                                ));
                            }
                            seen_trailing_interval_value = true;
                            f_trailing_interval_value = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TimestampedEventCounterStatObject {
                    stat_type: f_stat_type
                        .ok_or_else(|| serde::de::Error::missing_field("stat_type"))?,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    minutes_since_last_event: f_minutes_since_last_event,
                    trailing_interval_value: f_trailing_interval_value,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TimestampedEventCounterStatObject {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("stat_type", &self.stat_type)?;
        map.serialize_entry("value", &self.value)?;
        if let Some(v) = &self.minutes_since_last_event {
            map.serialize_entry("minutes_since_last_event", v)?;
        }
        if let Some(v) = &self.trailing_interval_value {
            map.serialize_entry("trailing_interval_value", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimestampedEventCounterStatObjectStatType {
    #[doc = "`timestamped_event_counter`"]
    TimestampedEventCounter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TimestampedEventCounterStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TimestampedEventCounterStatObjectStatType::TimestampedEventCounter => {
                "timestamped_event_counter"
            }
            TimestampedEventCounterStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TimestampedEventCounterStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "timestamped_event_counter" => {
                TimestampedEventCounterStatObjectStatType::TimestampedEventCounter
            }
            _ => TimestampedEventCounterStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TimestampedEventCounterStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(TimestampedEventCounterStatObjectValue::Integer)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(TimestampedEventCounterStatObjectValue::String)
            }
            other => {
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a TimestampedEventCounterStatObjectValue value",
                    crate::types::union::json_kind_name(other.copied())
                )))
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
