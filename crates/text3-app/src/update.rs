//! Checking for a newer release on GitHub.
//!
//! This only *checks*: it never downloads or runs anything. The user is
//! shown the release page and installs it themselves. Automatic install
//! waits for signed releases and an update manifest (docs/RELEASE.md).
//!
//! Network use is opt-in or user-initiated (PRIVACY.md). The request sends
//! no identifying data beyond a generic `Sanain/<version>` user agent.

use std::time::Duration;

/// All releases, newest first. Not `/releases/latest`: GitHub leaves
/// pre-releases out of it, and every release so far is a pre-release, so
/// it answered 404 ("no releases") forever.
pub const RELEASES_API: &str = "https://api.github.com/repos/Zanttux/Sanain/releases?per_page=30";
/// Release pages we are willing to send the user to.
const RELEASE_PAGE_PREFIX: &str = "https://github.com/Zanttux/Sanain/releases/";
const MAX_RESPONSE: u64 = 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    /// Pre-release identifier (`-beta.1`); `None` sorts after any
    /// pre-release of the same numbers.
    pub pre: Option<String>,
}

impl Version {
    /// Parse `1.2.3`, `v1.2.3` or `1.2.3-beta.1` (build metadata ignored).
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().trim_start_matches('v');
        let s = s.split('+').next()?;
        let (core, pre) = match s.split_once('-') {
            Some((c, p)) if !p.is_empty() => (c, Some(p.to_owned())),
            Some(_) => return None,
            None => (s, None),
        };
        let mut it = core.split('.').map(|n| n.parse::<u64>().ok());
        let v = Version {
            major: it.next()??,
            minor: it.next()??,
            patch: it.next()??,
            pre,
        };
        it.next().is_none().then_some(v)
    }

    /// Semantic-version ordering: equal numbers with a pre-release sort
    /// before the release.
    pub fn is_newer_than(&self, other: &Version) -> bool {
        let nums = |v: &Version| (v.major, v.minor, v.patch);
        match nums(self).cmp(&nums(other)) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => match (&self.pre, &other.pre) {
                (None, Some(_)) => true,
                (Some(a), Some(b)) => a > b,
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub version: Version,
    pub tag: String,
    /// The release page on github.com (validated prefix).
    pub page: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    UpToDate,
    Available(Release),
    /// The project has not published a release yet.
    NoReleases,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    Network(String),
    /// The response was not the expected release information.
    Invalid,
}

/// Extract the release from one GitHub release object.
pub fn parse_release(json: &str) -> Result<Release, UpdateError> {
    let root = text3_tools::json::tree(json).map_err(|_| UpdateError::Invalid)?;
    release_from(&root)
}

/// The newest published release in GitHub's release list: drafts and
/// tags that are not versions (the rolling `nightly`) are skipped;
/// pre-releases count, since every Sanain release is one for now.
/// `Ok(None)` when the list holds no such release.
pub fn parse_releases(json: &str) -> Result<Option<Release>, UpdateError> {
    use text3_tools::json::{Node, tree};
    let root = tree(json).map_err(|_| UpdateError::Invalid)?;
    let Node::Array(items) = &root else {
        return Err(UpdateError::Invalid);
    };
    let mut best: Option<Release> = None;
    for item in items {
        if is_draft(item) {
            continue;
        }
        let Ok(r) = release_from(item) else {
            continue;
        };
        if best
            .as_ref()
            .is_none_or(|b| r.version.is_newer_than(&b.version))
        {
            best = Some(r);
        }
    }
    Ok(best)
}

fn is_draft(node: &text3_tools::json::Node) -> bool {
    use text3_tools::json::Node;
    matches!(node, Node::Object(m) if m.iter().any(|(k, v)| {
        k == "\"draft\"" && matches!(v, Node::Scalar(s) if s == "true")
    }))
}

fn release_from(root: &text3_tools::json::Node) -> Result<Release, UpdateError> {
    use text3_tools::json::Node;
    let Node::Object(members) = root else {
        return Err(UpdateError::Invalid);
    };
    let string = |key: &str| {
        members.iter().find_map(|(k, v)| match v {
            Node::Scalar(s) if k == &format!("\"{key}\"") && s.starts_with('"') => {
                Some(s.trim_matches('"').to_owned())
            }
            _ => None,
        })
    };
    let tag = string("tag_name").ok_or(UpdateError::Invalid)?;
    let page = string("html_url").ok_or(UpdateError::Invalid)?;
    let version = Version::parse(&tag).ok_or(UpdateError::Invalid)?;
    if !page.starts_with(RELEASE_PAGE_PREFIX) || page.contains(['"', '\\', ' ']) {
        return Err(UpdateError::Invalid);
    }
    Ok(Release { version, tag, page })
}

/// Compare the published release with the running version.
pub fn status(current: &str, release: Release) -> Status {
    match Version::parse(current) {
        Some(cur) if release.version.is_newer_than(&cur) => Status::Available(release),
        _ => Status::UpToDate,
    }
}

/// Ask GitHub for the latest release (blocking; call off the UI thread).
pub fn check(current: &str) -> Result<Status, UpdateError> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(15)))
        .http_status_as_error(false)
        .build()
        .into();
    let mut resp = agent
        .get(RELEASES_API)
        .header("User-Agent", &format!("Sanain/{current}"))
        .header("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    match resp.status().as_u16() {
        200 => {}
        code => return Err(UpdateError::Network(format!("HTTP {code}"))),
    }
    let body = resp
        .body_mut()
        .with_config()
        .limit(MAX_RESPONSE)
        .read_to_string()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    Ok(match parse_releases(&body)? {
        Some(release) => status(current, release),
        None => Status::NoReleases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parsing_and_order() {
        let v = |s| Version::parse(s).unwrap();
        assert!(v("0.0.2").is_newer_than(&v("0.0.1")));
        assert!(v("v0.1.0").is_newer_than(&v("0.0.9")));
        assert!(v("1.0.0").is_newer_than(&v("1.0.0-rc.1")));
        assert!(!v("1.0.0-rc.1").is_newer_than(&v("1.0.0")));
        assert!(v("1.0.0-rc.2").is_newer_than(&v("1.0.0-rc.1")));
        assert!(!v("0.0.1").is_newer_than(&v("0.0.1")));
        assert_eq!(v("1.2.3+build.5"), v("1.2.3"));
        for bad in ["", "1", "1.2", "1.2.3.4", "a.b.c", "1.2.3-", "-1.0.0"] {
            assert_eq!(Version::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn parses_github_release_json() {
        let json = r#"{"url":"x","html_url":"https://github.com/Zanttux/Sanain/releases/tag/v0.1.0","tag_name":"v0.1.0","draft":false,"assets":[]}"#;
        let r = parse_release(json).unwrap();
        assert_eq!(r.tag, "v0.1.0");
        assert!(matches!(status("0.0.1", r.clone()), Status::Available(_)));
        assert_eq!(status("0.1.0", r), Status::UpToDate);
    }

    /// The shape GitHub returns today: only pre-releases plus `nightly`.
    #[test]
    fn picks_the_newest_versioned_release_from_the_list() {
        let page = |t: &str| format!("https://github.com/Zanttux/Sanain/releases/tag/{t}");
        let item = |t: &str, pre: bool, draft: bool| {
            format!(
                r#"{{"tag_name":"{t}","html_url":"{}","prerelease":{pre},"draft":{draft}}}"#,
                page(t)
            )
        };
        let json = format!(
            "[{},{},{},{},{}]",
            item("nightly", true, false),
            item("v0.2.1", true, false),
            item("v0.3.0", true, true),
            item("v0.2.0", true, false),
            item("v0.1.0", true, false),
        );
        let r = parse_releases(&json).unwrap().unwrap();
        assert_eq!(r.tag, "v0.2.1", "drafts and nightly skipped");
        assert!(matches!(status("0.2.0", r.clone()), Status::Available(_)));
        assert_eq!(status("0.2.1", r), Status::UpToDate);
        assert_eq!(parse_releases("[]").unwrap(), None);
        let only_nightly = format!("[{}]", item("nightly", true, false));
        assert_eq!(parse_releases(&only_nightly).unwrap(), None);
        for bad in ["not json", "{}", r#"{"tag_name":"v1.0.0"}"#] {
            assert_eq!(parse_releases(bad), Err(UpdateError::Invalid), "{bad}");
        }
        let hostile = r#"[{"tag_name":"v9.0.0","html_url":"https://evil.example/releases/x"}]"#;
        assert_eq!(parse_releases(hostile).unwrap(), None, "bad page ignored");
    }

    #[test]
    fn rejects_unexpected_or_hostile_responses() {
        for json in [
            "not json",
            "[]",
            r#"{"tag_name":"v1.0.0"}"#,
            r#"{"tag_name":"v1.0.0","html_url":"https://evil.example/releases/x"}"#,
            r#"{"tag_name":"v1.0.0","html_url":"https://github.com/Zanttux/Sanain/releases/x\" onclick"}"#,
            r#"{"tag_name":"latest","html_url":"https://github.com/Zanttux/Sanain/releases/x"}"#,
        ] {
            assert_eq!(parse_release(json), Err(UpdateError::Invalid), "{json}");
        }
    }
}
