# Naming Conventions

This document establishes naming conventions for the `py-import-helper`
codebase.

## General Principles

- **Clarity over brevity**: Use descriptive names that clearly indicate purpose
- **Consistency**: Use the same patterns throughout the codebase
- **Standard Rust conventions**: Follow Rust API guidelines

## Rust Naming Standards

### Constants

```rust
const MAX_LINE_LENGTH: usize = 88;          // SCREAMING_SNAKE_CASE
pub const PYTHON_STDLIB_MODULES: &[&str];   // Public constants
```

### Types

```rust
struct ImportHelper { }         // PascalCase for types
enum ImportCategory { }         // PascalCase for enums
type ImportMap = HashMap<...>;  // PascalCase for type aliases
```

### Variables and Parameters

```rust
let import_statement = "...";   // snake_case
let package_name = "myapp";     // snake_case
fn process_import(stmt: &str)   // snake_case
```

### Functions and Methods

```rust
pub fn new() -> Self                        // snake_case
pub fn add_from_import(&mut self, ...)      // snake_case
fn parse_import_statement(&self, ...)       // snake_case (private)
```

## Method Naming Patterns

### Constructor Patterns

- `new()` - Default constructor
- `with_*()` - Constructor with specific configuration
    - Example: `with_package_name(name: String)`
    - Example: `with_config(config: Config)`

### Add/Insert Patterns

- `add_*()` - Add items to collections
    - `add_from_string(stmt: &str)` - Add from Python string
    - `add_from_import(pkg, items)` - Add using builder pattern
    - `add_direct_import(module)` - Add direct import
    - `add_type_checking_*()` - Add to TYPE_CHECKING block

### Get/Retrieve Patterns

- `get_*()` - Retrieve data (preferred over `generate_*`)
    - `get_categorized()` - Get imports by category
    - `get_formatted()` - Get formatted output
    - `get_all_categorized()` - Get all categories including TYPE_CHECKING

### Boolean Check Patterns

- `is_*()` - Boolean checks returning true/false
    - `is_empty()` - Check if no imports
    - `is_type_checking_empty()` - Check if no TYPE_CHECKING imports
    - `is_relative_import()` - Check if import is relative

### Count Patterns

- `count*()` - Count items
    - `count()` - Count regular imports
    - `count_type_checking()` - Count TYPE_CHECKING imports

### Modification Patterns

- `reset()` - Clear all data, keep config
- `clear_*()` - Clear specific data
    - `clear_cache()` - Clear categorization cache
- `clone_*()` - Clone with specific behavior
    - `clone_config()` - Clone configuration without data

## Current API Method Names

### ✅ Well-Named Methods (Keep as-is)

```rust
// Constructors
new() -> Self
with_package_name(name: String) -> Self

// Configuration
add_local_package_prefix(prefix) -> &mut Self
add_local_package_prefixes(prefixes) -> &mut Self
clear_cache() -> &mut Self

// Adding imports - Builder pattern
add_from_import(package, items)
add_direct_import(module)
add_type_checking_from_import(package, items)
add_type_checking_direct_import(module)

// Queries
is_empty() -> bool
is_type_checking_empty() -> bool
count() -> usize
count_type_checking() -> usize

// State management
reset() -> &mut Self
clone_config() -> Self
```

### 🔄 Methods to Consider Renaming (Future versions)

| Current Name | Suggested Name | Reason |
|--------------|----------------|--------|
| `add_import(spec)` | `add_from_spec(spec)` | Clearer that it takes ImportSpec |
| `add_import_string(s)` | `add_from_string(s)` | Consistent with other `add_from_*` methods |
| `dump()` | `get_categorized()` | More descriptive verb |
| `generate_categorized_imports()` | (remove - duplicate of dump) | |
| `generate_all_imports()` | `get_all_categorized()` | Consistent naming |
| `generate_type_checking_imports()` | `get_type_checking_categorized()` | Consistent naming |
| `generate_sorted_imports()` | `get_formatted()` | More descriptive |

## File and Module Naming

### Files

- `lib.rs` - Library entry point
- `types.rs` - Type definitions
- `const.rs` - Constants
- `core.rs` - Core implementation

### Modules

```rust
mod types;              // Public types
mod r#const;            // Constants (r# for keyword)
mod core;               // Core logic
pub mod parser;         // Public utilities (if added)
pub mod formatter;      // Public utilities (if added)
```

## Documentation Comments

### Public Items

All public items MUST have doc comments:

```rust
/// Create a new import helper instance
///
/// # Examples
///
/// ```
/// use py_import_helper::ImportHelper;
/// let helper = ImportHelper::new();
/// ```
pub fn new() -> Self {
    Self::default()
}
```

### Private Items

Private items SHOULD have doc comments for complex logic:

```rust
/// Parse import statement and extract package name
fn extract_package(&self, stmt: &str) -> String {
    // implementation
}
```

## Test Naming

```rust
#[test]
fn test_<functionality>_<scenario>() {
    // test_import_merging_same_package()
    // test_categorization_stdlib_modules()
    // test_formatting_multiline_imports()
}
```

## Variable Naming in Implementation

### Good Examples

```rust
let import_statement = "from typing import Any";
let package_name = "typing";
let stdlib_imports = vec![];
let third_party_imports = vec![];
let local_imports = vec![];
let is_relative = stmt.starts_with("from .");
let has_type_checking = !sections.type_checking_future.is_empty();
```

### Avoid

```rust
let stmt = "...";           // Too abbreviated
let imp = "...";            // Ambiguous
let x = vec![];             // Non-descriptive
let tmp = String::new();    // Generic name
```

## Consistency Checklist

When adding new methods:

- [ ] Follow verb pattern (add/get/is/count/clear/reset)
- [ ] Use snake_case for method names
- [ ] Parameter names are descriptive
- [ ] Return type is clear from name
- [ ] Add doc comment with example
- [ ] Add test with test_<method>_<scenario> pattern
- [ ] Update README if public API

## Future Refactoring Considerations

If the codebase grows significantly, consider:

1. **Module splitting**:

   ```
   src/
   ├── lib.rs
   ├── types.rs
   ├── const.rs
   ├── helper/
   │   ├── mod.rs
   │   ├── builder.rs      # add_* methods
   │   ├── getter.rs       # get_*, is_*, count_* methods
   │   └── categorizer.rs  # categorization logic
   ├── parser.rs           # Parsing utilities
   └── formatter.rs        # Formatting utilities
   ```

2. **Trait-based design**:

   ```rust
   trait ImportBuilder {
       fn add_from_string(&mut self, stmt: &str);
       fn add_from_import(&mut self, pkg: &str, items: &[&str]);
   }

   trait ImportGetter {
       fn get_categorized(&self) -> (Vec<String>, Vec<String>, Vec<String>);
       fn get_formatted(&self) -> Vec<String>;
   }
   ```

3. **Builder pattern for configuration**:

   ```rust
   let helper = ImportHelper::builder()
       .with_package_name("myapp")
       .with_local_prefixes(vec!["myapp_utils"])
       .build();
   ```

## References

- [Rust API Guidelines -
  Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [PEP 8 - Style Guide for Python Code](https://peps.python.org/pep-0008/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
