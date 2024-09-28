# Bellande Importer

- Bellande Importer is an Importer file that makes it easier to import functions and classes and make other files like an extensions of another file

## 
```
use bellande_rust_import::{Importer, import, from_import};
use std::path::PathBuf;

fn main() {
    // Create a new Importer instance
    let mut importer = Importer::new();

    // Add a search path for modules
    importer.add_search_path(PathBuf::from("src/modules"));

    // Import a module
    let module = import!(importer, "math");

    // Import specific symbols from a module
    from_import!(importer, "math", add, subtract);

    // Use the imported symbols
    println!("2 + 3 = {}", add(2, 3));
    println!("5 - 2 = {}", subtract(5, 2));

    // Access the full module structure
    println!("Module path: {:?}", module.path);
    println!("Number of items: {}", module.ast.items.len());
}
```

## Website Crates
- https://crates.io/crates/bellande_rust_import

### Installation
- `cargo add bellande_rust_import`

```
Name: bellande_rust_import
Version: 0.0.1
Summary: Bellande Importer of Rust Module
sHome-page: github.com/Architecture-Mechanism/bellande_rust_import
Author: Ronaldson Bellande
Author-email: ronaldsonbellande@gmail.com
License: GNU General Public License v3.0
```

## License

BellandeOS Scripting Language is distributed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html), see [LICENSE](https://github.com/Architecture-Mechanism/bellandeos/blob/main/LICENSE) and [NOTICE](https://github.com/Architecture-Mechanism/bellandeos/blob/main/LICENSE) for more information.
