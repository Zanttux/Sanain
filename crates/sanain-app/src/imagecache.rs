//! Textures for preview images, decoded on background threads.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::SystemTime;

use eframe::egui;
use sanain_app::images::{self, ImageError, Rgba};

/// What the preview should show for an image reference.
#[derive(Clone)]
pub enum ImageView {
    Ready(egui::TextureHandle),
    Loading,
    /// Localization ID of a short explanation shown instead of the image.
    Note(&'static str),
}

type Key = (PathBuf, Option<SystemTime>);

pub struct ImageCache {
    map: HashMap<Key, ImageView>,
    tx: Sender<(Key, Result<Rgba, ImageError>)>,
    rx: Receiver<(Key, Result<Rgba, ImageError>)>,
}

impl Default for ImageCache {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            map: HashMap::new(),
            tx,
            rx,
        }
    }
}

impl ImageCache {
    /// Turn finished decodes into textures.
    pub fn poll(&mut self, ctx: &egui::Context) {
        for (key, result) in self.rx.try_iter() {
            let view = match result {
                Ok(img) => {
                    let size = [img.width as usize, img.height as usize];
                    let color = egui::ColorImage::from_rgba_unmultiplied(size, &img.pixels);
                    let name = key.0.to_string_lossy().into_owned();
                    ImageView::Ready(ctx.load_texture(name, color, egui::TextureOptions::LINEAR))
                }
                Err(e) => ImageView::Note(e.message_id()),
            };
            self.map.insert(key, view);
        }
    }

    /// The view for a local file, starting a decode if needed. The cache is
    /// keyed by modification time so an edited image reloads.
    pub fn get(&mut self, ctx: &egui::Context, path: &Path) -> ImageView {
        let mtime = std::fs::metadata(path).and_then(|m| m.modified()).ok();
        let key = (path.to_owned(), mtime);
        if let Some(v) = self.map.get(&key) {
            return v.clone();
        }
        self.map.insert(key.clone(), ImageView::Loading);
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        let spawned = std::thread::Builder::new()
            .name("sanain-image".into())
            .spawn(move || {
                let result = images::load(&key.0);
                let _ = tx.send((key, result));
                ctx.request_repaint();
            });
        if spawned.is_err() {
            return ImageView::Note("image-unreadable");
        }
        ImageView::Loading
    }
}
