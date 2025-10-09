# py-import-helper 🐍🦀

> A Python import organizer written in Rust.

Hey! 👋 I'm learning Rust and built this library to solve a problem I kept
running into: **organizing Python imports in my code generation projects**.

When you're generating Python code, you end up with a mess of import statements
scattered everywhere. This library helps you collect them, sort them properly
(following PEP 8), and output them in the right order.

[![Crates.io](https://img.shields.io/crates/v/py-import-helper.svg)](https://crates.io/crates/py-import-helper)
[![Test](https://github.com/timrabl/rs-py-import-helper/workflows/Test/badge.svg)](https://github.com/timrabl/rs-py-import-helper/actions)
[![Security
Audit](https://github.com/timrabl/rs-py-import-helper/workflows/Security%20Audit/badge.svg)](https://github.com/timrabl/rs-py-import-helper/actions)
[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What it does

- ✨ **Automatically sorts imports** the way Python tools like `isort` and
  `black` expect
- 📦 **Groups imports properly**: future → stdlib → third-party → your local
  code
- 🔀 **Merges duplicate imports** from the same package
- 🔍 **Handles TYPE_CHECKING** imports separately (for forward references)
- 🛠️ **Easy to use** - just throw import strings at it and get organized output

## Why I built this

I was working on some Rust projects that generate Python code (think: API
clients, data models, etc.), and I kept writing the same import-organizing logic
over and over. Plus, I wanted to learn Rust better by building something
actually useful.

This is my first "real" Rust library, so the code might not be perfect, but it
works well and has decent test coverage!

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
py-import-helper = "0.2"
```

Then use it:

```rust
use py_import_helper::ImportHelper;

let mut helper = ImportHelper::new();

// Just throw your imports at it
helper.add_import_string("from typing import Any, Optional");
helper.add_import_string("from pydantic import BaseModel");
helper.add_import_string("import json");
helper.add_import_string("from myproject.models import User");

// Get back properly organized imports
let formatted = helper.get_formatted();
for line in formatted {
    println!("{}", line);
}
```

**Output:**

```python
import json
from typing import Any, Optional

from pydantic import BaseModel

from myproject.models import User
```

## More Examples

**Working with TYPE_CHECKING imports:**

```rust
// For type hints that should only exist during type checking
helper.add_type_checking_from_import("httpx", &["Client"]);

// Get all imports including TYPE_CHECKING ones
let (future, stdlib, third_party, local, tc_future, tc_stdlib, tc_third_party, tc_local) =
    helper.get_all_categorized();
```

**Customizing what counts as "local" imports:**

```rust
let mut helper = ImportHelper::with_package_name("myproject".to_string());
helper.add_local_package_prefix("myproject_utils");

// Now these are recognized as local imports
helper.add_import_string("from myproject.core import Engine");
helper.add_import_string("from myproject_utils.helpers import format_date");
```

## Contributing

I'm still learning Rust, so if you see something that could be done better,
please let me know! Pull requests are welcome.

**Areas I'd love help with:**

- Better error handling
- Performance improvements
- More Python import edge cases
- Documentation improvements

## How It Works

The library recognizes different types of Python imports:

- **Standard library**: `json`, `os`, `typing`, etc.
- **Third-party**: `pydantic`, `httpx`, `fastapi`, etc.
- **Local**: Your project's own modules

It also handles `TYPE_CHECKING` imports separately for type hints that should
only exist during type checking.

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- 📚 [Full Documentation](https://docs.rs/py-import-helper)
- 📦 [Crates.io](https://crates.io/crates/py-import-helper)
- 🐛 [Report Issues](https://github.com/timrabl/rs-py-import-helper/issues)
