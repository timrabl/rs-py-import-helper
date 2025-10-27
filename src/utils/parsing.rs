//! Import statement parsing utilities
//!
//! This module provides functions for parsing Python import statements
//! and extracting relevant information such as package names and imported items.

use crate::types::{ImportCategory, ImportStatement, ImportType};

/// Extract the package name from an import statement
///
/// # Examples
///
/// ```
/// use py_import_helper::utils::parsing::extract_package;
///
/// assert_eq!(extract_package("from typing import Any"), "typing");
/// assert_eq!(extract_package("import json"), "json");
/// assert_eq!(extract_package("from collections.abc import Mapping"), "collections.abc");
/// ```
#[must_use]
pub fn extract_package(import_statement: &str) -> String {
    if let Some(from_part) = import_statement.strip_prefix("from ") {
        // Use split_once for Unicode-safe splitting
        if let Some((package, _)) = from_part.split_once(" import ") {
            let pkg = package.trim();
            // Validate non-empty package
            if pkg.is_empty() {
                return import_statement.to_string();
            }
            return pkg.to_string();
        }
    } else if let Some(import_part) = import_statement.strip_prefix("import ") {
        // For direct imports, return the full module path
        let pkg = import_part
            .split_whitespace()
            .next()
            .unwrap_or(import_part)
            .trim();
        // Validate non-empty package
        if pkg.is_empty() {
            return import_statement.to_string();
        }
        return pkg.to_string();
    }

    import_statement.to_string()
}

/// Extract imported items from an import statement
///
/// Items are automatically sorted with `ALL_CAPS` names first, then mixed case alphabetically.
///
/// # Examples
///
/// ```
/// use py_import_helper::utils::parsing::extract_items;
///
/// let items = extract_items("from typing import Any, Optional");
/// assert_eq!(items, vec!["Any", "Optional"]);
///
/// let items = extract_items("from typing import TYPE_CHECKING, Any");
/// assert_eq!(items, vec!["TYPE_CHECKING", "Any"]);
/// ```
#[must_use]
pub fn extract_items(import_statement: &str) -> Vec<String> {
    if let Some(from_part) = import_statement.strip_prefix("from ") {
        // Use split_once for Unicode-safe splitting
        if let Some((_, items_part)) = from_part.split_once(" import ") {
            // Unicode-safe character replacement in single pass
            let cleaned: String = items_part
                .chars()
                .map(|c| match c {
                    '(' | ')' | ',' => ' ',
                    _ => c,
                })
                .collect();
            let mut items: Vec<String> = cleaned
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // Sort items with ALL_CAPS first, then mixed case alphabetically
            items.sort_by(|a, b| custom_import_sort(a, b));
            return items;
        }
    } else if let Some(import_part) = import_statement.strip_prefix("import ") {
        // For direct imports, the "item" is the module itself
        return vec![import_part.trim().to_string()];
    }
    Vec::new()
}

/// Custom sorting for import items: `ALL_CAPS` first (alphabetically), then mixed case (alphabetically)
///
/// This follows the convention used by isort and Black formatters.
/// Wildcard imports (*) always come last.
#[must_use]
pub fn custom_import_sort(a: &str, b: &str) -> std::cmp::Ordering {
    // Wildcard imports always come last
    match (a, b) {
        ("*", "*") => return std::cmp::Ordering::Equal,
        ("*", _) => return std::cmp::Ordering::Greater,
        (_, "*") => return std::cmp::Ordering::Less,
        _ => {}
    }

    // Check if names are ALL_CAPS by filtering to only alphabetic characters
    // This correctly handles names like "TYPE_CHECKING" and "_private"
    let a_is_all_caps = !a.is_empty()
        && a.chars()
            .filter(|c| c.is_alphabetic())
            .all(char::is_uppercase);
    let b_is_all_caps = !b.is_empty()
        && b.chars()
            .filter(|c| c.is_alphabetic())
            .all(char::is_uppercase);

    match (a_is_all_caps, b_is_all_caps) {
        // Both are ALL_CAPS or both are mixed case - sort alphabetically (case-insensitive)
        (true, true) | (false, false) => {
            // Case-insensitive comparison to match isort/ruff behavior
            let a_lower = a.to_lowercase();
            let b_lower = b.to_lowercase();
            match a_lower.cmp(&b_lower) {
                std::cmp::Ordering::Equal => a.cmp(b), // If equal case-insensitively, use case-sensitive as tiebreaker
                other => other,
            }
        }
        // a is ALL_CAPS, b is mixed case - a comes first
        (true, false) => std::cmp::Ordering::Less,
        // a is mixed case, b is ALL_CAPS - b comes first
        (false, true) => std::cmp::Ordering::Greater,
    }
}

/// Parse an import statement and categorize it
#[must_use]
pub fn parse_import(import_statement: &str, category: ImportCategory) -> Option<ImportStatement> {
    let trimmed = import_statement.trim();
    if trimmed.is_empty() {
        return None;
    }

    let import_type = if trimmed.starts_with("from ") {
        ImportType::From
    } else {
        ImportType::Direct
    };

    let package = extract_package(trimmed);
    let items = extract_items(trimmed);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_package() {
        assert_eq!(extract_package("from typing import Any"), "typing");
        assert_eq!(extract_package("import json"), "json");
        assert_eq!(
            extract_package("from collections.abc import Mapping"),
            "collections.abc"
        );
    }

    #[test]
    fn test_extract_items() {
        let items = extract_items("from typing import Any, Optional");
        assert_eq!(items, vec!["Any", "Optional"]);

        let items = extract_items("from typing import TYPE_CHECKING, Any");
        assert_eq!(items, vec!["TYPE_CHECKING", "Any"]);
    }

    #[test]
    fn test_custom_import_sort() {
        let mut items = vec!["Any", "TYPE_CHECKING", "Optional", "LITERAL"];
        items.sort_by(|a, b| custom_import_sort(a, b));
        assert_eq!(items, vec!["LITERAL", "TYPE_CHECKING", "Any", "Optional"]);
    }
}
