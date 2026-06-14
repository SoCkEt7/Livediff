// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use super::*;
use crate::domain::ignore_engine::IgnoreEngine;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

#[test]
fn test_monitor_domain_ignores() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(
        false,
        false,
        false,
        false,
        &["target/".to_string(), "*.tmp".to_string()],
    )));
    let domain = MonitorDomain::new(engine);

    assert!(domain.is_ignored("target/debug/build"));
    assert!(domain.is_ignored("src/main.tmp"));
    assert!(!domain.is_ignored("src/main.rs"));
}

#[test]
fn test_monitor_domain_history_limit() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);
    for i in 0..60 {
        let modif = FileModification {
            path: format!("file_{}.rs", i),
            timestamp: SystemTime::now(),
            size: 100,
            added: 1,
            deleted: 0,
            diff_lines: vec![],
            is_binary: false,
        };
        domain.handle_file_changed(modif);
    }
    // History limit is 50, so older modifications should be popped
    assert_eq!(domain.modifications.len(), 50);
    assert_eq!(domain.modifications.front().unwrap().path, "file_59.rs");
}
