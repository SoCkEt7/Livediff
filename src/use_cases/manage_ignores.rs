// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::app::MonitorDomain;
use std::path::Path;

pub struct ManageIgnoresUseCase;

impl Default for ManageIgnoresUseCase {
    fn default() -> Self {
        Self::new()
    }
}

impl ManageIgnoresUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn add_pattern(&self, domain: &mut MonitorDomain, pattern: String) {
        if let Ok(mut engine) = domain.ignore_engine.write() {
            engine.add_ignore(pattern.clone());
        }
        domain.events.push_back(crate::domain::entities::DomainEvent::IgnoreAdded { pattern });
    }

    pub fn remove_pattern(&self, domain: &MonitorDomain, pattern: &str) {
        if let Ok(mut engine) = domain.ignore_engine.write() {
            engine.remove_ignore(pattern);
        }
    }

    pub fn clear_custom_ignores(&self, domain: &MonitorDomain) {
        if let Ok(mut engine) = domain.ignore_engine.write() {
            engine.ignore_list.clear();
            engine.rebuild_globset();
        }
    }

    pub fn reload_vcs_ignores(&self, domain: &MonitorDomain, root_path: &Path) -> Vec<String> {
        if let Ok(mut engine) = domain.ignore_engine.write() {
            engine.ignore_list.clear();
            engine.load_vcs_ignores(root_path)
        } else {
            Vec::new()
        }
    }
}
