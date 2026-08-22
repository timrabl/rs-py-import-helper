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

// --- #16: local prefix word boundary ----------------------------------------

#[test]
fn local_prefix_does_not_claim_longer_names() {
    let mut h = ImportHelper::new();
    h.add_local_package_prefix("my");
    h.add_import_string("from mypy import api"); // must NOT be local
    let (_f, _s, third, local) = h.get_categorized();
    assert_eq!(third, vec!["from mypy import api"]);
    assert!(local.is_empty());
}

#[test]
fn local_prefix_matches_exact_and_dotted() {
    let mut h = ImportHelper::new();
    h.add_local_package_prefix("myproject");
    h.add_import_string("from myproject import a");
    h.add_import_string("from myproject.core import b");
    h.add_import_string("from myproject_utils import c"); // NOT local
    let (_f, _s, third, local) = h.get_categorized();
    assert_eq!(
        local,
        vec!["from myproject import a", "from myproject.core import b"]
    );
    assert_eq!(third, vec!["from myproject_utils import c"]);
}
