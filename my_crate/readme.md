
Build the crate in dev profile.

```sh
cargo build
```
Build the crate in release profile.

```sh
cargo build --release
```

Write documentation for the crate.

- Use //! for documenting the overall module or crate, typically at the start of the file.
- Use /// for documenting specific items like functions, structs, enums, etc.
- Use markdown syntax for formatting the documentation.

```rust

Open the doc that rust generates for the crate.

```sh
cargo doc --open
```

Run the tests in Doc.

```sh
cargo test
```


Re-export the module from the lib.rs file.  
```rust
pub use lib::module; 
```

Publish the crate to crates.io.

Login to crates.io, verify the account email, then get the API key for login through the command line.
```sh
cargo login <API_KEY>
```
Add metadata to the Cargo.toml file.
```toml
authors = ["author_name <author_email>"]
description = "A description of the crate"
license = "MIT"
```

Then publish the crate to crates.io.
```sh
cargo publish
```

Update the crate in crates.io.
- Change the code in the crate.
- Change the version number in the Cargo.toml file.
```toml
version = "0.2.0"
```
- Publish the crate again.
```sh
cargo publish
```
