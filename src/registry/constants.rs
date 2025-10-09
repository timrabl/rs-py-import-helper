//! Default package lists for registry initialization
//!
//! This module contains the default lists of Python standard library modules
//! and common third-party packages used to initialize the package registry.

/// Python standard library modules for categorization
///
/// This list includes commonly used standard library modules that are part of
/// Python's built-in distribution. Used for automatic import categorization.
pub const PYTHON_STDLIB_MODULES: &[&str] = &[
    "os",
    "sys",
    "json",
    "re",
    "datetime",
    "time",
    "collections",
    "collections.abc",
    "itertools",
    "functools",
    "operator",
    "typing",
    "pathlib",
    "logging",
    "uuid",
    "hashlib",
    "base64",
    "urllib",
    "http",
    "email",
    "html",
    "xml",
    "sqlite3",
    "csv",
    "io",
    "tempfile",
    "shutil",
    "glob",
    "fnmatch",
    "linecache",
    "pickle",
    "copy",
    "math",
    "random",
    "statistics",
    "decimal",
    "fractions",
    "contextlib",
    "abc",
    "atexit",
    "traceback",
    "gc",
    "weakref",
    "enum",
    "dataclasses",
    "concurrent",
    "asyncio",
    "threading",
    "multiprocessing",
    "subprocess",
    "socket",
    "select",
    "ssl",
    "ipaddress",
    "argparse",
    "configparser",
    "getpass",
    "locale",
    "platform",
    "sysconfig",
    "types",
    "warnings",
];

/// Common third-party packages that might be recognized
///
/// This is a reference list of commonly used third-party packages.
/// Users should register their specific third-party packages using
/// `add_third_party_package()` or `add_third_party_packages()`.
pub const COMMON_THIRD_PARTY_PACKAGES: &[&str] = &[
    "pydantic",
    "httpx",
    "requests",
    "fastapi",
    "flask",
    "django",
    "numpy",
    "pandas",
    "pytest",
    "sqlalchemy",
];
