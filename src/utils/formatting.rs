//! Import formatting utilities
//!
//! This module provides functions for formatting Python import statements
//! according to PEP 8 and common formatting standards (isort, Black).

use super::parsing::custom_import_sort;
use crate::types::{FormattingConfig, ImportStatement};
use std::collections::{HashMap, HashSet};

/// Format a list of imports, merging same-package imports where appropriate
#[must_use]
pub fn format_imports(imports: &[ImportStatement], config: &FormattingConfig) -> Vec<String> {
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

        if let Some(first) = imports_for_package.get(0) {
            if imports_for_package.len() == 1 && first.items.is_empty() {
                // Single direct import (e.g., "import os"), use as-is
                result.push(first.statement.clone());
            } else {
                // Either multiple imports from same package, or a single import with items
                // In both cases, apply formatting logic (may need multi-line)
                result.extend(merge_package_imports(imports_for_package, config));
            }
        }
            // Either multiple imports from same package, or a single import with items
            // In both cases, apply formatting logic (may need multi-line)
            result.extend(merge_package_imports(imports_for_package, config));
        }
    }

    result
}

/// Merge multiple imports from the same package with configurable formatting
#[must_use]
pub fn merge_package_imports(
    imports: &[&ImportStatement],
    config: &FormattingConfig,
) -> Vec<String> {
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

    // Determine if we should use multi-line format
    let should_use_multiline = if config.force_multiline {
        true
    } else if config.force_single_line {
        false
    } else {
        // Auto-detect based on configuration
        let total_chars = sorted_items.iter().map(String::len).sum::<usize>();
        let import_line_length = "from ".len()
            + package.len()
            + " import ".len()
            + total_chars
            + (sorted_items.len() * 2);

        sorted_items.len() >= config.multiline_threshold || import_line_length > config.line_length
    };

    if should_use_multiline {
        // Multi-line with parentheses
        let indent = " ".repeat(config.indent_size);
        let mut result = vec![format!("from {} import (", package)];

        for item in &sorted_items {
            if config.use_trailing_comma {
                result.push(format!("{}{},", indent, item));
            } else {
                result.push(format!("{}{}", indent, item));
            }
        }

        result.push(")".to_string());
        result
    } else {
        // Single line
        vec![format!(
            "from {} import {}",
            package,
            sorted_items.join(", ")
        )]
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

        let config = FormattingConfig::default();
        let merged = merge_package_imports(&[&import1, &import2], &config);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].contains("Any"));
        assert!(merged[0].contains("Optional"));
    }
}
