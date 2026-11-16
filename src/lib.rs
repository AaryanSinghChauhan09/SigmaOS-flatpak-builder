// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-flatpak-builder: flatpak builder component model.
//!
//! Real implementation: builders, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Builder {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlatpakBuilderError {
    EmptyBuilderName,
    DuplicateBuilder(String),
    EmptyEventSubject,
}

impl fmt::Display for FlatpakBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBuilderName => write!(f, "builder name must not be empty"),
            Self::DuplicateBuilder(id) => write!(f, "duplicate builder {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for FlatpakBuilderError {}

pub struct FlatpakBuilder {
    pub name: String,
    builders: BTreeMap<String, Builder>,
    events: BTreeMap<String, Event>,
}

impl FlatpakBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            builders: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_builder(&mut self, item: Builder) -> Result<(), FlatpakBuilderError> {
        if item.name.trim().is_empty() {
            return Err(FlatpakBuilderError::EmptyBuilderName);
        }
        let id = item.id.clone();
        if self.builders.contains_key(&id) {
            return Err(FlatpakBuilderError::DuplicateBuilder(id.clone()));
        }
        self.builders.insert(id, item);
        Ok(())
    }

    pub fn builder_count(&self) -> usize {
        self.builders.len()
    }

    pub fn get_builder(&self, id: &str) -> Option<&Builder> {
        self.builders.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), FlatpakBuilderError> {
        if event.subject.trim().is_empty() {
            return Err(FlatpakBuilderError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_builder(id: &str, name: &str) -> Builder {
        Builder {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = FlatpakBuilder::new("FlatpakBuilder");
        assert_eq!(m.name, "FlatpakBuilder");
        assert_eq!(m.builder_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_builder() {
        let mut m = FlatpakBuilder::new("FlatpakBuilder");
        m.add_builder(sample_builder("a1", "Alpha")).unwrap();
        assert_eq!(m.builder_count(), 1);
        assert!(m.get_builder("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = FlatpakBuilder::new("FlatpakBuilder");
        assert!(m.add_builder(sample_builder("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = FlatpakBuilder::new("FlatpakBuilder");
        m.add_builder(sample_builder("x", "One")).unwrap();
        assert!(m.add_builder(sample_builder("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = FlatpakBuilder::new("FlatpakBuilder");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
