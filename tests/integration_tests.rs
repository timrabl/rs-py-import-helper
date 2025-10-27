//! Integration tests for py-import-helper
//!
//! This test suite validates the library with real-world scenarios including
//! popular Python frameworks (FastAPI, Django, Pydantic) and complex import patterns.

use py_import_helper::{types::ImportSpec, ImportHelper};

/// Test a complete FastAPI application imports
#[test]
fn test_fastapi_app_imports() {
    let mut helper = ImportHelper::with_package_name("myapi".to_string());

    // Future imports
    helper.add_import_string("from __future__ import annotations");

    // Standard library
    helper.add_from_import("typing", &["Any", "Optional"]);
    helper.add_direct_import("json");

    // Third-party
    helper.add_from_import("fastapi", &["FastAPI", "Depends", "HTTPException"]);
    helper.add_from_import("pydantic", &["BaseModel", "Field", "validator"]);
    helper.add_from_import("sqlalchemy", &["Column", "Integer", "String"]);

    // Local
    helper.add_from_import("myapi.database", &["get_db"]);
    helper.add_from_import("myapi.models", &["User", "Post"]);

    // TYPE_CHECKING
    helper.add_type_checking_from_import("sqlalchemy.orm", &["Session"]);

    let imports = helper.get_formatted();
    let import_text = imports.join("\n");

    // Validate structure
    assert!(import_text.starts_with("from __future__ import annotations"));
    assert!(import_text.contains("from typing import"));
    assert!(import_text.contains("from fastapi import"));
    assert!(import_text.contains("from myapi."));
}

/// Test a Pydantic model file imports
#[test]
fn test_pydantic_model_imports() {
    let mut helper = ImportHelper::with_package_name("myapp".to_string());

    helper.add_from_import("datetime", &["datetime", "date"]);
    helper.add_from_import("decimal", &["Decimal"]);
    helper.add_from_import("uuid", &["UUID"]);
    helper.add_from_import("pydantic", &["BaseModel", "Field", "ConfigDict"]);

    // TYPE_CHECKING imports for forward references
    helper.add_type_checking_from_import("myapp.users", &["User"]);

    let (_, stdlib, third_party, _) = helper.get_categorized();

    assert!(stdlib.len() >= 3);
    assert!(!third_party.is_empty());
    assert!(!helper.is_type_checking_empty());
}

/// Test Django application imports
#[test]
fn test_django_app_imports() {
    let mut helper = ImportHelper::with_package_name("myproject".to_string());

    helper.add_from_import("django.db", &["models"]);
    helper.add_from_import("django.contrib.auth.models", &["AbstractUser"]);
    helper.add_from_import("django.utils.translation", &["gettext_lazy"]);
    helper.add_from_import("myproject.utils", &["generate_slug"]);

    let (_, _, third_party, local) = helper.get_categorized();

    assert!(third_party.iter().any(|s| s.contains("django")));
    assert!(local.iter().any(|s| s.contains("myproject")));
}

/// Test data science imports (pandas, numpy)
#[test]
fn test_data_science_imports() {
    let mut helper = ImportHelper::new();

    helper.add_direct_import("pandas");
    helper.add_direct_import("numpy");
    helper.add_from_import("typing", &["Any", "Optional"]);
    helper.add_from_import("pathlib", &["Path"]);

    let (_, stdlib, third_party, _) = helper.get_categorized();

    assert!(stdlib.iter().any(|s| s.contains("typing")));
    assert!(stdlib.iter().any(|s| s.contains("pathlib")));
    assert!(third_party.iter().any(|s| s.contains("pandas")));
    assert!(third_party.iter().any(|s| s.contains("numpy")));
}

/// Test complex relative imports
#[test]
fn test_complex_relative_imports() {
    let mut helper = ImportHelper::new();

    helper.add_import_string("from . import module");
    helper.add_import_string("from .. import parent_module");
    helper.add_import_string("from ..sibling import function");
    helper.add_import_string("from ...grandparent import util");

    let (_, _, _, local) = helper.get_categorized();

    assert_eq!(local.len(), 4);
    assert!(local.iter().any(|s| s.contains("from . import")));
    assert!(local.iter().any(|s| s.contains("from .. import")));
    assert!(local.iter().any(|s| s.contains("from ..sibling import")));
    assert!(local
        .iter()
        .any(|s| s.contains("from ...grandparent import")));
}

/// Test ImportSpec builder API
#[test]
fn test_import_spec_api() {
    let mut helper = ImportHelper::new();

    // Direct import
    helper.add_import(&ImportSpec::direct("sys"));

    // From import
    helper.add_import(&ImportSpec::from("typing", vec!["Any", "Optional"]));

    // TYPE_CHECKING imports
    helper.add_import(&ImportSpec::type_checking_from("httpx", vec!["Client"]));

    let (_, stdlib, _, _) = helper.get_categorized();
    let (_, _, tc_third_party, _) = helper.get_type_checking_categorized();

    assert!(stdlib.iter().any(|s| s.contains("import sys")), "Should contain 'import sys', got: {:?}", stdlib);
    assert!(stdlib.iter().any(|s| s.contains("from typing import")));
    assert!(tc_third_party
        .iter()
        .any(|s| s.contains("from httpx import Client")));
}

/// Test handling duplicate imports
#[test]
fn test_duplicate_imports() {
    let mut helper = ImportHelper::new();

    // Add same import multiple times
    helper.add_import_string("from typing import Any");
    helper.add_import_string("from typing import Any");
    helper.add_import_string("from typing import Any");

    let (_, stdlib, _, _) = helper.get_categorized();

    // Should only appear once after merging
    let typing_imports: Vec<_> = stdlib
        .iter()
        .filter(|s| s.contains("from typing import"))
        .collect();

    assert_eq!(typing_imports.len(), 1);
}

/// Test mixed direct and from imports from same module
#[test]
fn test_mixed_import_types() {
    let mut helper = ImportHelper::new();

    helper.add_direct_import("json");
    helper.add_from_import("json", &["loads", "dumps"]);

    let (_, stdlib, _, _) = helper.get_categorized();

    // Should have both direct and from imports
    assert!(stdlib.iter().any(|s| s.contains("import json")));
    assert!(stdlib.iter().any(|s| s.contains("from json import")));
}

/// Test empty helper
#[test]
fn test_empty_helper() {
    let helper = ImportHelper::new();

    assert!(helper.is_empty());
    assert!(helper.is_type_checking_empty());
    assert_eq!(helper.count(), 0);
    assert_eq!(helper.count_type_checking(), 0);

    let (_future, stdlib, third_party, local) = helper.get_categorized();
    assert!(stdlib.is_empty());
    assert!(third_party.is_empty());
    assert!(local.is_empty());
}

/// Test helper reset functionality
#[test]
fn test_helper_reset() {
    let mut helper = ImportHelper::with_package_name("myapp".to_string());

    helper.add_import_string("from typing import Any");
    helper.add_import_string("from myapp.models import User");

    assert_eq!(helper.count(), 2);

    // Use clear() to preserve package name configuration
    helper.clear();

    assert!(helper.is_empty());
    assert_eq!(helper.count(), 0);

    // Package name should still be configured after clear()
    helper.add_import_string("from myapp.utils import helper");
    let (_, _, _, local) = helper.get_categorized();

    assert!(!local.is_empty(), "Local imports should be recognized after clear()");
}

/// Test helper clone_config
#[test]
fn test_helper_clone_config() {
    let base = ImportHelper::with_package_name("myapp".to_string());

    let mut helper1 = base.clone_config();
    let mut helper2 = base.clone_config();

    helper1.add_import_string("from typing import Any");
    helper2.add_import_string("from pydantic import BaseModel");

    assert_eq!(helper1.count(), 1);
    assert_eq!(helper2.count(), 1);

    // Both should recognize myapp as local
    helper1.add_import_string("from myapp.models import User");
    helper2.add_import_string("from myapp.utils import format");

    let (_, _, _, local1) = helper1.get_categorized();
    let (_, _, _, local2) = helper2.get_categorized();

    assert!(!local1.is_empty());
    assert!(!local2.is_empty());
}

/// Test local package prefix configuration
#[test]
fn test_local_package_prefix() {
    let mut helper = ImportHelper::new();

    helper.add_local_package_prefix("myapp");
    helper.add_local_package_prefix("myapp_utils");

    helper.add_import_string("from myapp.models import User");
    helper.add_import_string("from myapp_utils.helpers import format_date");
    helper.add_import_string("from pydantic import BaseModel");

    let (_, _, third_party, local) = helper.get_categorized();

    assert_eq!(local.len(), 2);
    assert_eq!(third_party.len(), 1);
}

/// Test get_all_categorized returns 8-tuple
#[test]
fn test_generate_all_imports() {
    let mut helper = ImportHelper::with_package_name("myapp".to_string());

    helper.add_import_string("from __future__ import annotations");
    helper.add_import_string("from typing import Any");
    helper.add_import_string("from pydantic import BaseModel");
    helper.add_import_string("from myapp.models import User");

    helper.add_type_checking_from_import("httpx", &["Client"]);

    let (future, stdlib, third_party, local, tc_future, tc_stdlib, tc_third_party, tc_local) =
        helper.get_all_categorized();

    assert!(!future.is_empty());
    assert!(!stdlib.is_empty());
    assert!(!third_party.is_empty());
    assert!(!local.is_empty());
    assert!(tc_future.is_empty());
    assert!(tc_stdlib.is_empty());
    assert!(!tc_third_party.is_empty());
    assert!(tc_local.is_empty());
}

/// Test real Python 3.13 script imports
#[test]
fn test_real_python313_script() {
    let mut helper = ImportHelper::with_package_name("myproject".to_string());

    // Typical Python 3.13 script
    helper.add_import_string("from __future__ import annotations");
    helper.add_from_import("typing", &["Self", "Never", "TypeVar"]);
    helper.add_from_import("collections.abc", &["Callable", "Iterable", "Mapping"]);
    helper.add_from_import("dataclasses", &["dataclass", "field"]);
    helper.add_from_import("pathlib", &["Path"]);
    helper.add_from_import("datetime", &["datetime", "timedelta"]);

    let (_, stdlib, third_party, _) = helper.get_categorized();

    assert!(stdlib.len() >= 5);
    assert!(third_party.is_empty());

    // Check ordering
    let imports = helper.get_formatted();
    assert!(imports[0].contains("from __future__ import"));
}

/// Test async imports
#[test]
fn test_async_imports() {
    let mut helper = ImportHelper::new();

    helper.add_direct_import("asyncio");
    helper.add_from_import("typing", &["Coroutine", "Awaitable"]);
    helper.add_from_import("collections.abc", &["AsyncIterator"]);

    let (_, stdlib, _, _) = helper.get_categorized();

    assert!(stdlib.iter().any(|s| s.contains("asyncio")));
    assert!(stdlib.iter().any(|s| s.contains("Coroutine")));
    assert!(stdlib.iter().any(|s| s.contains("AsyncIterator")));
}

/// Test pytest fixture imports
#[test]
fn test_pytest_imports() {
    let mut helper = ImportHelper::with_package_name("tests".to_string());

    helper.add_direct_import("pytest");
    helper.add_from_import("pytest", &["fixture", "mark"]);
    helper.add_from_import("tests.conftest", &["app", "db"]);

    let (_, _, third_party, local) = helper.get_categorized();

    assert!(third_party.iter().any(|s| s.contains("pytest")));
    assert!(local.iter().any(|s| s.contains("tests.conftest")));
}

/// Test comprehensive TYPE_CHECKING scenario
#[test]
fn test_comprehensive_type_checking() {
    let mut helper = ImportHelper::with_package_name("myapp".to_string());

    // Regular runtime imports
    helper.add_from_import("pydantic", &["BaseModel"]);
    helper.add_from_import("typing", &["Any"]);

    // TYPE_CHECKING imports to avoid circular dependencies
    helper.add_type_checking_from_import("myapp.users", &["User"]);
    helper.add_type_checking_from_import("myapp.posts", &["Post"]);
    helper.add_type_checking_from_import("collections.abc", &["Callable"]);

    let (_, stdlib, _, _) = helper.get_categorized();
    let (_, tc_stdlib, _, tc_local) = helper.get_type_checking_categorized();

    // TYPE_CHECKING should be automatically added
    assert!(stdlib.iter().any(|s| s.contains("TYPE_CHECKING")));

    // Check TYPE_CHECKING imports
    assert!(tc_stdlib.iter().any(|s| s.contains("Callable")));
    assert_eq!(tc_local.len(), 2);
}
