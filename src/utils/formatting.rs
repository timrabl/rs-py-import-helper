//! Import formatting utilities
//!
//! This module provides functions for formatting Python import statements
//! according to PEP 8 and common formatting standards (isort, Black).

use super::parsing::custom_import_sort;
use crate::types::ImportStatement;
use std::collections::{HashMap, HashSet};

/// Format a list of imports, merging same-package imports where appropriate
#[must_use]
pub fn format_imports(imports: &[ImportStatement]) -> Vec<String> {
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
        let imports_for_package = package_imports
            .get(package)
            .expect("BUG: package key must exist in HashMap");

        if imports_for_package.len() == 1 {
            // Single import, use as-is
            result.push(imports_for_package[0].statement.clone());
        } else {
            // Multiple imports from same package, merge if possible
            result.extend(merge_package_imports(imports_for_package));
        }
    }

    result
}

/// Merge multiple imports from the same package
#[must_use]
pub fn merge_package_imports(imports: &[&ImportStatement]) -> Vec<String> {
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
    sorted_items.sort_by(|a, b| custom_import_sort(a, b));

    // Format as single line or multi-line based on length
    if sorted_items.len() <= 3
        && sorted_items
            .iter()
            .map(std::string::String::len)
            .sum::<usize>()
            < 60
    {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ImportCategory, ImportType};

    #[test]
    fn test_merge_package_imports() {
        let import1 = ImportStatement {
            statement: "from typing import Any".to_string(),
            category: ImportCategory::StandardLibrary,
            import_type: ImportType::From,
            package: "typing".to_string(),
            items: vec!["Any".to_string()],
            is_multiline: false,
        };

        let import2 = ImportStatement {
            statement: "from typing import Optional".to_string(),
            category: ImportCategory::StandardLibrary,
            import_type: ImportType::From,
            package: "typing".to_string(),
            items: vec!["Optional".to_string()],
            is_multiline: false,
        };

        let merged = merge_package_imports(&[&import1, &import2]);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].contains("Any"));
        assert!(merged[0].contains("Optional"));
    }
}
