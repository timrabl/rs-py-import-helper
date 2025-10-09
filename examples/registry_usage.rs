//! Example demonstrating the PackageRegistry feature
//!
//! Shows how to customize package categorization at runtime
//! using the registry system.

use py_import_helper::ImportHelper;

fn main() {
    println!("=== Package Registry Example ===\n");

    // Example 1: Adding custom stdlib packages
    println!("1. Adding custom stdlib package:");
    let mut helper = ImportHelper::new();

    // Before: unknown package defaults to third-party
    helper.add_import_string("import my_custom_stdlib");
    let (_future, _stdlib, third_party, _local) = helper.get_categorized();
    println!("   Before registry update:");
    println!("   Third-party imports: {:?}", third_party);

    // After: register as stdlib
    helper.clear_cache(); // Clear categorization cache
    helper.registry_mut().add_stdlib_package("my_custom_stdlib");

    let mut helper2 = ImportHelper::new();
    helper2
        .registry_mut()
        .add_stdlib_package("my_custom_stdlib");
    helper2.add_import_string("import my_custom_stdlib");
    let (_future, stdlib, third_party, _local) = helper2.get_categorized();
    println!("\n   After registry update:");
    println!("   Stdlib imports: {:?}", stdlib);
    println!("   Third-party imports: {:?}", third_party);

    // Example 2: Adding company-specific packages
    println!("\n2. Adding company packages:");
    let mut helper = ImportHelper::new();

    helper
        .registry_mut()
        .add_third_party_package("mycompany_auth")
        .add_third_party_package("mycompany_utils")
        .add_third_party_package("mycompany_db");

    helper.add_import_string("from mycompany_auth import authenticate");
    helper.add_import_string("from mycompany_utils import format_date");
    helper.add_import_string("from mycompany_db import Connection");

    let formatted = helper.get_formatted();
    println!("   Company packages recognized:");
    for import in formatted {
        println!("   {}", import);
    }

    // Example 3: Bulk operations
    println!("\n3. Bulk package registration:");
    let mut helper = ImportHelper::new();

    // Add multiple packages at once
    helper
        .registry_mut()
        .add_stdlib_packages(&["custom_pkg1", "custom_pkg2", "custom_pkg3"]);

    println!(
        "   Registered {} stdlib packages",
        helper.registry().count_stdlib_packages()
    );
    println!(
        "   Registered {} third-party packages",
        helper.registry().count_third_party_packages()
    );

    // Example 4: Python version-specific packages
    println!("\n4. Python version-specific handling:");
    let mut helper = ImportHelper::new();

    // For Python 3.9+, tomllib is stdlib (built-in from 3.11)
    helper.registry_mut().add_stdlib_package("tomllib");

    // For older Python, you might remove it and use tomli instead
    // helper.registry_mut().remove_stdlib_package("tomllib");
    // helper.registry_mut().add_third_party_package("tomli");

    helper.add_import_string("import tomllib");
    let (_future, stdlib, _third, _local) = helper.get_categorized();
    println!("   tomllib categorized as stdlib: {}", !stdlib.is_empty());

    // Example 5: Reset to defaults
    println!("\n5. Reset registry:");
    let mut helper = ImportHelper::new();

    helper.registry_mut().add_stdlib_package("custom");
    println!(
        "   Added custom package, count: {}",
        helper.registry().count_stdlib_packages()
    );

    helper.registry_mut().reset_stdlib_to_defaults();
    println!(
        "   After reset, count: {}",
        helper.registry().count_stdlib_packages()
    );

    // Example 6: Configuration cloning preserves registry
    println!("\n6. Cloning configuration:");
    let mut helper = ImportHelper::new();
    helper
        .registry_mut()
        .add_stdlib_package("custom1")
        .add_third_party_package("custom2");

    // Clone config preserves registry settings
    let helper2 = helper.clone_config();
    println!(
        "   Original registry: {} stdlib, {} third-party",
        helper.registry().count_stdlib_packages(),
        helper.registry().count_third_party_packages()
    );
    println!(
        "   Cloned registry: {} stdlib, {} third-party",
        helper2.registry().count_stdlib_packages(),
        helper2.registry().count_third_party_packages()
    );

    println!("\n=== Example Complete ===");
}
