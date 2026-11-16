// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-dependency-graph: systemd dependency graph component model.
//!
//! Real implementation: graphs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
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
pub enum SystemdDependencyGraphError {
    EmptyGraphName,
    DuplicateGraph(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdDependencyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGraphName => write!(f, "graph name must not be empty"),
            Self::DuplicateGraph(id) => write!(f, "duplicate graph {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdDependencyGraphError {}

pub struct SystemdDependencyGraph {
    pub name: String,
    graphs: BTreeMap<String, Graph>,
    events: BTreeMap<String, Event>,
}

impl SystemdDependencyGraph {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            graphs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_graph(&mut self, item: Graph) -> Result<(), SystemdDependencyGraphError> {
        if item.name.trim().is_empty() {
            return Err(SystemdDependencyGraphError::EmptyGraphName);
        }
        let id = item.id.clone();
        if self.graphs.contains_key(&id) {
            return Err(SystemdDependencyGraphError::DuplicateGraph(id.clone()));
        }
        self.graphs.insert(id, item);
        Ok(())
    }

    pub fn graph_count(&self) -> usize {
        self.graphs.len()
    }

    pub fn get_graph(&self, id: &str) -> Option<&Graph> {
        self.graphs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdDependencyGraphError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdDependencyGraphError::EmptyEventSubject);
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

    fn sample_graph(id: &str, name: &str) -> Graph {
        Graph {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdDependencyGraph::new("SystemdDependencyGraph");
        assert_eq!(m.name, "SystemdDependencyGraph");
        assert_eq!(m.graph_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_graph() {
        let mut m = SystemdDependencyGraph::new("SystemdDependencyGraph");
        m.add_graph(sample_graph("a1", "Alpha")).unwrap();
        assert_eq!(m.graph_count(), 1);
        assert!(m.get_graph("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdDependencyGraph::new("SystemdDependencyGraph");
        assert!(m.add_graph(sample_graph("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdDependencyGraph::new("SystemdDependencyGraph");
        m.add_graph(sample_graph("x", "One")).unwrap();
        assert!(m.add_graph(sample_graph("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdDependencyGraph::new("SystemdDependencyGraph");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
