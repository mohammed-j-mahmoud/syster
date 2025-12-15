// TDD: Tests for lazy stdlib loading (load on first reference)

#![allow(clippy::unwrap_used)]

use super::StdLibLoader;
use crate::semantic::Workspace;

#[test]
fn test_lazy_loader_does_not_load_immediately() {
    // TDD: Lazy loader should not load files until requested
    let _loader = StdLibLoader::lazy();
    let workspace = Workspace::new();

    // Should not have stdlib loaded yet
    assert!(!workspace.has_stdlib(), "Stdlib should not be loaded yet");
    assert_eq!(
        workspace.file_count(),
        0,
        "Should have no files before lazy load"
    );
}

#[test]
fn test_lazy_load_on_demand() {
    // TDD: First request should trigger loading
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::new();

    assert!(!workspace.has_stdlib(), "Should not be loaded initially");

    // First call to ensure_loaded should load the stdlib
    loader.ensure_loaded(&mut workspace).unwrap();

    assert!(
        workspace.has_stdlib(),
        "Should be loaded after ensure_loaded"
    );
    assert!(
        workspace.file_count() > 0,
        "Should have stdlib files loaded"
    );
}

#[test]
fn test_lazy_load_only_once() {
    // TDD: Subsequent requests should not reload
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::new();

    // First load
    loader.ensure_loaded(&mut workspace).unwrap();
    let first_count = workspace.file_count();

    // Second request - should not reload (file count stays the same)
    loader.ensure_loaded(&mut workspace).unwrap();
    assert_eq!(
        workspace.file_count(),
        first_count,
        "Should not reload on second ensure_loaded call"
    );
}

#[test]
fn test_can_check_if_stdlib_loaded() {
    // TDD: Should be able to query if stdlib is loaded
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::new();

    assert!(!workspace.has_stdlib(), "Initially not loaded");
    assert!(!loader.is_loaded(), "Loader should report not loaded");

    // After loading
    loader.ensure_loaded(&mut workspace).unwrap();
    assert!(
        workspace.has_stdlib(),
        "Should be loaded after ensure_loaded"
    );
    assert!(loader.is_loaded(), "Loader should report loaded");
}

#[test]
fn test_eager_load_still_works() {
    // TDD: Original eager loading should still work
    let loader = StdLibLoader::new();
    let mut workspace = Workspace::new();

    // Eager load immediately loads
    loader.load(&mut workspace).unwrap();

    assert!(
        workspace.has_stdlib(),
        "Should be loaded after eager load()"
    );
    assert!(
        workspace.file_count() > 0,
        "Should have stdlib files loaded"
    );
}

#[test]
fn test_lazy_avoids_reloading() {
    // TDD: Lazy loader should not reload if stdlib already in workspace
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::new();

    // Manually mark stdlib as loaded (simulate pre-loaded state)
    workspace.mark_stdlib_loaded();

    // ensure_loaded should respect existing stdlib
    loader.ensure_loaded(&mut workspace).unwrap();

    // Should not have added files since stdlib was already marked
    assert!(workspace.has_stdlib());
}
