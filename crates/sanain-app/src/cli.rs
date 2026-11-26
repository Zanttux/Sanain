//! Command line: `sanain [--diff LEFT RIGHT] [FILE…]`, `--version`, `--help`.
//! Paths are taken literally (no shell interpolation); `--` ends options so
//! a file whose name starts with `--` can still be opened.

use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Cli {
    /// Files to open.
    pub paths: Vec<PathBuf>,
    /// Open both and show them side by side.
    pub diff: Option<(PathBuf, PathBuf)>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Parsed {
    Run(Cli),
    Version,
    Help,
    /// The offending argument.
    Unknown(String),
    /// `--diff` without two paths.
    DiffNeedsTwo,
}

pub fn parse(args: impl IntoIterator<Item = OsString>) -> Parsed {
    let mut cli = Cli::default();
    let mut args = args.into_iter();
    let mut options = true;
    while let Some(a) = args.next() {
        if options && let Some(s) = a.to_str() {
            match s {
                "--" => {
                    options = false;
                    continue;
                }
                "--version" | "-V" => return Parsed::Version,
                "--help" | "-h" => return Parsed::Help,
                "--diff" | "-d" => {
                    let (Some(l), Some(r)) = (args.next(), args.next()) else {
                        return Parsed::DiffNeedsTwo;
                    };
                    cli.diff = Some((l.into(), r.into()));
                    continue;
                }
                s if s.starts_with('-') && s.len() > 1 => return Parsed::Unknown(s.to_owned()),
                _ => {}
            }
        }
        cli.paths.push(a.into());
    }
    Parsed::Run(cli)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(args: &[&str]) -> Parsed {
        parse(args.iter().map(OsString::from))
    }

    #[test]
    fn parses() {
        assert_eq!(p(&[]), Parsed::Run(Cli::default()));
        assert_eq!(
            p(&["a.txt", "b.txt"]),
            Parsed::Run(Cli {
                paths: vec!["a.txt".into(), "b.txt".into()],
                diff: None
            })
        );
        assert_eq!(
            p(&["--diff", "old", "new", "x"]),
            Parsed::Run(Cli {
                paths: vec!["x".into()],
                diff: Some(("old".into(), "new".into()))
            })
        );
        assert_eq!(p(&["--diff", "old"]), Parsed::DiffNeedsTwo);
        assert_eq!(p(&["--version"]), Parsed::Version);
        assert_eq!(p(&["-h"]), Parsed::Help);
        assert_eq!(p(&["--nope"]), Parsed::Unknown("--nope".into()));
        assert_eq!(
            p(&["--", "--nope", "-"]),
            Parsed::Run(Cli {
                paths: vec!["--nope".into(), "-".into()],
                diff: None
            })
        );
    }
}
