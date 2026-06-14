// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::app::MonitorDomain;
use crate::domain::entities::FileModification;

pub struct ProcessFileChangeUseCase;

impl Default for ProcessFileChangeUseCase {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessFileChangeUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, domain: &mut MonitorDomain, modif: FileModification) -> bool {
        domain.handle_file_changed(modif)
    }
}
