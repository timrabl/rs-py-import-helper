//! Exact tests for create_model_imports type-name matching (#18).

use py_import_helper::ImportHelper;

fn stdlib_of(h: &ImportHelper) -> Vec<String> {
    h.get_categorized().1
}

#[test]
fn unrelated_type_names_do_not_pull_typing_imports() {
    let mut h = ImportHelper::new();
    h.create_model_imports(&["Anything".to_string(), "MyProtocolMessage".to_string()]);
    let stdlib = stdlib_of(&h);
    // "Anything"/"MyProtocolMessage" must NOT import Any/Protocol.
    assert!(
        !stdlib.iter().any(|s| s.contains("from typing import")),
        "spurious typing import: {stdlib:?}"
    );
}

#[test]
fn real_typing_tokens_are_detected() {
    let mut h = ImportHelper::new();
    h.create_model_imports(&["dict[str, Any]".to_string(), "Protocol".to_string()]);
    let stdlib = stdlib_of(&h);
    assert!(
        stdlib
            .iter()
            .any(|s| s == "from typing import Any, Protocol"),
        "got: {stdlib:?}"
    );
}

#[test]
fn callable_token_boundary() {
    let mut h = ImportHelper::new();
    h.create_model_imports(&["MyCallableThing".to_string()]);
    let stdlib = stdlib_of(&h);
    assert!(
        !stdlib.iter().any(|s| s.contains("collections.abc")),
        "spurious Callable import: {stdlib:?}"
    );

    let mut h2 = ImportHelper::new();
    h2.create_model_imports(&["Callable[[int], str]".to_string()]);
    let stdlib2 = stdlib_of(&h2);
    assert!(
        stdlib2
            .iter()
            .any(|s| s == "from collections.abc import Callable"),
        "got: {stdlib2:?}"
    );
}
