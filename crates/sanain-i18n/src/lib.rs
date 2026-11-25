//! Localization of UI strings with Project Fluent.
//!
//! Resources are embedded at build time; nothing is fetched at runtime.
//! Lookup falls back per message: requested locale → its base language →
//! en-US. A message missing everywhere renders as its ID so the gap is
//! visible instead of blank.

use std::collections::HashMap;

use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;
use unic_langid::LanguageIdentifier;

pub use fluent_bundle::{FluentArgs, FluentValue};

/// The canonical locale. Every message must exist here.
pub const FALLBACK: &str = "en-US";

/// Embedded locale resources: (BCP 47 tag, FTL source).
pub const RESOURCES: &[(&str, &str)] = &[
    ("en-US", include_str!("../../../locales/en-US/main.ftl")),
    ("fi-FI", include_str!("../../../locales/fi-FI/main.ftl")),
    ("cs-CZ", include_str!("../../../locales/cs-CZ/main.ftl")),
    ("da-DK", include_str!("../../../locales/da-DK/main.ftl")),
    ("de-DE", include_str!("../../../locales/de-DE/main.ftl")),
    ("el-GR", include_str!("../../../locales/el-GR/main.ftl")),
    ("es-ES", include_str!("../../../locales/es-ES/main.ftl")),
    ("fr-FR", include_str!("../../../locales/fr-FR/main.ftl")),
    ("hu-HU", include_str!("../../../locales/hu-HU/main.ftl")),
    ("id-ID", include_str!("../../../locales/id-ID/main.ftl")),
    ("it-IT", include_str!("../../../locales/it-IT/main.ftl")),
    ("ja-JP", include_str!("../../../locales/ja-JP/main.ftl")),
    ("ko-KR", include_str!("../../../locales/ko-KR/main.ftl")),
    ("nb-NO", include_str!("../../../locales/nb-NO/main.ftl")),
    ("nl-NL", include_str!("../../../locales/nl-NL/main.ftl")),
    ("pl-PL", include_str!("../../../locales/pl-PL/main.ftl")),
    ("pt-BR", include_str!("../../../locales/pt-BR/main.ftl")),
    ("pt-PT", include_str!("../../../locales/pt-PT/main.ftl")),
    ("ro-RO", include_str!("../../../locales/ro-RO/main.ftl")),
    ("ru-RU", include_str!("../../../locales/ru-RU/main.ftl")),
    ("sv-SE", include_str!("../../../locales/sv-SE/main.ftl")),
    ("tr-TR", include_str!("../../../locales/tr-TR/main.ftl")),
    ("uk-UA", include_str!("../../../locales/uk-UA/main.ftl")),
    ("vi-VN", include_str!("../../../locales/vi-VN/main.ftl")),
    ("zh-CN", include_str!("../../../locales/zh-CN/main.ftl")),
    ("zh-TW", include_str!("../../../locales/zh-TW/main.ftl")),
];

/// Every shipped UI language with its name in that language, for the
/// language picker. All but en-US are translations awaiting or under
/// native-speaker review (see docs/LOCALIZATION.md).
pub const LANGUAGES: &[(&str, &str)] = &[
    ("en-US", "English"),
    ("fi-FI", "Suomi"),
    ("cs-CZ", "Čeština"),
    ("da-DK", "Dansk"),
    ("de-DE", "Deutsch"),
    ("el-GR", "Ελληνικά"),
    ("es-ES", "Español"),
    ("fr-FR", "Français"),
    ("hu-HU", "Magyar"),
    ("id-ID", "Bahasa Indonesia"),
    ("it-IT", "Italiano"),
    ("ja-JP", "日本語"),
    ("ko-KR", "한국어"),
    ("nb-NO", "Norsk bokmål"),
    ("nl-NL", "Nederlands"),
    ("pl-PL", "Polski"),
    ("pt-BR", "Português (Brasil)"),
    ("pt-PT", "Português (Portugal)"),
    ("ro-RO", "Română"),
    ("ru-RU", "Русский"),
    ("sv-SE", "Svenska"),
    ("tr-TR", "Türkçe"),
    ("uk-UA", "Українська"),
    ("vi-VN", "Tiếng Việt"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceError {
    pub locale: String,
    pub message: String,
}

/// Parse one locale's FTL source into a bundle. Fails on any syntax error
/// or duplicate message ID.
pub fn build_bundle(
    tag: &str,
    source: &str,
) -> Result<FluentBundle<FluentResource>, ResourceError> {
    let err = |message: String| ResourceError {
        locale: tag.to_owned(),
        message,
    };
    let lang: LanguageIdentifier = tag.parse().map_err(|e| err(format!("bad tag: {e}")))?;
    let res = FluentResource::try_new(source.to_owned())
        .map_err(|(_, errs)| err(format!("syntax: {errs:?}")))?;
    let mut bundle = FluentBundle::new_concurrent(vec![lang]);
    bundle
        .add_resource(res)
        .map_err(|errs| err(format!("resource: {errs:?}")))?;
    Ok(bundle)
}

pub struct Localizer {
    /// Bundles in lookup order, ending with en-US.
    chain: Vec<FluentBundle<FluentResource>>,
}

impl std::fmt::Debug for Localizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locales: Vec<String> = self
            .chain
            .iter()
            .filter_map(|b| b.locales.first().map(ToString::to_string))
            .collect();
        f.debug_struct("Localizer")
            .field("chain", &locales)
            .finish()
    }
}

impl Localizer {
    /// Build a localizer for the first supported entry of `requested`
    /// (e.g. OS preferences or the user's explicit choice).
    pub fn new(requested: &[&str]) -> Result<Self, ResourceError> {
        let available: HashMap<&str, &str> = RESOURCES.iter().copied().collect();
        let mut order: Vec<&str> = Vec::new();
        if let Some(tag) = negotiate(requested) {
            order.push(tag);
        }
        if !order.contains(&FALLBACK) {
            order.push(FALLBACK);
        }
        let chain = order
            .into_iter()
            .map(|tag| build_bundle(tag, available[tag]))
            .collect::<Result<_, _>>()?;
        Ok(Self { chain })
    }

    /// The locale actually selected (first in the chain).
    pub fn locale(&self) -> String {
        self.chain
            .first()
            .and_then(|b| b.locales.first())
            .map_or_else(|| FALLBACK.to_owned(), ToString::to_string)
    }

    pub fn get(&self, id: &str) -> String {
        self.format(id, None)
    }

    pub fn format(&self, id: &str, args: Option<&FluentArgs<'_>>) -> String {
        for bundle in &self.chain {
            let Some(pattern) = bundle.get_message(id).and_then(|m| m.value()) else {
                continue;
            };
            let mut errors = Vec::new();
            let text = bundle.format_pattern(pattern, args, &mut errors);
            if errors.is_empty() {
                return text.into_owned();
            }
        }
        id.to_owned()
    }
}

/// Pick the first requested tag we ship, trying exact match then base
/// language (e.g. `fi` or `fi-SE` → `fi-FI`). Never guesses from region.
fn negotiate(requested: &[&str]) -> Option<&'static str> {
    for req in requested {
        let Ok(want) = req.parse::<LanguageIdentifier>() else {
            continue;
        };
        let parsed = RESOURCES
            .iter()
            .filter_map(|(tag, _)| tag.parse::<LanguageIdentifier>().ok().map(|l| (*tag, l)));
        let mut language_match = None;
        for (tag, have) in parsed {
            if have == want {
                return Some(tag);
            }
            if language_match.is_none() && have.language == want.language {
                language_match = Some(tag);
            }
        }
        if language_match.is_some() {
            return language_match;
        }
    }
    None
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Fluent wraps placeables in Unicode isolation marks; strip for asserts.
    fn plain(s: String) -> String {
        s.replace(['\u{2068}', '\u{2069}'], "")
    }

    #[test]
    fn negotiation_exact_language_and_fallback() {
        assert_eq!(Localizer::new(&["fi-FI"]).unwrap().locale(), "fi-FI");
        assert_eq!(Localizer::new(&["fi"]).unwrap().locale(), "fi-FI");
        assert_eq!(Localizer::new(&["fi-SE"]).unwrap().locale(), "fi-FI");
        assert_eq!(Localizer::new(&["sv-FI"]).unwrap().locale(), "sv-SE");
        assert_eq!(Localizer::new(&["is-IS"]).unwrap().locale(), "en-US");
        assert_eq!(Localizer::new(&["xx-!!", "fi"]).unwrap().locale(), "fi-FI");
        assert_eq!(Localizer::new(&[]).unwrap().locale(), "en-US");
    }

    #[test]
    fn plurals_follow_locale_rules() {
        let fi = Localizer::new(&["fi-FI"]).unwrap();
        let en = Localizer::new(&["en-US"]).unwrap();
        for (n, fi_text, en_text) in [
            (1, "Yksi välilehti", "One tab"),
            (3, "3 välilehteä", "3 tabs"),
        ] {
            let mut args = FluentArgs::new();
            args.set("count", n);
            assert_eq!(plain(fi.format("status-tabs", Some(&args))), fi_text);
            assert_eq!(plain(en.format("status-tabs", Some(&args))), en_text);
        }
    }

    #[test]
    fn missing_message_falls_back_then_shows_id() {
        let fi = Localizer::new(&["fi-FI"]).unwrap();
        assert_eq!(fi.get("file-save"), "Tallenna");
        assert_eq!(fi.get("no-such-message"), "no-such-message");
    }

    #[test]
    fn missing_argument_falls_back_rather_than_rendering_partial_text() {
        let en = Localizer::new(&["en-US"]).unwrap();
        // A formatting error in every bundle yields the ID.
        assert_eq!(en.format("error-open", None), "error-open");
    }
}
