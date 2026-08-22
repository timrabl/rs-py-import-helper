//! Exact-output golden tests.
//!
//! Every assertion here is `assert_eq!` on the complete formatted output, not a
//! substring check. Substring assertions are what let the core formatting bugs
//! (#12 `import json` -> `from json import json`, #13 alias mangling, #14
//! multi-line scrambling) pass a green suite: `"from sys import sys"` contains
//! `"import sys"`. These tests pin the exact strings so a regression cannot hide.

use py_import_helper::{FormattingConfig, ImportHelper};

// --- #12: plain direct imports must round-trip -----------------------------

#[test]
fn direct_import_round_trips() {
    let mut h = ImportHelper::new();
    h.add_import_string("import json");
    assert_eq!(h.get_formatted(), vec!["import json"]);
}

#[test]
fn direct_import_with_alias_round_trips() {
    let mut h = ImportHelper::new();
    h.add_import_string("import numpy as np");
    assert_eq!(h.get_formatted(), vec!["import numpy as np"]);
}

#[test]
fn multiple_direct_imports_each_round_trip() {
    let mut h = ImportHelper::new();
    h.add_direct_import("json");
    h.add_direct_import("sys");
    // Both are stdlib direct imports; each stays a plain `import`.
    assert_eq!(h.get_formatted(), vec!["import json", "import sys"]);
}

#[test]
fn direct_and_from_of_same_package_not_merged() {
    let mut h = ImportHelper::new();
    h.add_direct_import("json");
    h.add_from_import("json", &["dumps", "loads"]);
    // Direct stays direct, from stays from; they are never merged into one.
    assert_eq!(
        h.get_formatted(),
        vec!["import json", "from json import dumps, loads"]
    );
}

// --- #13: aliased from-imports stay valid Python ----------------------------

#[test]
fn from_import_alias_preserved() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Any as A");
    assert_eq!(h.get_formatted(), vec!["from typing import Any as A"]);
}

#[test]
fn from_import_alias_sorts_on_original_name() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Optional as Opt, Any as A");
    // Sorted on the original bound name (Any before Optional), aliases intact.
    assert_eq!(
        h.get_formatted(),
        vec!["from typing import Any as A, Optional as Opt"]
    );
}

// --- #14: multi-line blocks stay contiguous ---------------------------------

#[test]
fn multiline_block_not_scrambled_in_categorized() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Any, Optional, Union, Literal, Protocol, TypeVar");
    let (_future, stdlib, _third, _local) = h.get_categorized();
    assert_eq!(
        stdlib,
        vec![
            "from typing import (",
            "    Any,",
            "    Literal,",
            "    Optional,",
            "    Protocol,",
            "    TypeVar,",
            "    Union,",
            ")",
        ]
    );
}

#[test]
fn multiline_block_contiguous_with_other_statements() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Any, Optional, Union, Literal, Protocol, TypeVar");
    h.add_import_string("import os");
    let (_future, stdlib, _third, _local) = h.get_categorized();
    // Direct import first, then the multi-line from-block, unscrambled.
    assert_eq!(
        stdlib,
        vec![
            "import os",
            "from typing import (",
            "    Any,",
            "    Literal,",
            "    Optional,",
            "    Protocol,",
            "    TypeVar,",
            "    Union,",
            ")",
        ]
    );
}

// --- correct behaviours locked so fixes don't regress them ------------------

#[test]
fn same_package_from_imports_merge_sorted() {
    let mut h = ImportHelper::new();
    h.add_from_import("pathlib", &["Path"]);
    h.add_from_import("pathlib", &["PurePath"]);
    assert_eq!(
        h.get_formatted(),
        vec!["from pathlib import Path, PurePath"]
    );
}

#[test]
fn single_line_stays_single_below_threshold() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Any, Optional");
    assert_eq!(h.get_formatted(), vec!["from typing import Any, Optional"]);
}

#[test]
fn all_caps_sorts_before_mixed_case() {
    let mut h = ImportHelper::new();
    h.add_import_string("from typing import Any, TYPE_CHECKING, Optional");
    assert_eq!(
        h.get_formatted(),
        vec!["from typing import TYPE_CHECKING, Any, Optional"]
    );
}

#[test]
fn force_multiline_config_wraps_single_item() {
    let cfg = FormattingConfig {
        force_multiline: true,
        ..Default::default()
    };
    let mut h = ImportHelper::with_formatting_config(cfg);
    h.add_import_string("from typing import Any");
    assert_eq!(
        h.get_formatted(),
        vec!["from typing import (", "    Any,", ")"]
    );
}
