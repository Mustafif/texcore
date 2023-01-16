# TexCore

> This project is under the [MIT License](LICENSE)

TexCore is the successor to `tex-rs` and uses linked lists to help walk and sort elements to either
write to tex code or compile to pdf.

To add to your project:

```toml
[dependencies]
texcore = "0.4"
```

### The Compile Feature

To allow this library to be able to run on Windows systems, I have made the `tectonic` dependency optional and only
available under the `compile` feature. This means that the function, `texcore::compile()` and `ElementList::compile()`
are hidden under this feature.

```toml
[dependencies]
texcore = { version = "0.4", features = ["compile"] }
```

### The TexCreate Template Feature

To allow easier development with the TexCreate project, I have decided to add the `texcreate-templates` portion under
the `texcreate_template` feature.

```toml
texcore = { version = "0.4", features = ["texcreate_template"] }
```

### The Full Feature

To enable both the `compile` and `texcreate_template` feature, you may use the `full` feature.

```toml
texcore = { version = "0.4", features = ["full"] }
```

Read documentation [here](https://docs.rs/crate/texcore/latest)

## Changes in 0.4

- `ElementList::new()` has been changed so it needs `&Metadata` as an argument
- Fixed the `From` trait that is implement for all of the `Tex` types for `Element<Any>`

The following functions uses `ElementList` mutably, as well as other described changes:

- `ElementList::to_latex_string()` & `ElementList::to_latex_split_string()` have been modified.
    - No longer use a `sort()` method and instead utilizes `fpop()`.
- The `ElementList::write()` function has been split into two different functions:
    - `ElementList::write()`: Writes to a file given a path, `main`
    - `ElementList::write_split()`: Write to two files by splitting the meta + document and packages level

