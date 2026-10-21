/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Persistent bookmarks and browsing history for servoshell.
//!
//! Data is stored as JSON files in the Servo config directory:
//! `bookmarks.json` and `history.json`.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use log::warn;
use serde::{Deserialize, Serialize};

use crate::prefs::default_config_dir;

/// A single bookmark entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Bookmark {
    pub(crate) url: String,
    pub(crate) title: String,
}

/// Persistent list of bookmarks.
#[derive(Default)]
pub(crate) struct Bookmarks {
    entries: Vec<Bookmark>,
    path: Option<PathBuf>,
}

impl Bookmarks {
    pub(crate) fn load() -> Self {
        let path = default_config_dir().map(|dir| dir.join("bookmarks.json"));
        let entries = path
            .as_ref()
            .filter(|path| path.exists())
            .and_then(|path| {
                fs::read_to_string(path)
                    .ok()
                    .and_then(|contents| serde_json::from_str(&contents).ok())
            })
            .unwrap_or_default();
        Self { entries, path }
    }

    fn save(&self) {
        let Some(path) = &self.path else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match serde_json::to_string_pretty(&self.entries) {
            Ok(json) => {
                if let Err(error) = fs::write(path, json) {
                    warn!("Failed to save bookmarks: {error}");
                }
            },
            Err(error) => warn!("Failed to serialize bookmarks: {error}"),
        }
    }

    pub(crate) fn entries(&self) -> &[Bookmark] {
        &self.entries
    }

    pub(crate) fn contains(&self, url: &str) -> bool {
        self.entries.iter().any(|bookmark| bookmark.url == url)
    }

    pub(crate) fn add(&mut self, url: &str, title: &str) {
        if self.contains(url) {
            return;
        }
        self.entries.push(Bookmark {
            url: url.to_string(),
            title: title.to_string(),
        });
        self.save();
    }

    pub(crate) fn remove(&mut self, url: &str) {
        let old_len = self.entries.len();
        self.entries.retain(|bookmark| bookmark.url != url);
        if self.entries.len() != old_len {
            self.save();
        }
    }
}

/// A single history entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct HistoryEntry {
    pub(crate) url: String,
    pub(crate) title: String,
    /// Seconds since the Unix epoch.
    pub(crate) timestamp: u64,
}

/// Maximum number of history entries kept on disk.
const MAX_HISTORY_ENTRIES: usize = 1000;

/// Maximum number of entries shown in the history UI at once.
pub(crate) const HISTORY_UI_LIMIT: usize = 100;

/// Persistent browsing history, most recent entry first.
#[derive(Default)]
pub(crate) struct History {
    entries: Vec<HistoryEntry>,
    path: Option<PathBuf>,
}

impl History {
    pub(crate) fn load() -> Self {
        let path = default_config_dir().map(|dir| dir.join("history.json"));
        let entries = path
            .as_ref()
            .filter(|path| path.exists())
            .and_then(|path| {
                fs::read_to_string(path)
                    .ok()
                    .and_then(|contents| serde_json::from_str(&contents).ok())
            })
            .unwrap_or_default();
        Self { entries, path }
    }

    fn save(&self) {
        let Some(path) = &self.path else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match serde_json::to_string_pretty(&self.entries) {
            Ok(json) => {
                if let Err(error) = fs::write(path, json) {
                    warn!("Failed to save history: {error}");
                }
            },
            Err(error) => warn!("Failed to serialize history: {error}"),
        }
    }

    /// Record a visit. Consecutive visits to the same URL update the existing entry.
    pub(crate) fn add_visit(&mut self, url: &str, title: &str) {
        // Skip internal pages.
        if url.starts_with("neo:") || url == "about:blank" {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        if let Some(first) = self.entries.first_mut() {
            if first.url == url {
                first.timestamp = timestamp;
                if !title.is_empty() {
                    first.title = title.to_string();
                }
                self.save();
                return;
            }
        }

        self.entries.insert(
            0,
            HistoryEntry {
                url: url.to_string(),
                title: title.to_string(),
                timestamp,
            },
        );
        self.entries.truncate(MAX_HISTORY_ENTRIES);
        self.save();
    }

    /// Search history, most recent first, case-insensitive on URL and title.
    pub(crate) fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        let query = query.trim().to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                query.is_empty() ||
                    entry.url.to_lowercase().contains(&query) ||
                    entry.title.to_lowercase().contains(&query)
            })
            .take(HISTORY_UI_LIMIT)
            .collect()
    }
}
