//! Type definitions for Python import management
//!
//! This module defines the core types and data structures used throughout the
//! py-import-helper library, including import categories, statements, and
//! type aliases for better API ergonomics.

/// Represents the different categories of Python imports for proper ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportCategory {
    /// Future imports (from __future__ import ...)
    Future,
    /// Python standard library imports
    StandardLibrary,
    /// Third-party package imports
    ThirdParty,
    /// Local/relative imports from the current package
    Local,
}

/// Represents the type of import statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportType {
    /// Direct import (import module)
    Direct,
    /// From import (from module import item)
    From,
}

/// Specification for adding imports in a structured way
#[derive(Debug, Clone)]
pub struct ImportSpec {
    /// The package/module name (e.g., "httpx", "typing")
    pub package: String,
    /// Optional items to import from the package (e.g., `["URL", "Client"]`)
    /// If None or empty, creates a direct import (import package)
    /// If Some(items), creates a from import (from package import items...)
    pub items: Option<Vec<String>>,
    /// Whether this import should go in `TYPE_CHECKING` block
    pub type_checking: bool,
}

impl ImportSpec {
    /// Create a direct import specification (import package)
    pub fn direct(package: impl Into<String>) -> Self {
        Self {
            package: package.into(),
            items: None,
            type_checking: false,
        }
    }

    /// Create a from import specification (from package import items...)
    pub fn from(package: impl Into<String>, items: Vec<impl Into<String>>) -> Self {
        Self {
            package: package.into(),
            items: Some(items.into_iter().map(Into::into).collect()),
            type_checking: false,
        }
    }

    /// Create a `TYPE_CHECKING` direct import
    pub fn type_checking_direct(package: impl Into<String>) -> Self {
        Self {
            package: package.into(),
            items: None,
            type_checking: true,
        }
    }

    /// Create a `TYPE_CHECKING` from import
    pub fn type_checking_from(package: impl Into<String>, items: Vec<impl Into<String>>) -> Self {
        Self {
            package: package.into(),
            items: Some(items.into_iter().map(Into::into).collect()),
            type_checking: true,
        }
    }

    /// Mark this import as `TYPE_CHECKING`
    #[must_use]
    pub const fn as_type_checking(mut self) -> Self {
        self.type_checking = true;
        self
    }
}

/// Represents a single import statement with its category and formatting information
#[derive(Debug, Clone)]
pub struct ImportStatement {
    /// The complete import statement as a string
    pub statement: String,
    /// The category this import belongs to
    pub category: ImportCategory,
    /// The type of import (direct or from)
    pub import_type: ImportType,
    /// The package/module being imported from (for organization)
    pub package: String,
    /// Individual items being imported (for merging similar imports)
    pub items: Vec<String>,
    /// Whether this is a multi-line import
    #[allow(dead_code)]
    pub is_multiline: bool,
}

/// Type alias for the return type of categorized imports methods
/// Returns (future, stdlib, `third_party`, local, `tc_future`, `tc_stdlib`, `tc_third_party`, `tc_local`)
pub type AllCategorizedImports = (
    Vec<String>, // future_imports
    Vec<String>, // stdlib_imports
    Vec<String>, // third_party_imports
    Vec<String>, // local_imports
    Vec<String>, // type_checking_future_imports
    Vec<String>, // type_checking_stdlib_imports
    Vec<String>, // type_checking_third_party_imports
    Vec<String>, // type_checking_local_imports
);

/// Type alias for the return type of regular categorized imports methods
/// Returns (future, stdlib, `third_party`, local)
pub type CategorizedImports = (
    Vec<String>, // future_imports
    Vec<String>, // stdlib_imports
    Vec<String>, // third_party_imports
    Vec<String>, // local_imports
);

/// A collection of imports organized by category and type for proper formatting
#[derive(Debug, Default)]
pub struct ImportSections {
    /// Future imports
    pub future: Vec<ImportStatement>,
    /// Standard library direct imports (import module)
    pub standard_library_direct: Vec<ImportStatement>,
    /// Standard library from imports (from module import item)
    pub standard_library_from: Vec<ImportStatement>,
    /// Third-party direct imports
    pub third_party_direct: Vec<ImportStatement>,
    /// Third-party from imports
    pub third_party_from: Vec<ImportStatement>,
    /// Local direct imports
    pub local_direct: Vec<ImportStatement>,
    /// Local from imports
    pub local_from: Vec<ImportStatement>,

    // TYPE_CHECKING block imports
    /// `TYPE_CHECKING` future imports
    pub type_checking_future: Vec<ImportStatement>,
    /// `TYPE_CHECKING` standard library direct imports
    pub type_checking_standard_library_direct: Vec<ImportStatement>,
    /// `TYPE_CHECKING` standard library from imports
    pub type_checking_standard_library_from: Vec<ImportStatement>,
    /// `TYPE_CHECKING` third-party direct imports
    pub type_checking_third_party_direct: Vec<ImportStatement>,
    /// `TYPE_CHECKING` third-party from imports
    pub type_checking_third_party_from: Vec<ImportStatement>,
    /// `TYPE_CHECKING` local direct imports
    pub type_checking_local_direct: Vec<ImportStatement>,
    /// `TYPE_CHECKING` local from imports
    pub type_checking_local_from: Vec<ImportStatement>,
}
