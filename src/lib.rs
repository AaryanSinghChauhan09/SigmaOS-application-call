// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-application-call: application call component model.
//!
//! Real implementation: calls, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
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
pub enum ApplicationCallError {
    EmptyCallName,
    DuplicateCall(String),
    EmptyEventSubject,
}

impl fmt::Display for ApplicationCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCallName => write!(f, "call name must not be empty"),
            Self::DuplicateCall(id) => write!(f, "duplicate call {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ApplicationCallError {}

pub struct ApplicationCall {
    pub name: String,
    calls: BTreeMap<String, Call>,
    events: BTreeMap<String, Event>,
}

impl ApplicationCall {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            calls: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_call(&mut self, item: Call) -> Result<(), ApplicationCallError> {
        if item.name.trim().is_empty() {
            return Err(ApplicationCallError::EmptyCallName);
        }
        let id = item.id.clone();
        if self.calls.contains_key(&id) {
            return Err(ApplicationCallError::DuplicateCall(id.clone()));
        }
        self.calls.insert(id, item);
        Ok(())
    }

    pub fn call_count(&self) -> usize {
        self.calls.len()
    }

    pub fn get_call(&self, id: &str) -> Option<&Call> {
        self.calls.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ApplicationCallError> {
        if event.subject.trim().is_empty() {
            return Err(ApplicationCallError::EmptyEventSubject);
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

    fn sample_call(id: &str, name: &str) -> Call {
        Call {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ApplicationCall::new("ApplicationCall");
        assert_eq!(m.name, "ApplicationCall");
        assert_eq!(m.call_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_call() {
        let mut m = ApplicationCall::new("ApplicationCall");
        m.add_call(sample_call("a1", "Alpha")).unwrap();
        assert_eq!(m.call_count(), 1);
        assert!(m.get_call("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ApplicationCall::new("ApplicationCall");
        assert!(m.add_call(sample_call("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ApplicationCall::new("ApplicationCall");
        m.add_call(sample_call("x", "One")).unwrap();
        assert!(m.add_call(sample_call("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ApplicationCall::new("ApplicationCall");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
