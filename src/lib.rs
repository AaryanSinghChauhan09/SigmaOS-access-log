//! SigmaOS-access-log: access log model.
//!
//! Real implementation: audit trails, entries, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Log entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub id: String,
    pub name: String,
    pub user: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessLogError {
    EmptyName,
    DuplicateEntry(String),
}

impl fmt::Display for AccessLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "entry name must not be empty"),
            Self::DuplicateEntry(id) => write!(f, "duplicate entry {:?}", id),
        }
    }
}

impl std::error::Error for AccessLogError {}

/// Access log manager.
pub struct AccessLog {
    pub name: String,
    entries: BTreeMap<String, LogEntry>,
}

impl AccessLog {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: BTreeMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: LogEntry) -> Result<(), AccessLogError> {
        if entry.name.trim().is_empty() { return Err(AccessLogError::EmptyName); }
        let id = entry.id.clone();
        if self.entries.contains_key(&id) {
            return Err(AccessLogError::DuplicateEntry(id.clone()));
        }
        self.entries.insert(id, entry);
        Ok(())
    }

    pub fn entry_count(&self) -> usize { self.entries.len() }

    pub fn get_entry(&self, id: &str) -> Option<&LogEntry> { self.entries.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_create_aaa() {
        let l = AccessLog::new("My Log");
        assert_eq!(l.name, "My Log");
        assert_eq!(l.entry_count(), 0);
    }

    #[test]
    fn test_add_entry_aaa() {
        let mut l = AccessLog::new("My Log");
        let e = LogEntry { id: "e1".into(), name: "login".into(), user: "admin".into(), action: "auth".into() };
        l.add_entry(e).unwrap();
        assert_eq!(l.entry_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut l = AccessLog::new("My Log");
        let e = LogEntry { id: "x".into(), name: "".into(), user: "a".into(), action: "b".into() };
        assert!(l.add_entry(e).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut l = AccessLog::new("My Log");
        let e = LogEntry { id: "x".into(), name: "X".into(), user: "a".into(), action: "b".into() };
        l.add_entry(e).unwrap();
        let e2 = LogEntry { id: "x".into(), name: "X".into(), user: "a".into(), action: "b".into() };
        assert!(l.add_entry(e2).is_err());
    }
}
