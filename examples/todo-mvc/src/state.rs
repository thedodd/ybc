use serde::{Deserialize, Serialize};

use crate::{entry::Entry, filter::Filter};

#[derive(Deserialize, Serialize)]
pub struct State {
    pub entries: Vec<Entry>,
    pub filter: Filter,
    pub value: String,
    pub edit_value: String,
}

impl State {
    pub fn total(&self) -> usize {
        self.entries.len()
    }

    pub fn total_completed(&self) -> usize {
        self.entries.iter().filter(|entry| Filter::Completed.fits(entry)).count()
    }

    pub fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self.entries.iter().filter(|entry| self.filter.fits(entry)).peekable();

        match filtered_iter.peek() {
            Some(_) => filtered_iter.all(|entry| entry.completed),
            None => false,
        }
    }

    pub fn clear_completed(&mut self) {
        let entries = self.entries.drain(..).filter(|entry| Filter::Active.fits(entry)).collect();
        self.entries = entries;
    }

    pub fn toggle(&mut self, idx: usize) {
        let filter = self.filter;
        if let Some(entry) = self.entries.iter_mut().filter(|entry| filter.fits(entry)).nth(idx) {
            entry.completed = !entry.completed;
        }
    }

    pub fn toggle_all(&mut self, value: bool) {
        for entry in &mut self.entries {
            if self.filter.fits(entry) {
                entry.completed = value;
            }
        }
    }

    pub fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter;
        let entry = self.entries.iter_mut().filter(|entry| filter.fits(entry)).nth(idx).unwrap();
        entry.editing = !entry.editing;
    }

    pub fn clear_all_edit(&mut self) {
        for entry in &mut self.entries {
            entry.editing = false;
        }
    }

    pub fn complete_edit(&mut self, idx: usize, val: String) {
        if val.is_empty() {
            self.remove(idx);
        } else {
            let filter = self.filter;
            let entry = self.entries.iter_mut().filter(|entry| filter.fits(entry)).nth(idx).unwrap();
            entry.description = val;
            entry.editing = !entry.editing;
        }
    }

    pub fn remove(&mut self, filter_idx: usize) {
        let idx_opt = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, entry)| self.filter.fits(entry))
                .map(|(idx, _)| idx)
                .nth(filter_idx);
        if let Some(idx) = idx_opt {
            self.entries.remove(idx);
        };
    }
}
