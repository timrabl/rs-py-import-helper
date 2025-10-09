//! Basic usage example for py-import-helper
//!
//! This example demonstrates the fundamental features of the library including
//! import categorization, merging, and formatting.

use py_import_helper::ImportHelper;

fn main() {
    println!("=== Basic Import Helper Example ===\n");

    // Create a new import helper
    let mut helper = ImportHelper::new();

    println!("Adding various Python imports...\n");

    // Add future imports
    helper.add_import_string("from __future__ import annotations");

    // Add standard library imports (various styles)
    helper.add_direct_import("json");
    helper.add_direct_import("sys");
    helper.add_from_import("typing", &["Any", "Optional", "List", "Dict"]);
    helper.add_from_import("collections", &["defaultdict"]);
    helper.add_from_import("datetime", &["datetime", "date"]);

    // Add third-party imports
    helper.add_import_string("from pydantic import BaseModel, Field");
    helper.add_import_string("from httpx import Client, AsyncClient");
    helper.add_import_string("import pytest");

    // Add multiple imports from same package (will be merged)
    helper.add_from_import("pathlib", &["Path"]);
    helper.add_from_import("pathlib", &["PurePath"]);

    println!("Total imports collected: {}\n", helper.count());

    // Generate sorted and formatted imports
    println!("=== Formatted Python Imports ===\n");
    let imports = helper.get_formatted();
    for import in &imports {
        println!("{}", import);
    }

    println!("\n=== Categorized Imports ===\n");

    // Get categorized imports
    let (_future, stdlib, third_party, local) = helper.get_categorized();

    println!("Standard Library ({} imports):", stdlib.len());
    for import in &stdlib {
        println!("  {}", import);
    }

    println!("\nThird-Party ({} imports):", third_party.len());
    for import in &third_party {
        println!("  {}", import);
    }

    if !local.is_empty() {
        println!("\nLocal ({} imports):", local.len());
        for import in &local {
            println!("  {}", import);
        }
    } else {
        println!("\nLocal: (no local imports)");
    }

    // Demonstrate helper reset
    println!("\n=== Demonstrating Helper Reset ===\n");

    helper.reset();
    println!("After reset, import count: {}", helper.count());

    helper.add_from_import("typing", &["Protocol"]);
    helper.add_import_string("import os");

    println!("After adding new imports: {}", helper.count());

    let new_imports = helper.get_formatted();
    println!("\nNew imports:");
    for import in new_imports {
        println!("  {}", import);
    }
}
