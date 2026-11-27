//! Start-up measurement (docs/BENCHMARKS.md, scripts/measure-startup.py).
//!
//! With `TEXT3_MEASURE_STARTUP=<file>` set, Text³ notes how long the start
//! took up to a few points and appends them to `<file>` as one line once
//! the first frame (and, when a file was opened, the first frame showing
//! its text) has been presented. With `TEXT3_MEASURE_EXIT=1` it then exits
//! at once. Without the variable nothing is recorded and nothing changes.
//!
//! Times are milliseconds since [`start`], which `main` calls first, plus
//! the wall-clock time of that call (`main_epoch_us`), so a launcher can
//! add the time spent before `main` (exec, dynamic linking). Marks, in
//! order: `settings` (settings file read), `files` (localization, recovery
//! folder, files from the command line opened), `window` (eframe created
//! the window and renderer), `app` (application state built), `first_ui`
//! (first frame laid out, before it is painted), `first_frame` (first
//! frame presented) and, when a file was opened, `text_frame` (first
//! frame showing its text presented).
//!
//! "Presented": egui paints and presents a frame after the app's `ui`
//! returns, so the start of the next frame is the first moment the app
//! knows the previous one is on screen. The probe asks for that next
//! frame itself.

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct Probe {
    file: PathBuf,
    exit: bool,
    t0: Instant,
    epoch_us: u128,
    marks: Vec<(&'static str, Duration)>,
    /// Frames whose `ui` has finished.
    frames: u32,
    /// A file was opened at start-up: wait for a frame showing its text.
    expect_text: bool,
    /// A finished frame drew the text; it is on screen at the next start.
    text_drawn: bool,
    text_done: bool,
    written: bool,
}

static PROBE: Mutex<Option<Probe>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut Probe) -> R) -> Option<R> {
    let mut guard = PROBE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard.as_mut().map(f)
}

/// Start measuring if `TEXT3_MEASURE_STARTUP` is set. Call first in `main`.
pub fn start() {
    let Some(file) = crate::migrate::var_os("MEASURE_STARTUP") else {
        return;
    };
    let epoch_us = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_micros());
    let probe = Probe {
        file: PathBuf::from(file),
        exit: crate::migrate::var("MEASURE_EXIT").as_deref() == Some("1"),
        t0: Instant::now(),
        epoch_us,
        marks: Vec::new(),
        frames: 0,
        expect_text: false,
        text_drawn: false,
        text_done: false,
        written: false,
    };
    *PROBE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(probe);
}

/// Whether measuring is on.
pub fn active() -> bool {
    with(|_| ()).is_some()
}

/// Note that start-up reached `name` now.
pub fn mark(name: &'static str) {
    with(|p| {
        let at = p.t0.elapsed();
        p.marks.push((name, at));
    });
}

/// A file was opened at start-up: also wait for the first frame showing
/// its text.
pub fn expect_text() {
    with(|p| p.expect_text = true);
}

/// Call at the start of every frame. Returns `true` when the app should
/// exit now (`TEXT3_MEASURE_EXIT=1` and everything was recorded).
pub fn frame_start() -> bool {
    with(|p| {
        let now = p.t0.elapsed();
        if let Some(line) = p.begin(now)
            && let Err(e) = append(&p.file, &line)
        {
            eprintln!("text3: could not write {}: {e}", p.file.display());
        }
        p.written && p.exit
    })
    .unwrap_or(false)
}

/// Call at the end of every frame; `text_shown` says whether the frame
/// drew the opened file's text. Returns `true` while the probe still needs
/// another frame (ask egui to repaint).
pub fn frame_end(text_shown: bool) -> bool {
    with(|p| {
        let now = p.t0.elapsed();
        p.end(now, text_shown)
    })
    .unwrap_or(false)
}

impl Probe {
    /// A frame starts `now` after [`start`]: the previous one is on
    /// screen. Returns the line once everything has been recorded.
    fn begin(&mut self, now: Duration) -> Option<String> {
        if self.frames == 1 {
            self.marks.push(("first_frame", now));
        }
        if self.text_drawn && !self.text_done {
            self.text_done = true;
            self.marks.push(("text_frame", now));
        }
        let complete = self.frames >= 1 && (!self.expect_text || self.text_done);
        if complete && !self.written {
            self.written = true;
            return Some(format_line(self.epoch_us, &self.marks));
        }
        None
    }

    /// A frame's `ui` finished `now` (it is painted after this).
    fn end(&mut self, now: Duration, text_shown: bool) -> bool {
        if self.frames == 0 {
            self.marks.push(("first_ui", now));
        }
        self.frames += 1;
        if text_shown {
            self.text_drawn = true;
        }
        !self.written
    }
}

fn append(file: &std::path::Path, line: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)?;
    writeln!(f, "{line}")
}

/// `main_epoch_us=… settings_ms=1.234 …`: space-separated `key=value`.
fn format_line(epoch_us: u128, marks: &[(&'static str, Duration)]) -> String {
    let mut line = format!("main_epoch_us={epoch_us}");
    for (name, at) in marks {
        line.push_str(&format!(" {name}_ms={:.3}", at.as_secs_f64() * 1000.0));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_lists_marks_in_order() {
        let marks = [
            ("settings", Duration::from_micros(1500)),
            ("first_frame", Duration::from_millis(120)),
        ];
        assert_eq!(
            format_line(42, &marks),
            "main_epoch_us=42 settings_ms=1.500 first_frame_ms=120.000"
        );
    }

    #[test]
    fn inactive_without_the_variable() {
        // The test environment never sets TEXT3_MEASURE_STARTUP; start()
        // is not called here, so every hook is a no-op.
        assert!(!active());
        mark("settings");
        assert!(!frame_start());
        assert!(!frame_end(true));
        assert!(!frame_start());
    }

    fn probe(expect_text: bool) -> Probe {
        Probe {
            file: PathBuf::new(),
            exit: true,
            t0: Instant::now(),
            epoch_us: 7,
            marks: Vec::new(),
            frames: 0,
            expect_text,
            text_drawn: false,
            text_done: false,
            written: false,
        }
    }

    #[test]
    fn empty_window_is_done_when_the_first_frame_is_presented() {
        let mut p = probe(false);
        assert_eq!(p.begin(Duration::from_millis(10)), None);
        assert!(
            p.end(Duration::from_millis(20), false),
            "asks for the frame that proves the first is shown"
        );
        let line = p.begin(Duration::from_millis(30)).unwrap();
        assert_eq!(
            line,
            "main_epoch_us=7 first_ui_ms=20.000 first_frame_ms=30.000"
        );
        assert!(!p.end(Duration::from_millis(40), false));
        assert_eq!(p.begin(Duration::from_millis(50)), None, "written once");
    }

    #[test]
    fn opened_file_waits_for_a_frame_with_its_text() {
        let mut p = probe(true);
        p.begin(Duration::from_millis(10));
        p.end(Duration::from_millis(15), false);
        assert_eq!(p.begin(Duration::from_millis(20)), None);
        assert!(p.end(Duration::from_millis(30), true));
        let line = p.begin(Duration::from_millis(40)).unwrap();
        assert_eq!(
            line,
            "main_epoch_us=7 first_ui_ms=15.000 first_frame_ms=20.000 text_frame_ms=40.000"
        );
    }

    #[test]
    fn appends_one_line_per_run() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("m.txt");
        append(&file, "a=1").unwrap();
        append(&file, "b=2").unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "a=1\nb=2\n");
    }
}
