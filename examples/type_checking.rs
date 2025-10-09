//! TYPE_CHECKING imports example
//!
//! This example demonstrates how to use the TYPE_CHECKING block functionality
//! for handling forward references and type-only imports in Python.

use py_import_helper::ImportHelper;

fn main() {
    println!("=== TYPE_CHECKING Imports Example ===\n");

    // Create helper with a package name for local import detection
    let mut helper = ImportHelper::with_package_name("myapp".to_string());

    // Add regular runtime imports
    println!("Adding regular runtime imports...");
    helper.add_from_import("pydantic", &["BaseModel", "Field"]);
    helper.add_from_import("datetime", &["datetime"]);
    helper.add_direct_import("json");

    // Add TYPE_CHECKING imports (for forward references, avoiding circular imports)
    println!("Adding TYPE_CHECKING imports...\n");
    helper.add_type_checking_from_import("httpx", &["Client", "Response"]);
    helper.add_type_checking_from_import("typing", &["Protocol"]);
    helper.add_type_checking_from_import("collections.abc", &["Callable"]);
    helper.add_type_checking_from_import("myapp.models", &["User", "Post"]);
    helper.add_type_checking_direct_import("logging");

    // Show counts
    println!("Regular imports: {}", helper.count());
    println!("TYPE_CHECKING imports: {}\n", helper.count_type_checking());

    // Generate all imports
    let (future, stdlib, third_party, local, tc_future, tc_stdlib, tc_third_party, tc_local) =
        helper.get_all_categorized();

    println!("=== Complete Python File Structure ===\n");

    // Print regular imports
    if !future.is_empty() {
        for import in future {
            println!("{}", import);
        }
        println!();
    }

    if !stdlib.is_empty() {
        for import in &stdlib {
            println!("{}", import);
        }
        println!();
    }

    if !third_party.is_empty() {
        for import in &third_party {
            println!("{}", import);
        }
        println!();
    }

    if !local.is_empty() {
        for import in &local {
            println!("{}", import);
        }
        println!();
    }

    // Print TYPE_CHECKING block if it has imports
    if !helper.is_type_checking_empty() {
        println!("if TYPE_CHECKING:");

        if !tc_future.is_empty() {
            for import in &tc_future {
                println!("    {}", import);
            }
        }

        if !tc_stdlib.is_empty() {
            if !tc_future.is_empty() {
                println!();
            }
            for import in &tc_stdlib {
                println!("    {}", import);
            }
        }

        if !tc_third_party.is_empty() {
            if !tc_future.is_empty() || !tc_stdlib.is_empty() {
                println!();
            }
            for import in &tc_third_party {
                println!("    {}", import);
            }
        }

        if !tc_local.is_empty() {
            if !tc_future.is_empty() || !tc_stdlib.is_empty() || !tc_third_party.is_empty() {
                println!();
            }
            for import in &tc_local {
                println!("    {}", import);
            }
        }

        println!();
    }

    println!("\n=== Separate TYPE_CHECKING Categories ===\n");

    let (tc_future, tc_stdlib, tc_third_party, tc_local) = helper.get_type_checking_categorized();

    println!("TYPE_CHECKING Future ({}):", tc_future.len());
    for import in tc_future {
        println!("  {}", import);
    }

    println!("\nTYPE_CHECKING Standard Library ({}):", tc_stdlib.len());
    for import in tc_stdlib {
        println!("  {}", import);
    }

    println!("\nTYPE_CHECKING Third-Party ({}):", tc_third_party.len());
    for import in tc_third_party {
        println!("  {}", import);
    }

    println!("\nTYPE_CHECKING Local ({}):", tc_local.len());
    for import in tc_local {
        println!("  {}", import);
    }
}
