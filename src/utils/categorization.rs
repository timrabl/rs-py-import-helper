//! Import categorization utilities
//!
//! This module provides functions for categorizing Python imports into
//! future, standard library, third-party, and local categories according to PEP 8.

use super::parsing::extract_package;
use crate::registry::constants::{COMMON_THIRD_PARTY_PACKAGES, PYTHON_STDLIB_MODULES};
use crate::types::ImportCategory;
use std::collections::HashSet;

/// Categorize an import statement
#[must_use]
pub fn categorize_import<S: ::std::hash::BuildHasher>(
    import_statement: &str,
    local_package_prefixes: &HashSet<String, S>,
) -> ImportCategory {
    // Future imports always come first
    if import_statement.starts_with("from __future__") {
        return ImportCategory::Future;
    }

    let package = extract_package(import_statement);

    // Determine category with priority order:
    // 1. Local imports (relative or matching local prefixes)
    // 2. Standard library (built-in or custom registered)
    // 3. Third-party (custom registered or default)
    if is_local_import(import_statement, local_package_prefixes) {
        ImportCategory::Local
    } else if is_standard_library_package(&package) {
        ImportCategory::StandardLibrary
    } else if is_common_third_party_package(&package) {
        ImportCategory::ThirdParty
    } else {
        // Default to third-party for unknown packages
        ImportCategory::ThirdParty
    }
}

/// Check if this is a local/relative import
#[must_use]
pub fn is_local_import<S: ::std::hash::BuildHasher>(
    import_statement: &str,
    local_package_prefixes: &HashSet<String, S>,
) -> bool {
    // Check for relative imports
    if import_statement.contains("from .")
        || import_statement.contains("from ..")
        || import_statement.contains("from ...")
        || import_statement.contains("from ....")
    {
        return true;
    }

    let package = extract_package(import_statement);

    // Check custom local package prefixes
    local_package_prefixes
        .iter()
        .any(|prefix| matches_local_prefix(&package, prefix))
}

/// Whether `package` belongs to the local `prefix`, on module-path boundaries.
///
/// `myproject` matches `myproject` and `myproject.core`, but not `myproject_utils`
/// or `mypyfoo`. Prevents a short prefix from claiming unrelated packages (#16).
#[must_use]
pub(crate) fn matches_local_prefix(package: &str, prefix: &str) -> bool {
    package == prefix
        || package
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('.'))
}

/// Check if a package is part of Python's standard library.
///
/// Matches the exact name or, for a dotted path, its root segment
/// (`urllib.parse` -> `urllib`). Uses the built-in default list; for
/// registry-aware categorization that reflects runtime customization, use
/// [`ImportHelper`](crate::ImportHelper).
#[must_use]
pub fn is_standard_library_package(package: &str) -> bool {
    PYTHON_STDLIB_MODULES.contains(&package)
        || root_segment(package).is_some_and(|root| PYTHON_STDLIB_MODULES.contains(&root))
}

/// Check if a package is a common third-party package (built-in default list).
#[must_use]
pub fn is_common_third_party_package(package: &str) -> bool {
    COMMON_THIRD_PARTY_PACKAGES.contains(&package)
        || root_segment(package).is_some_and(|root| COMMON_THIRD_PARTY_PACKAGES.contains(&root))
}

/// Root (first dotted segment) of a package path, if it has one.
fn root_segment(package: &str) -> Option<&str> {
    package.split_once('.').map(|(root, _)| root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_future_import() {
        let prefixes = HashSet::new();
        let category = categorize_import("from __future__ import annotations", &prefixes);
        assert_eq!(category, ImportCategory::Future);
    }

    #[test]
    fn test_categorize_stdlib_import() {
        let prefixes = HashSet::new();
        let category = categorize_import("from typing import Any", &prefixes);
        assert_eq!(category, ImportCategory::StandardLibrary);
    }

    #[test]
    fn test_categorize_third_party_import() {
        let prefixes = HashSet::new();
        let category = categorize_import("from pydantic import BaseModel", &prefixes);
        assert_eq!(category, ImportCategory::ThirdParty);
    }

    #[test]
    fn test_categorize_local_import() {
        let mut prefixes = HashSet::new();
        prefixes.insert("myapp".to_string());

        let category = categorize_import("from myapp.models import User", &prefixes);
        assert_eq!(category, ImportCategory::Local);

        let category = categorize_import("from .utils import helper", &prefixes);
        assert_eq!(category, ImportCategory::Local);
    }

    #[test]
    fn test_is_local_import() {
        let mut prefixes = HashSet::new();
        prefixes.insert("myapp".to_string());

        assert!(is_local_import("from . import module", &prefixes));
        assert!(is_local_import("from .. import parent", &prefixes));
        assert!(is_local_import("from myapp.core import Engine", &prefixes));
        assert!(!is_local_import("from typing import Any", &prefixes));
    }

    #[test]
    fn test_is_standard_library_package() {
        assert!(is_standard_library_package("typing"));
        assert!(is_standard_library_package("os"));
        assert!(is_standard_library_package("sys"));
        assert!(is_standard_library_package("collections.abc"));
        assert!(!is_standard_library_package("pydantic"));
    }

    #[test]
    fn test_is_common_third_party_package() {
        assert!(is_common_third_party_package("pydantic"));
        assert!(is_common_third_party_package("httpx"));
        assert!(is_common_third_party_package("pytest"));
        assert!(!is_common_third_party_package("typing"));
    }
}
