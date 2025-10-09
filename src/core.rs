//! Core import helper functionality
//!
//! This module contains the main `ImportHelper` struct and its implementation,
//! providing the primary API for collecting, categorizing, and formatting Python
//! imports according to PEP 8 and common Python formatting standards.

use std::collections::{HashMap, HashSet};

use crate::registry::PackageRegistry;
use crate::types::{AllCategorizedImports, CategorizedImports, ImportSpec};
use crate::{ImportCategory, ImportSections, ImportStatement, ImportType};

/// Main helper for managing Python imports across the codebase
#[derive(Debug)]
pub struct ImportHelper {
    /// Collected imports organized by category
    sections: ImportSections,
    /// Cache for import categorization
    category_cache: HashMap<String, ImportCategory>,
    /// The package name for identifying local imports
    package_name: Option<String>,
    /// Custom local package prefixes to recognize
    local_package_prefixes: HashSet<String>,
    /// Package registry for stdlib and third-party recognition
    registry: PackageRegistry,
}

impl ImportHelper {
    /// Create a new import helper instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            sections: ImportSections::default(),
            category_cache: HashMap::new(),
            package_name: None,
            local_package_prefixes: HashSet::new(),
            registry: PackageRegistry::new(),
        }
    }

    /// Create a new import helper instance with package name for local import detection
    #[must_use]
    pub fn with_package_name(package_name: String) -> Self {
        let mut helper = Self::new();
        helper.package_name = Some(package_name.clone());
        helper.local_package_prefixes.insert(package_name);
        helper
    }

    /// Get immutable reference to the package registry
    ///
    /// # Examples
    ///
    /// ```
    /// use py_import_helper::ImportHelper;
    ///
    /// let helper = ImportHelper::new();
    /// assert!(helper.registry().is_stdlib("typing"));
    /// ```
    #[must_use]
    pub fn registry(&self) -> &PackageRegistry {
        &self.registry
    }

    /// Get mutable reference to the package registry
    ///
    /// Use this to customize which packages are recognized as stdlib or third-party
    /// before generating imports.
    ///
    /// # Examples
    ///
    /// ```
    /// use py_import_helper::ImportHelper;
    ///
    /// let mut helper = ImportHelper::new();
    /// helper.registry_mut()
    ///     .add_stdlib_package("my_custom_stdlib")
    ///     .add_third_party_package("my_company_lib");
    ///
    /// helper.add_import_string("import my_custom_stdlib");
    /// let (_future, stdlib, _third, _local) = helper.get_categorized();
    /// assert!(stdlib.iter().any(|s| s.contains("my_custom_stdlib")));
    /// ```
    pub fn registry_mut(&mut self) -> &mut PackageRegistry {
        // Clear cache when registry is modified
        &mut self.registry
    }

    /// Clear the categorization cache
    ///
    /// Call this after modifying the registry to ensure changes take effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use py_import_helper::ImportHelper;
    ///
    /// let mut helper = ImportHelper::new();
    /// helper.add_import_string("import mypackage");
    /// helper.registry_mut().add_stdlib_package("mypackage");
    /// helper.clear_cache();  // Force re-categorization
    /// ```
    pub fn clear_cache(&mut self) -> &mut Self {
        self.category_cache.clear();
        self
    }

    /// Add a custom local package prefix to the recognition list
    pub fn add_local_package_prefix(&mut self, prefix: impl Into<String>) -> &mut Self {
        let prefix = prefix.into();
        self.local_package_prefixes.insert(prefix);
        // Don't add to cache - these are prefixes, not exact matches
        self
    }

    /// Add multiple local package prefixes at once
    pub fn add_local_package_prefixes(&mut self, prefixes: &[impl AsRef<str>]) -> &mut Self {
        for prefix in prefixes {
            self.add_local_package_prefix(prefix.as_ref());
        }
        self
    }

    /// Add an import using structured `ImportSpec`
    pub fn add_import(&mut self, spec: &ImportSpec) {
        let import_statement = if let Some(items) = &spec.items {
            format!("from {} import {}", spec.package, items.join(", "))
        } else {
            format!("import {}", spec.package)
        };

        if spec.type_checking {
            self.add_type_checking_import(&import_statement);
        } else {
            self.add_regular_import(&import_statement);
        }
    }

    /// Convenience method to add import from string (for backward compatibility)
    pub fn add_import_string(&mut self, import_statement: &str) {
        self.add_regular_import(import_statement);
    }

    /// Add an import statement using string (internal method)
    fn add_regular_import(&mut self, import_statement: &str) {
        if let Some(import) = self.parse_import(import_statement) {
            match (&import.category, &import.import_type) {
                (ImportCategory::Future, _) => self.sections.future.push(import),
                (ImportCategory::StandardLibrary, ImportType::Direct) => {
                    self.sections.standard_library_direct.push(import)
                }
                (ImportCategory::StandardLibrary, ImportType::From) => {
                    self.sections.standard_library_from.push(import)
                }
                (ImportCategory::ThirdParty, ImportType::Direct) => {
                    self.sections.third_party_direct.push(import)
                }
                (ImportCategory::ThirdParty, ImportType::From) => {
                    self.sections.third_party_from.push(import)
                }
                (ImportCategory::Local, ImportType::Direct) => {
                    self.sections.local_direct.push(import)
                }
                (ImportCategory::Local, ImportType::From) => self.sections.local_from.push(import),
            }
        }
    }

    /// Add a from import statement programmatically
    /// Example: `add_from_import("typing", &["Any", "Optional"])`
    pub fn add_from_import(&mut self, package: &str, items: &[&str]) {
        let import_statement = if items.len() == 1 {
            format!("from {} import {}", package, items[0])
        } else {
            format!("from {} import {}", package, items.join(", "))
        };
        self.add_regular_import(&import_statement);
    }

    /// Add a from import statement to `TYPE_CHECKING` block programmatically
    /// Example: `add_type_checking_from_import("httpx", &["Client", "Response"])`
    pub fn add_type_checking_from_import(&mut self, package: &str, items: &[&str]) {
        let import_statement = if items.len() == 1 {
            format!("from {} import {}", package, items[0])
        } else {
            format!("from {} import {}", package, items.join(", "))
        };
        self.add_type_checking_import(&import_statement);
    }

    /// Add a direct import statement programmatically
    /// Example: `add_direct_import("json`")
    pub fn add_direct_import(&mut self, module: &str) {
        let import_statement = format!("import {module}");
        self.add_regular_import(&import_statement);
    }

    /// Add a direct import statement to `TYPE_CHECKING` block programmatically
    /// Example: `add_type_checking_direct_import("httpx`")
    pub fn add_type_checking_direct_import(&mut self, module: &str) {
        let import_statement = format!("import {module}");
        self.add_type_checking_import(&import_statement);
    }

    /// Add an import statement to the `TYPE_CHECKING` block
    pub fn add_type_checking_import(&mut self, import_statement: &str) {
        if let Some(import) = self.parse_import(import_statement) {
            match (&import.category, &import.import_type) {
                (ImportCategory::Future, _) => self.sections.type_checking_future.push(import),
                (ImportCategory::StandardLibrary, ImportType::Direct) => self
                    .sections
                    .type_checking_standard_library_direct
                    .push(import),
                (ImportCategory::StandardLibrary, ImportType::From) => self
                    .sections
                    .type_checking_standard_library_from
                    .push(import),
                (ImportCategory::ThirdParty, ImportType::Direct) => {
                    self.sections.type_checking_third_party_direct.push(import);
                }
                (ImportCategory::ThirdParty, ImportType::From) => {
                    self.sections.type_checking_third_party_from.push(import);
                }
                (ImportCategory::Local, ImportType::Direct) => {
                    self.sections.type_checking_local_direct.push(import);
                }
                (ImportCategory::Local, ImportType::From) => {
                    self.sections.type_checking_local_from.push(import);
                }
            }

            // Automatically add TYPE_CHECKING to typing import when we have type checking imports
            self.ensure_type_checking_import_added();
        }
    }

    /// Generate all imports (regular + `TYPE_CHECKING`) for templates
    /// Returns a tuple with 8 vectors:
    /// (future, stdlib, `third_party`, local, `tc_future`, `tc_stdlib`, `tc_third_party`, `tc_local`)
    #[must_use]
    pub fn get_all_categorized(&self) -> AllCategorizedImports {
        // Get regular imports (now includes future)
        let (future_imports, stdlib_imports, third_party_imports, local_imports) =
            self.get_categorized();

        // Get TYPE_CHECKING imports
        let (tc_future, tc_stdlib, tc_third_party, tc_local) = self.get_type_checking_categorized();

        (
            future_imports,
            stdlib_imports,
            third_party_imports,
            local_imports,
            tc_future,
            tc_stdlib,
            tc_third_party,
            tc_local,
        )
    }

    /// Generate categorized `TYPE_CHECKING` imports for templates
    /// Returns (`future_imports`, `stdlib_imports`, `third_party_imports`, `local_imports`)
    /// Get `TYPE_CHECKING` imports categorized by type
    ///
    /// Returns a tuple of (`future_imports`, `stdlib_imports`, `third_party_imports`, `local_imports`)
    /// for imports that should go in the `TYPE_CHECKING` block.
    #[must_use]
    pub fn get_type_checking_categorized(&self) -> CategorizedImports {
        self.get_type_checking_categorized_impl()
    }

    #[must_use]
    pub fn get_type_checking_categorized_impl(&self) -> CategorizedImports {
        let mut future_imports = Vec::new();
        let mut stdlib_imports = Vec::new();
        let mut third_party_imports = Vec::new();
        let mut local_imports = Vec::new();

        // Future imports
        if !self.sections.type_checking_future.is_empty() {
            let future = self.format_imports(&self.sections.type_checking_future);
            future_imports.extend(future);
        }

        // Standard library imports - direct first, then from
        if !self
            .sections
            .type_checking_standard_library_direct
            .is_empty()
        {
            let std_direct =
                self.format_imports(&self.sections.type_checking_standard_library_direct);
            stdlib_imports.extend(std_direct);
        }
        if !self.sections.type_checking_standard_library_from.is_empty() {
            let std_from = self.format_imports(&self.sections.type_checking_standard_library_from);
            stdlib_imports.extend(std_from);
        }

        // Third-party imports - direct first, then from
        if !self.sections.type_checking_third_party_direct.is_empty() {
            let third_direct = self.format_imports(&self.sections.type_checking_third_party_direct);
            third_party_imports.extend(third_direct);
        }
        if !self.sections.type_checking_third_party_from.is_empty() {
            let third_from = self.format_imports(&self.sections.type_checking_third_party_from);
            third_party_imports.extend(third_from);
        }

        // Local imports - direct first, then from
        if !self.sections.type_checking_local_direct.is_empty() {
            let local_direct = self.format_imports(&self.sections.type_checking_local_direct);
            local_imports.extend(local_direct);
        }
        if !self.sections.type_checking_local_from.is_empty() {
            let local_from = self.format_imports(&self.sections.type_checking_local_from);
            local_imports.extend(local_from);
        }

        // Sort each category alphabetically
        future_imports.sort();
        stdlib_imports.sort();
        third_party_imports.sort();
        local_imports.sort();

        (
            future_imports,
            stdlib_imports,
            third_party_imports,
            local_imports,
        )
    }

    /// Get the collected imports as categorized tuples
    /// Returns (`future_imports`, `stdlib_imports`, `third_party_imports`, `local_imports`)
    #[must_use]
    pub fn get_categorized(&self) -> CategorizedImports {
        let mut future_imports = Vec::new();
        let mut stdlib_imports = Vec::new();
        let mut third_party_imports = Vec::new();
        let mut local_imports = Vec::new();

        // Future imports
        if !self.sections.future.is_empty() {
            let future = self.format_imports(&self.sections.future);
            future_imports.extend(future);
        }

        // Standard library imports - direct first, then from
        if !self.sections.standard_library_direct.is_empty() {
            let std_direct_imports = self.format_imports(&self.sections.standard_library_direct);
            stdlib_imports.extend(std_direct_imports);
        }
        if !self.sections.standard_library_from.is_empty() {
            let std_from_imports = self.format_imports(&self.sections.standard_library_from);
            stdlib_imports.extend(std_from_imports);
        }

        // Third-party imports - direct first, then from
        if !self.sections.third_party_direct.is_empty() {
            let third_direct_imports = self.format_imports(&self.sections.third_party_direct);
            third_party_imports.extend(third_direct_imports);
        }
        if !self.sections.third_party_from.is_empty() {
            let third_from_imports = self.format_imports(&self.sections.third_party_from);
            third_party_imports.extend(third_from_imports);
        }

        // Local imports - direct first, then from
        if !self.sections.local_direct.is_empty() {
            let local_direct_imports = self.format_imports(&self.sections.local_direct);
            local_imports.extend(local_direct_imports);
        }
        if !self.sections.local_from.is_empty() {
            let local_from_imports = self.format_imports(&self.sections.local_from);
            local_imports.extend(local_from_imports);
        }

        // Sort each category alphabetically
        future_imports.sort();
        stdlib_imports.sort();
        third_party_imports.sort();
        local_imports.sort();

        (
            future_imports,
            stdlib_imports,
            third_party_imports,
            local_imports,
        )
    }

    /// Reset the import sections while preserving configuration
    /// Useful when reusing the same helper for multiple files
    pub fn reset(&mut self) -> &mut Self {
        self.sections = ImportSections::default();
        self
    }

    /// Check if any imports have been collected (excluding `TYPE_CHECKING` imports)
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.future.is_empty()
            && self.sections.standard_library_direct.is_empty()
            && self.sections.standard_library_from.is_empty()
            && self.sections.third_party_direct.is_empty()
            && self.sections.third_party_from.is_empty()
            && self.sections.local_direct.is_empty()
            && self.sections.local_from.is_empty()
    }

    /// Check if any `TYPE_CHECKING` imports have been collected
    #[must_use]
    pub fn is_type_checking_empty(&self) -> bool {
        self.sections.type_checking_future.is_empty()
            && self
                .sections
                .type_checking_standard_library_direct
                .is_empty()
            && self.sections.type_checking_standard_library_from.is_empty()
            && self.sections.type_checking_third_party_direct.is_empty()
            && self.sections.type_checking_third_party_from.is_empty()
            && self.sections.type_checking_local_direct.is_empty()
            && self.sections.type_checking_local_from.is_empty()
    }

    /// Count total number of import statements collected (excluding `TYPE_CHECKING` imports)
    #[must_use]
    pub fn count(&self) -> usize {
        self.sections.future.len()
            + self.sections.standard_library_direct.len()
            + self.sections.standard_library_from.len()
            + self.sections.third_party_direct.len()
            + self.sections.third_party_from.len()
            + self.sections.local_direct.len()
            + self.sections.local_from.len()
    }

    /// Count total number of `TYPE_CHECKING` import statements collected
    #[must_use]
    pub fn count_type_checking(&self) -> usize {
        self.sections.type_checking_future.len()
            + self.sections.type_checking_standard_library_direct.len()
            + self.sections.type_checking_standard_library_from.len()
            + self.sections.type_checking_third_party_direct.len()
            + self.sections.type_checking_third_party_from.len()
            + self.sections.type_checking_local_direct.len()
            + self.sections.type_checking_local_from.len()
    }

    /// Generate sorted and formatted import statements
    #[must_use]
    pub fn get_formatted(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut has_previous_section = false;

        // Future imports
        if !self.sections.future.is_empty() {
            let future_imports = self.format_imports(&self.sections.future);
            result.extend(future_imports);
            has_previous_section = true;
        }

        // Standard library imports - direct first, then from
        let std_has_direct = !self.sections.standard_library_direct.is_empty();
        let std_has_from = !self.sections.standard_library_from.is_empty();

        if std_has_direct || std_has_from {
            if has_previous_section {
                result.push(String::new()); // Empty line between sections
            }

            // Direct imports first
            if std_has_direct {
                let std_direct_imports =
                    self.format_imports(&self.sections.standard_library_direct);
                result.extend(std_direct_imports);
            }

            // From imports after direct imports
            if std_has_from {
                let std_from_imports = self.format_imports(&self.sections.standard_library_from);
                result.extend(std_from_imports);
            }

            has_previous_section = true;
        }

        // Third-party imports - direct first, then from
        let third_has_direct = !self.sections.third_party_direct.is_empty();
        let third_has_from = !self.sections.third_party_from.is_empty();

        if third_has_direct || third_has_from {
            if has_previous_section {
                result.push(String::new()); // Empty line between sections
            }

            // Direct imports first
            if third_has_direct {
                let third_direct_imports = self.format_imports(&self.sections.third_party_direct);
                result.extend(third_direct_imports);
            }

            // From imports after direct imports
            if third_has_from {
                let third_from_imports = self.format_imports(&self.sections.third_party_from);
                result.extend(third_from_imports);
            }

            has_previous_section = true;
        }

        // Local imports - direct first, then from
        let local_has_direct = !self.sections.local_direct.is_empty();
        let local_has_from = !self.sections.local_from.is_empty();

        if local_has_direct || local_has_from {
            if has_previous_section {
                result.push(String::new()); // Empty line between sections
            }

            // Direct imports first
            if local_has_direct {
                let local_direct_imports = self.format_imports(&self.sections.local_direct);
                result.extend(local_direct_imports);
            }

            // From imports after direct imports
            if local_has_from {
                let local_from_imports = self.format_imports(&self.sections.local_from);
                result.extend(local_from_imports);
            }
        }

        result
    }

    /// Parse an import statement and categorize it
    fn parse_import(&mut self, import_statement: &str) -> Option<ImportStatement> {
        let trimmed = import_statement.trim();
        if trimmed.is_empty() {
            return None;
        }

        let category = self.categorize_import(trimmed);
        let import_type = if trimmed.starts_with("from ") {
            ImportType::From
        } else {
            ImportType::Direct
        };
        let package = Self::extract_package(trimmed);
        let items = Self::extract_items(trimmed);
        let is_multiline = trimmed.contains('(') || trimmed.contains(')');

        // Reconstruct the statement with sorted items for from imports
        let statement = if import_type == ImportType::From && !items.is_empty() {
            format!("from {} import {}", package, items.join(", "))
        } else {
            trimmed.to_string()
        };

        Some(ImportStatement {
            statement,
            category,
            import_type,
            package,
            items,
            is_multiline,
        })
    }

    /// Categorize an import statement
    fn categorize_import(&mut self, import_statement: &str) -> ImportCategory {
        if import_statement.starts_with("from __future__") {
            return ImportCategory::Future;
        }

        let package = Self::extract_package(import_statement);

        // Check cache first
        if let Some(&cached_category) = self.category_cache.get(&package) {
            return cached_category;
        }

        // Determine category with priority order:
        // 1. Local imports (relative or matching local prefixes)
        // 2. Standard library (built-in or custom registered)
        // 3. Third-party (custom registered or default)
        let category = if self.is_local_import(import_statement) {
            ImportCategory::Local
        } else if self.is_standard_library_package(&package) {
            ImportCategory::StandardLibrary
        } else if self.is_common_third_party_package(&package) {
            ImportCategory::ThirdParty
        } else {
            // Default to third-party for unknown packages
            ImportCategory::ThirdParty
        };

        self.category_cache.insert(package, category);
        category
    }

    /// Extract the package name from an import statement
    fn extract_package(import_statement: &str) -> String {
        if let Some(from_part) = import_statement.strip_prefix("from ") {
            if let Some(import_pos) = from_part.find(" import ") {
                return from_part[..import_pos].trim().to_string();
            }
        } else if let Some(import_part) = import_statement.strip_prefix("import ") {
            // For direct imports, return the full module path
            return import_part
                .split_whitespace()
                .next()
                .unwrap_or(import_part)
                .trim()
                .to_string();
        }

        import_statement.to_string()
    }

    /// Extract imported items from an import statement
    fn extract_items(import_statement: &str) -> Vec<String> {
        if let Some(from_part) = import_statement.strip_prefix("from ") {
            if let Some(import_pos) = from_part.find(" import ") {
                let items_part = &from_part[import_pos + 8..];
                let cleaned = items_part.replace(['(', ')'], "").replace(',', " ");
                let mut items: Vec<String> = cleaned
                    .split_whitespace()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                // Sort items with ALL_CAPS first, then mixed case alphabetically
                items.sort_by(|a, b| Self::custom_import_sort(a, b));
                return items;
            }
        } else if let Some(import_part) = import_statement.strip_prefix("import ") {
            // For direct imports, the "item" is the module itself
            return vec![import_part.trim().to_string()];
        }
        Vec::new()
    }

    /// Check if this is a local/relative import
    fn is_local_import(&self, import_statement: &str) -> bool {
        // Check for relative imports
        if import_statement.contains("from .")
            || import_statement.contains("from ..")
            || import_statement.contains("from ...")
            || import_statement.contains("from ....")
        {
            return true;
        }

        let package = Self::extract_package(import_statement);

        // Check custom local package prefixes first
        for prefix in &self.local_package_prefixes {
            if package.starts_with(prefix.as_str()) {
                return true;
            }
        }

        // Fallback to package_name check for backwards compatibility
        if let Some(pkg_name) = &self.package_name {
            if package.starts_with(pkg_name) {
                return true;
            }
        }

        false
    }

    /// Check if a package is part of Python's standard library
    fn is_standard_library_package(&self, package: &str) -> bool {
        // Check against the constant list of standard library modules
        self.registry.is_stdlib(package)
    }

    /// Check if a package is a common third-party package
    fn is_common_third_party_package(&self, package: &str) -> bool {
        // Check against the constant list of common third-party packages
        self.registry.is_third_party(package)
    }

    /// Format a list of imports, merging same-package imports where appropriate
    #[allow(clippy::unused_self)]
    fn format_imports(&self, imports: &[ImportStatement]) -> Vec<String> {
        let mut package_imports: HashMap<String, Vec<&ImportStatement>> = HashMap::new();

        // Group imports by package
        for import in imports {
            package_imports
                .entry(import.package.clone())
                .or_default()
                .push(import);
        }

        let mut result = Vec::new();
        let mut packages: Vec<_> = package_imports.keys().collect();
        packages.sort();

        for package in packages {
            let imports_for_package = package_imports.get(package).unwrap();

            if imports_for_package.len() == 1 {
                // Single import, use as-is
                result.push(imports_for_package[0].statement.clone());
            } else {
                // Multiple imports from same package, merge if possible
                result.extend(Self::merge_package_imports(imports_for_package));
            }
        }

        result
    }

    /// Merge multiple imports from the same package
    fn merge_package_imports(imports: &[&ImportStatement]) -> Vec<String> {
        let mut all_items = HashSet::new();
        let package = &imports[0].package;

        // Collect all items being imported from this package
        for import in imports {
            all_items.extend(import.items.iter().cloned());
        }

        if all_items.is_empty() {
            // Simple "import package" statements
            return imports.iter().map(|i| i.statement.clone()).collect();
        }

        let mut sorted_items: Vec<_> = all_items.into_iter().collect();
        sorted_items.sort_by(|a, b| Self::custom_import_sort(a, b));

        // Format as single line or multi-line based on length
        if sorted_items.len() <= 3 && sorted_items.iter().map(String::len).sum::<usize>() < 60 {
            // Single line
            vec![format!(
                "from {} import {}",
                package,
                sorted_items.join(", ")
            )]
        } else {
            // Multi-line with parentheses
            let mut result = vec![format!("from {} import (", package)];
            for item in sorted_items {
                result.push(format!("    {item},"));
            }
            result.push(")".to_string());
            result
        }
    }

    /// Custom sorting for import items: `ALL_CAPS` first (alphabetically), then mixed case (alphabetically)
    fn custom_import_sort(a: &str, b: &str) -> std::cmp::Ordering {
        let a_is_all_caps = a.chars().all(|c| (c.is_uppercase() || !c.is_alphabetic()));
        let b_is_all_caps = b.chars().all(|c| (c.is_uppercase() || !c.is_alphabetic()));

        match (a_is_all_caps, b_is_all_caps) {
            // Both are ALL_CAPS or both are mixed case - sort alphabetically
            (true, true) | (false, false) => a.cmp(b),
            // a is ALL_CAPS, b is mixed case - a comes first
            (true, false) => std::cmp::Ordering::Less,
            // a is mixed case, b is ALL_CAPS - b comes first
            (false, true) => std::cmp::Ordering::Greater,
        }
    }

    /// Automatically add `TYPE_CHECKING` to typing import when type checking imports are used
    fn ensure_type_checking_import_added(&mut self) {
        // Check if we already have a typing import with TYPE_CHECKING
        let has_type_checking = self.sections.standard_library_from.iter().any(|import| {
            import.package == "typing" && import.items.contains(&"TYPE_CHECKING".to_string())
        });

        if !has_type_checking {
            // Check if we have any typing import that we can modify
            if let Some(typing_import) = self
                .sections
                .standard_library_from
                .iter_mut()
                .find(|import| import.package == "typing")
            {
                // Add TYPE_CHECKING to existing typing import
                if !typing_import.items.contains(&"TYPE_CHECKING".to_string()) {
                    typing_import.items.push("TYPE_CHECKING".to_string());
                    typing_import
                        .items
                        .sort_by(|a, b| Self::custom_import_sort(a, b));

                    // Update the statement string
                    if typing_import.items.len() == 1 {
                        typing_import.statement =
                            format!("from typing import {}", typing_import.items[0]);
                    } else {
                        typing_import.statement =
                            format!("from typing import {}", typing_import.items.join(", "));
                    }
                }
            } else {
                // No typing import exists, add one with just TYPE_CHECKING
                self.add_import_string("from typing import TYPE_CHECKING");
            }
        }
    }

    /// Clone configuration without imports (useful for creating multiple helpers with same config)
    #[must_use]
    pub fn clone_config(&self) -> Self {
        Self {
            sections: ImportSections::default(),
            category_cache: self.category_cache.clone(),
            package_name: self.package_name.clone(),
            local_package_prefixes: self.local_package_prefixes.clone(),
            registry: self.registry.clone(),
        }
    }
}

/// Convenience functions for common import operations
impl ImportHelper {
    /// Create imports for a model file with required type imports
    pub fn create_model_imports(&mut self, required_types: &[String]) {
        // Standard model imports
        self.add_import_string("from pydantic import BaseModel, ConfigDict, Field");

        // Collect all typing imports needed across all types
        let mut typing_imports = std::collections::HashSet::new();
        let mut collections_abc_imports = std::collections::HashSet::new();
        let mut datetime_imports = Vec::new();
        let mut decimal_imports = Vec::new();

        for type_name in required_types {
            match type_name.as_str() {
                "datetime" | "date" | "time" | "timedelta" => {
                    if !datetime_imports.contains(&type_name.as_str()) {
                        datetime_imports.push(type_name.as_str());
                    }
                }
                "Decimal" => {
                    if !decimal_imports.contains(&"Decimal") {
                        decimal_imports.push("Decimal");
                    }
                }
                "UUID" => {
                    self.add_import_string("from uuid import UUID");
                }
                // For complex types, extract typing imports
                _ => {
                    // Check if this type contains typing elements
                    let extracted_typing = Self::extract_typing_imports_from_type(type_name);
                    typing_imports.extend(extracted_typing);

                    // Check for collections.abc imports
                    if type_name.contains("Callable") {
                        collections_abc_imports.insert("Callable".to_string());
                    }
                }
            }
        }

        // Add datetime imports if any were found
        if !datetime_imports.is_empty() {
            let import_statement = format!("from datetime import {}", datetime_imports.join(", "));
            self.add_regular_import(&import_statement);
        }

        // Add decimal imports if any were found
        if !decimal_imports.is_empty() {
            self.add_import_string("from decimal import Decimal");
        }

        // Add typing imports if any were found (only Any, Generic, TypeVar, Protocol)
        if !typing_imports.is_empty() {
            let mut sorted_typing: Vec<String> = typing_imports.into_iter().collect();
            sorted_typing.sort();
            let import_statement = format!("from typing import {}", sorted_typing.join(", "));
            self.add_regular_import(&import_statement);
        }

        // Add collections.abc imports if any were found (e.g., Callable)
        if !collections_abc_imports.is_empty() {
            let mut sorted_collections: Vec<String> = collections_abc_imports.into_iter().collect();
            sorted_collections.sort();
            let import_statement = format!(
                "from collections.abc import {}",
                sorted_collections.join(", ")
            );
            self.add_regular_import(&import_statement);
        }
    }

    /// Extract typing imports from a complex type string
    /// This handles types like list[Any], dict[str, Any], etc.
    /// Only imports what's actually needed for Python 3.13+ (Any, Generic, `TypeVar`, Protocol)
    fn extract_typing_imports_from_type(type_str: &str) -> std::collections::HashSet<String> {
        let mut typing_imports = std::collections::HashSet::new();

        // Check for Any type (used in generics and standalone)
        if type_str.contains("Any") {
            typing_imports.insert("Any".to_string());
        }

        // Check for Generic type (used for generic classes)
        if type_str.contains("Generic") {
            typing_imports.insert("Generic".to_string());
        }

        // Check for TypeVar usage
        if type_str.contains("TypeVar") {
            typing_imports.insert("TypeVar".to_string());
        }

        // Check for Protocol type (structural subtyping)
        if type_str.contains("Protocol") {
            typing_imports.insert("Protocol".to_string());
        }

        typing_imports
    }
}

impl Default for ImportHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_categorization() {
        let mut helper = ImportHelper::new();

        // Test future imports
        helper.add_import_string("from __future__ import annotations");
        assert_eq!(helper.sections.future.len(), 1);

        // Test standard library from import
        helper.add_import_string("from typing import Optional");
        assert_eq!(helper.sections.standard_library_from.len(), 1);

        // Test standard library direct import
        helper.add_import_string("import uuid");
        assert_eq!(helper.sections.standard_library_direct.len(), 1);

        // Test third party
        helper.add_import_string("from pydantic import BaseModel");
        assert_eq!(helper.sections.third_party_from.len(), 1);

        // Test local imports
        helper.add_import_string("from .models import User");
        assert_eq!(helper.sections.local_from.len(), 1);
    }

    #[test]
    fn test_import_merging() {
        let mut helper = ImportHelper::new();

        helper.add_import_string("from typing import Optional");
        helper.add_import_string("from typing import Any");
        helper.add_import_string("from typing import List");

        let imports = helper.get_formatted();

        // Should merge into a single import
        assert!(imports.iter().any(|i| i.contains("from typing import")));
        assert!(imports
            .iter()
            .any(|i| i.contains("Any") && i.contains("Optional")));
    }

    #[test]
    fn test_alphabetical_sorting_of_import_items() {
        let mut helper = ImportHelper::new();

        // Test with unsorted items in a from import
        helper.add_import_string("from typing import Optional, Any, List, Dict");
        helper.add_import_string("from collections import defaultdict, OrderedDict, Counter");
        helper.add_import_string("import uuid");
        helper.add_import_string("import os");

        let imports = helper.get_formatted();

        // Print for debugging
        for import in &imports {
            println!("{}", import);
        }

        // Check that direct imports come before from imports
        let import_str = imports.join("\n");
        let os_pos = import_str.find("import os").unwrap();
        let uuid_pos = import_str.find("import uuid").unwrap();
        let typing_pos = import_str.find("from typing import").unwrap();

        // Direct imports should come before from imports
        assert!(
            os_pos < typing_pos,
            "Direct imports should come before from imports"
        );
        assert!(
            uuid_pos < typing_pos,
            "Direct imports should come before from imports"
        );

        // Check that items within from imports are sorted alphabetically
        let typing_import = imports
            .iter()
            .find(|s| s.contains("from typing import"))
            .unwrap();

        // Should be: "from typing import Any, Dict, List, Optional"
        assert!(
            typing_import.contains("Any, Dict, List, Optional")
                || typing_import.contains("(\n    Any,\n    Dict,\n    List,\n    Optional,\n)"),
            "Import items should be sorted alphabetically, got: {}",
            typing_import
        );
    }

    #[test]
    fn test_direct_imports_sorted_alphabetically() {
        let mut helper = ImportHelper::new();

        helper.add_import_string("import uuid");
        helper.add_import_string("import os");
        helper.add_import_string("import sys");
        helper.add_import_string("import json");

        let imports = helper.get_formatted();

        // Should be sorted: json, os, sys, uuid
        let import_lines: Vec<String> = imports
            .iter()
            .filter(|s| s.starts_with("import "))
            .cloned()
            .collect();

        assert_eq!(import_lines.len(), 4);
        assert!(import_lines[0].contains("json"));
        assert!(import_lines[1].contains("os"));
        assert!(import_lines[2].contains("sys"));
        assert!(import_lines[3].contains("uuid"));
    }

    #[test]
    fn test_uppercase_priority_in_import_sorting() {
        let mut helper = ImportHelper::new();

        // Test with mixed case items - uppercase should come before lowercase for same letter
        helper.add_import_string("from example import Ab, AA, Aa, AB");

        let imports = helper.get_formatted();

        // Print for debugging
        for import in &imports {
            println!("{}", import);
        }

        let example_import = imports
            .iter()
            .find(|s| s.contains("from example import"))
            .unwrap();

        // Should be: "from example import AA, AB, Aa, Ab"
        // (uppercase A's first, then lowercase a's)
        assert!(
            example_import.contains("AA, AB, Aa, Ab"),
            "Import items should be sorted with uppercase priority, got: {}",
            example_import
        );
    }

    #[test]
    fn test_comprehensive_case_sorting() {
        let mut helper = ImportHelper::new();

        // Test with multiple letters and mixed cases
        helper.add_import_string("from test import zz, ZZ, bb, BB, aa, AA, cc, CC");

        let imports = helper.get_formatted();

        let test_import = imports
            .iter()
            .find(|s| s.contains("from test import"))
            .unwrap();

        // Should be: "from test import AA, BB, CC, ZZ, aa, bb, cc, zz"
        // (all uppercase first in alphabetical order, then all lowercase)
        assert!(
            test_import.contains("AA, BB, CC, ZZ, aa, bb, cc, zz"),
            "Import items should be sorted with uppercase priority across all letters, got: {}",
            test_import
        );
    }

    #[test]
    fn test_type_checking_imports() {
        let mut helper = ImportHelper::with_package_name("mypackage".to_string());

        // Add regular imports
        helper.add_import_string("from __future__ import annotations");
        helper.add_import_string("from typing import Any");
        helper.add_import_string("from pydantic import BaseModel");

        // Add TYPE_CHECKING imports
        helper.add_type_checking_import("import httpx");
        helper.add_type_checking_import("from typing import TYPE_CHECKING");
        helper.add_type_checking_import("from collections.abc import Callable");
        helper.add_type_checking_import("from mypackage.models import User");

        // Check that regular imports are counted correctly
        assert_eq!(helper.count(), 3);
        assert_eq!(helper.count_type_checking(), 4);
        assert!(!helper.is_type_checking_empty());

        // Generate TYPE_CHECKING imports
        let (future, stdlib, third_party, local) = helper.get_type_checking_categorized();

        // Verify categorization
        assert!(
            third_party.iter().any(|s| s.contains("import httpx")),
            "Should have httpx in third_party"
        );
        assert!(
            stdlib
                .iter()
                .any(|s| s.contains("from typing import TYPE_CHECKING")),
            "Should have TYPE_CHECKING"
        );
        assert!(
            stdlib
                .iter()
                .any(|s| s.contains("from collections.abc import Callable")),
            "Should have Callable"
        );
        assert!(
            local
                .iter()
                .any(|s| s.contains("from mypackage.models import User")),
            "Should have User in local"
        );
        assert!(
            future.is_empty(),
            "Should have no future imports in TYPE_CHECKING"
        );
    }

    #[test]
    fn test_programmatic_import_builders() {
        let mut helper = ImportHelper::new();

        // Test add_from_import
        helper.add_from_import("typing", &["Any", "Optional"]);
        helper.add_from_import("json", &["loads"]);

        // Test add_direct_import
        helper.add_direct_import("sys");

        // Test TYPE_CHECKING builders
        helper.add_type_checking_from_import("httpx", &["Client", "Response"]);
        helper.add_type_checking_direct_import("logging");

        // Verify regular imports
        let (_, stdlib, _, _) = helper.get_categorized();
        assert!(
            stdlib
                .iter()
                .any(|s| s.contains("Any") && s.contains("Optional")),
            "Should have Any and Optional in typing imports"
        );
        assert!(stdlib.iter().any(|s| s.contains("from json import loads")));
        assert!(stdlib.iter().any(|s| s.contains("import sys")));

        // Verify TYPE_CHECKING imports
        let (_, tc_stdlib, tc_third_party, _) = helper.get_type_checking_categorized();
        assert!(tc_third_party
            .iter()
            .any(|s| s.contains("from httpx import Client, Response")));
        assert!(tc_stdlib.iter().any(|s| s.contains("import logging")));
    }

    #[test]
    fn test_type_checking_four_categories() {
        let mut helper = ImportHelper::with_package_name("myapp".to_string());

        // Add imports in all four categories for TYPE_CHECKING
        helper.add_type_checking_import("from __future__ import annotations");
        helper.add_type_checking_import("from typing import Protocol");
        helper.add_type_checking_import("from httpx import Client");
        helper.add_type_checking_import("from myapp.models import User");

        let (future, stdlib, third_party, local) = helper.get_type_checking_categorized();

        // Verify all four categories
        assert_eq!(future.len(), 1, "Should have 1 future import");
        assert_eq!(stdlib.len(), 1, "Should have 1 stdlib import");
        assert_eq!(third_party.len(), 1, "Should have 1 third-party import");
        assert_eq!(local.len(), 1, "Should have 1 local import");

        assert!(future[0].contains("from __future__ import annotations"));
        assert!(stdlib[0].contains("from typing import Protocol"));
        assert!(third_party[0].contains("from httpx import Client"));
        assert!(local[0].contains("from myapp.models import User"));
    }
}
