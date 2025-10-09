//! Python import organization library
//!
//! A Rust library for automatically organizing Python imports according to PEP 8.
//! Perfect for code generation projects, it categorizes imports into standard library,
//! third-party, and local packages, then formats them with proper spacing and ordering.
//!
//! # Usage
//!
//! ```rust
//! use py_import_helper::ImportHelper;
//!
//! // Create a helper with package name for local import detection
//! let mut helper = ImportHelper::with_package_name("mypackage".to_string());
//!
//! // Add custom local package prefixes
//! helper.add_local_package_prefix("mypackage");
//!
//! // Collect imports using strings
//! helper.add_import_string("from typing import Any");
//! helper.add_import_string("from pydantic import BaseModel");
//! helper.add_import_string("from mypackage.models import User");
//!
//! // Or use the builder pattern
//! helper.add_from_import("typing", &["Optional", "List"]);
//! helper.add_direct_import("json");
//!
//! // Get categorized imports
//! let (future, stdlib, third_party, local) = helper.get_categorized();
//! ```

// Modules
mod core;
pub mod registry;
pub mod types;

// Utility modules (public for advanced usage)
pub mod utils;

// Re-export the main ImportHelper and key types
pub use core::ImportHelper;
pub use registry::PackageRegistry;

// Re-export types that might be needed for advanced usage
#[allow(unused_imports)]
pub use types::{ImportCategory, ImportSections, ImportStatement, ImportType};

// Re-export constants for external use
#[allow(unused_imports)]
pub use registry::constants::{COMMON_THIRD_PARTY_PACKAGES, PYTHON_STDLIB_MODULES};
