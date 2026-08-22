//! Categorization correctness tests (registry coverage, dotted submodules,
//! local-prefix boundaries, cache invalidation) with exact assertions.

use py_import_helper::{ImportHelper, PackageRegistry};

// --- #15: stdlib coverage + dotted submodule fallback -----------------------

#[test]
fn previously_missing_stdlib_modules_are_stdlib() {
    let reg = PackageRegistry::new();
    for m in [
        "secrets", "string", "inspect", "unittest", "struct", "zlib", "tomllib", "zoneinfo",
    ] {
        assert!(reg.is_stdlib(m), "{m} should be stdlib");
    }
    assert!(reg.count_stdlib_packages() > 200);
}

#[test]
fn dotted_submodules_fall_back_to_root() {
    let reg = PackageRegistry::new();
    assert!(reg.is_stdlib("urllib.parse"));
    assert!(reg.is_stdlib("os.path"));
    assert!(reg.is_stdlib("collections.abc"));
    assert!(!reg.is_stdlib("pydantic.fields"));
}

#[test]
fn dotted_stdlib_import_categorizes_as_stdlib() {
    let mut h = ImportHelper::new();
    h.add_import_string("from urllib.parse import urlparse");
    let (_f, stdlib, third, _l) = h.get_categorized();
    assert_eq!(stdlib, vec!["from urllib.parse import urlparse"]);
    assert!(third.is_empty());
}
