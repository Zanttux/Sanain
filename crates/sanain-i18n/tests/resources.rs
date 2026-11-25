//! Localization resource checks run in CI: every locale parses, has no
//! duplicate IDs, contains only messages that exist in en-US, and uses the
//! same variables as en-US. Missing translations are listed, not failed,
//! except for en-US which must be complete by definition.

#![allow(clippy::unwrap_used)]

use std::collections::{BTreeMap, BTreeSet};

use fluent_bundle::FluentResource;
use fluent_syntax::ast::{Entry, Expression, InlineExpression, Pattern, PatternElement};
use sanain_i18n::{FALLBACK, RESOURCES, build_bundle};

fn messages(source: &str) -> BTreeMap<String, BTreeSet<String>> {
    let res = FluentResource::try_new(source.to_owned()).unwrap();
    let mut out = BTreeMap::new();
    for entry in res.entries() {
        if let Entry::Message(m) = entry {
            let mut vars = BTreeSet::new();
            if let Some(p) = &m.value {
                collect_vars(p, &mut vars);
            }
            let dup = out.insert(m.id.name.to_owned(), vars);
            assert!(dup.is_none(), "duplicate message id {}", m.id.name);
        }
    }
    out
}

fn collect_vars(p: &Pattern<&str>, vars: &mut BTreeSet<String>) {
    for el in &p.elements {
        if let PatternElement::Placeable { expression } = el {
            collect_expr(expression, vars);
        }
    }
}

fn collect_expr(e: &Expression<&str>, vars: &mut BTreeSet<String>) {
    match e {
        Expression::Inline(i) => collect_inline(i, vars),
        Expression::Select { selector, variants } => {
            collect_inline(selector, vars);
            for v in variants {
                collect_vars(&v.value, vars);
            }
        }
    }
}

fn collect_inline(i: &InlineExpression<&str>, vars: &mut BTreeSet<String>) {
    match i {
        InlineExpression::VariableReference { id } => {
            vars.insert(id.name.to_owned());
        }
        InlineExpression::Placeable { expression } => collect_expr(expression, vars),
        _ => {}
    }
}

#[test]
fn all_locales_parse_and_match_canonical_messages() {
    let canonical = messages(RESOURCES.iter().find(|(t, _)| *t == FALLBACK).unwrap().1);
    assert!(!canonical.is_empty());
    for (tag, source) in RESOURCES {
        build_bundle(tag, source).unwrap_or_else(|e| panic!("{e:?}"));
        let msgs = messages(source);
        for (id, vars) in &msgs {
            let expected = canonical
                .get(id)
                .unwrap_or_else(|| panic!("{tag}: `{id}` does not exist in {FALLBACK}"));
            assert_eq!(vars, expected, "{tag}: `{id}` uses different variables");
        }
        let missing: Vec<_> = canonical
            .keys()
            .filter(|k| !msgs.contains_key(*k))
            .collect();
        if !missing.is_empty() {
            eprintln!("{tag}: {} untranslated: {missing:?}", missing.len());
        }
    }
}

#[test]
fn every_message_formats_without_errors_in_every_locale() {
    use fluent_bundle::FluentArgs;
    let canonical = messages(RESOURCES.iter().find(|(t, _)| *t == FALLBACK).unwrap().1);
    for (tag, source) in RESOURCES {
        let bundle = build_bundle(tag, source).unwrap();
        for (id, vars) in &canonical {
            let Some(pattern) = bundle.get_message(id).and_then(|m| m.value()) else {
                continue;
            };
            // Exercise plural branches with several counts.
            for n in [0, 1, 2, 5, 22] {
                let mut args = FluentArgs::new();
                for v in vars {
                    args.set(v.as_str(), n);
                }
                let mut errors = Vec::new();
                let text = bundle.format_pattern(pattern, Some(&args), &mut errors);
                assert!(errors.is_empty(), "{tag}: `{id}` with {n}: {errors:?}");
                assert!(!text.trim().is_empty(), "{tag}: `{id}` is empty");
            }
        }
    }
}
