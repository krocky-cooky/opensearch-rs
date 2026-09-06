/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Per-parameter type overrides.
//!
//! A few query parameters are unions in the REST API specification (e.g.
//! boolean-or-integer) that the flattened [TypeKind] model cannot express;
//! their spec-derived kind would generate a type that loses one of the
//! accepted forms. The entries in this module redirect such parameters to
//! handwritten types (defined outside the generated sections of
//! `opensearch/src/params.rs`) that model the full union.
//!
//! Every override names both the parameter and the [TypeKind] it applies
//! to, because the same parameter name may legitimately carry different
//! kinds on different endpoints (e.g. `slices` is an auto-or-integer union
//! on most endpoints but a plain string on others); parameters whose kind
//! does not match fall through to the general mapping.

use crate::generator::TypeKind;

/// Matches the spec-derived [TypeKind] an override expects, without
/// requiring the boxed contents of [TypeKind::Union] to be spelled out
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

/// A parameter whose generated type deviates from the general
/// [TypeKind] mapping
struct ParamTypeOverride {
    /// The parameter name the override applies to
    name: &'static str,
    /// The [TypeKind] the flattened specification is expected to produce
    /// for this parameter
    applies_to: TypeKindMatch,
    /// The type generated for the request builder struct field
    field_ty: &'static str,
    /// The type generated for the builder method argument, when it differs
    /// from `field_ty` (e.g. an `Into<T>` bound for ergonomics)
    fn_arg_ty: Option<&'static str>,
}

const PARAM_TYPE_OVERRIDES: &[ParamTypeOverride] = &[
    // boolean-or-integer union in the specification
    // (_core.search___TrackHits), flattened to Boolean by the model; the
    // handwritten TrackTotalHits enum models both forms and the Into bound
    // lets callers pass either a bool or an i64
    ParamTypeOverride {
        name: "track_total_hits",
        applies_to: TypeKindMatch::Boolean,
        field_ty: "TrackTotalHits",
        fn_arg_ty: Some("Into<TrackTotalHits>"),
    },
    // accepts multiple comma-separated values, so the enum is generated as
    // a slice. https://github.com/elastic/elasticsearch/issues/53212 was
    // opened to discuss whether this really should be a collection
    ParamTypeOverride {
        name: "expand_wildcards",
        applies_to: TypeKindMatch::Enum,
        field_ty: "&'b [ExpandWildcards]",
        fn_arg_ty: None,
    },
    // "auto"-or-integer union in the specification (_common___Slices),
    // modelled by the handwritten Slices enum
    ParamTypeOverride {
        name: "slices",
        applies_to: TypeKindMatch::Union,
        field_ty: "Slices",
        fn_arg_ty: None,
    },
];

/// Looks up the type override for a parameter, returning the type to
/// generate for the request builder struct field (`fn_arg == false`) or the
/// builder method argument (`fn_arg == true`).
///
/// An override applies only when both the parameter name and the
/// spec-derived [TypeKind] match: the same parameter name may legitimately
/// carry different kinds on different endpoints (e.g. `slices` is an
/// auto-or-integer union on most endpoints but a plain string on others),
/// and only the kind named by the override deviates from the general
/// mapping.
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
        assert_eq!(param_type_override("pretty", &TypeKind::Boolean, false), None);
        assert_eq!(param_type_override("timeout", &TypeKind::Time, true), None);
    }

    #[test]
    fn kind_mismatch_falls_through_to_the_general_mapping() {
        // the same parameter name may carry a different kind on other
        // endpoints: `slices` is a plain string on some APIs and must not
        // be overridden there
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
