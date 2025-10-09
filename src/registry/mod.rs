//! Package registry for categorizing Python imports
//!
//! This module provides a configurable registry for determining whether packages
//! are standard library, third-party, or local imports. Users can update these
//! registries at runtime to handle custom packages or Python version differences.

pub mod constants;

use constants::{COMMON_THIRD_PARTY_PACKAGES, PYTHON_STDLIB_MODULES};
use std::collections::HashSet;

/// Registry for package categorization
///
/// Maintains lists of known standard library and third-party packages.
/// These can be updated by users to handle custom packages or different
/// Python versions.
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    /// Known Python standard library packages
    stdlib_packages: HashSet<String>,
    /// Known third-party packages
    third_party_packages: HashSet<String>,
}

impl PackageRegistry {
    /// Create a new registry with default Python 3.13 stdlib and common third-party packages
    #[must_use]
    pub fn new() -> Self {
        Self {
            stdlib_packages: Self::default_stdlib_packages(),
            third_party_packages: Self::default_third_party_packages(),
        }
    }

    /// Check if a package is in the standard library
    #[must_use]
    pub fn is_stdlib(&self, package: &str) -> bool {
        self.stdlib_packages.contains(package)
    }

    /// Check if a package is a known third-party package
    #[must_use]
    pub fn is_third_party(&self, package: &str) -> bool {
        self.third_party_packages.contains(package)
    }

    /// Add a package to the standard library registry
    ///
    /// # Examples
    ///
    /// ```
    /// use py_import_helper::ImportHelper;
    ///
    /// let mut helper = ImportHelper::new();
    /// helper.registry_mut().add_stdlib_package("my_custom_stdlib");
    /// ```
    pub fn add_stdlib_package(&mut self, package: impl Into<String>) -> &mut Self {
        self.stdlib_packages.insert(package.into());
        self
    }

    /// Add a package to the third-party registry
    ///
    /// # Examples
    ///
    /// ```
    /// use py_import_helper::ImportHelper;
    ///
    /// let mut helper = ImportHelper::new();
    /// helper.registry_mut().add_third_party_package("my_company_lib");
    /// ```
    pub fn add_third_party_package(&mut self, package: impl Into<String>) -> &mut Self {
        self.third_party_packages.insert(package.into());
        self
    }

    /// Remove a package from the standard library registry
    pub fn remove_stdlib_package(&mut self, package: &str) -> &mut Self {
        self.stdlib_packages.remove(package);
        self
    }

    /// Remove a package from the third-party registry
    pub fn remove_third_party_package(&mut self, package: &str) -> &mut Self {
        self.third_party_packages.remove(package);
        self
    }

    /// Clear all standard library packages
    pub fn clear_stdlib_packages(&mut self) -> &mut Self {
        self.stdlib_packages.clear();
        self
    }

    /// Clear all third-party packages
    pub fn clear_third_party_packages(&mut self) -> &mut Self {
        self.third_party_packages.clear();
        self
    }

    /// Add multiple standard library packages at once
    pub fn add_stdlib_packages(&mut self, packages: &[&str]) -> &mut Self {
        for package in packages {
            self.stdlib_packages.insert((*package).to_string());
        }
        self
    }

    /// Add multiple third-party packages at once
    pub fn add_third_party_packages(&mut self, packages: &[&str]) -> &mut Self {
        for package in packages {
            self.third_party_packages.insert((*package).to_string());
        }
        self
    }

    /// Get the default Python 3.13 standard library packages
    fn default_stdlib_packages() -> HashSet<String> {
        PYTHON_STDLIB_MODULES
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    }

    /// Get the default common third-party packages
    fn default_third_party_packages() -> HashSet<String> {
        COMMON_THIRD_PARTY_PACKAGES
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    }

    /// Reset to default stdlib packages (Python 3.13)
    pub fn reset_stdlib_to_defaults(&mut self) -> &mut Self {
        self.stdlib_packages = Self::default_stdlib_packages();
        self
    }

    /// Reset to default third-party packages
    pub fn reset_third_party_to_defaults(&mut self) -> &mut Self {
        self.third_party_packages = Self::default_third_party_packages();
        self
    }

    /// Get count of registered stdlib packages
    #[must_use]
    pub fn count_stdlib_packages(&self) -> usize {
        self.stdlib_packages.len()
    }

    /// Get count of registered third-party packages
    #[must_use]
    pub fn count_third_party_packages(&self) -> usize {
        self.third_party_packages.len()
    }
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_registry() {
        let registry = PackageRegistry::new();
        assert!(registry.is_stdlib("typing"));
        assert!(registry.is_stdlib("os"));
        assert!(registry.is_third_party("pydantic"));
        assert!(registry.is_third_party("pytest"));
    }

    #[test]
    fn test_add_stdlib_package() {
        let mut registry = PackageRegistry::new();
        registry.add_stdlib_package("my_custom_stdlib");
        assert!(registry.is_stdlib("my_custom_stdlib"));
    }

    #[test]
    fn test_add_third_party_package() {
        let mut registry = PackageRegistry::new();
        registry.add_third_party_package("my_company_lib");
        assert!(registry.is_third_party("my_company_lib"));
    }

    #[test]
    fn test_remove_packages() {
        let mut registry = PackageRegistry::new();
        registry.remove_stdlib_package("typing");
        assert!(!registry.is_stdlib("typing"));
    }

    #[test]
    fn test_bulk_add() {
        let mut registry = PackageRegistry::new();
        registry.add_stdlib_packages(&["pkg1", "pkg2", "pkg3"]);
        assert!(registry.is_stdlib("pkg1"));
        assert!(registry.is_stdlib("pkg2"));
        assert!(registry.is_stdlib("pkg3"));
    }

    #[test]
    fn test_reset_to_defaults() {
        let mut registry = PackageRegistry::new();
        registry.clear_stdlib_packages();
        assert_eq!(registry.count_stdlib_packages(), 0);

        registry.reset_stdlib_to_defaults();
        assert!(registry.count_stdlib_packages() > 0);
        assert!(registry.is_stdlib("typing"));
    }

    #[test]
    fn test_chaining() {
        let mut registry = PackageRegistry::new();
        registry
            .add_stdlib_package("pkg1")
            .add_stdlib_package("pkg2")
            .add_third_party_package("lib1");

        assert!(registry.is_stdlib("pkg1"));
        assert!(registry.is_stdlib("pkg2"));
        assert!(registry.is_third_party("lib1"));
    }
}
