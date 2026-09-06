/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Per-parameter type overrides, compensating for spec unions the flattened
//! [TypeKind] model cannot express. The target types are handwritten in
//! `opensearch/src/params.rs` outside the generated section.
//!
//! An override applies only when both name and kind match: the same name may
//! carry different kinds on different endpoints (e.g. `slices` is an
//! auto-or-integer union on most endpoints but a plain string on others).

use crate::generator::TypeKind;

/// Matches a [TypeKind] without spelling out [TypeKind::Union] contents
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeKindMatch {
    Boolean,
    Enum,
    Union,
}

impl TypeKindMatch {
    fn matches(&self, kind: &TypeKind) -> bool {
        matches!(
            (self, kind),
            (TypeKindMatch::Boolean, TypeKind::Boolean)
                | (TypeKindMatch::Enum, TypeKind::Enum)
                | (TypeKindMatch::Union, TypeKind::Union(_))
        )
    }
}

/// A parameter whose generated type deviates from the general mapping
struct ParamTypeOverride {
    name: &'static str,
    applies_to: TypeKindMatch,
    /// Type of the request builder struct field
    field_ty: &'static str,
    /// Builder method argument type, when it differs from `field_ty`
    fn_arg_ty: Option<&'static str>,
}

const PARAM_TYPE_OVERRIDES: &[ParamTypeOverride] = &[
    // boolean-or-integer union (_core.search___TrackHits)
    ParamTypeOverride {
        name: "track_total_hits",
        applies_to: TypeKindMatch::Boolean,
        field_ty: "TrackTotalHits",
        fn_arg_ty: Some("Into<TrackTotalHits>"),
    },
    // comma-separated multi-value enum, generated as a slice
    // (https://github.com/elastic/elasticsearch/issues/53212)
    ParamTypeOverride {
        name: "expand_wildcards",
        applies_to: TypeKindMatch::Enum,
        field_ty: "&'b [ExpandWildcards]",
        fn_arg_ty: None,
    },
    // "auto"-or-integer union (_common___Slices)
    ParamTypeOverride {
        name: "slices",
        applies_to: TypeKindMatch::Union,
        field_ty: "Slices",
        fn_arg_ty: None,
    },
];

/// Returns the overridden type for a parameter, or `None` to use the
/// general mapping
pub fn param_type_override(name: &str, kind: &TypeKind, fn_arg: bool) -> Option<&'static str> {
    let o = PARAM_TYPE_OVERRIDES
        .iter()
        .find(|o| o.name == name && o.applies_to.matches(kind))?;
    match (fn_arg, o.fn_arg_ty) {
        (true, Some(fn_arg_ty)) => Some(fn_arg_ty),
        _ => Some(o.field_ty),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_total_hits_overrides_boolean() {
        assert_eq!(
            param_type_override("track_total_hits", &TypeKind::Boolean, false),
            Some("TrackTotalHits")
        );
        assert_eq!(
            param_type_override("track_total_hits", &TypeKind::Boolean, true),
            Some("Into<TrackTotalHits>")
        );
    }

    #[test]
    fn expand_wildcards_overrides_enum_regardless_of_fn_arg() {
        for fn_arg in [false, true] {
            assert_eq!(
                param_type_override("expand_wildcards", &TypeKind::Enum, fn_arg),
                Some("&'b [ExpandWildcards]")
            );
        }
    }

    #[test]
    fn slices_overrides_union() {
        let kind = TypeKind::Union(Box::new((TypeKind::String, TypeKind::Long)));
        assert_eq!(param_type_override("slices", &kind, false), Some("Slices"));
    }

    #[test]
    fn unoverridden_parameters_return_none() {
        assert_eq!(
            param_type_override("pretty", &TypeKind::Boolean, false),
            None
        );
        assert_eq!(param_type_override("timeout", &TypeKind::Time, true), None);
    }

    #[test]
    fn kind_mismatch_falls_through_to_the_general_mapping() {
        // `slices` is a plain string on some endpoints
        assert_eq!(
            param_type_override("slices", &TypeKind::String, false),
            None
        );
        assert_eq!(
            param_type_override("track_total_hits", &TypeKind::String, false),
            None
        );
    }
}
