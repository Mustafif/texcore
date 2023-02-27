# TexCore

> This project is under the [MIT License](LICENSE)

TexCore is a library that allows a developer to write `LaTeX` using native `Rust` types. We also provide functions
to compile the code using a Rust built LaTeX compiler, `tectonic`.

To add to your project:

```toml
[dependencies]
texcore = "0.5"
```

### The Compile Feature

To allow this library to be able to run on Windows systems, I have made the `tectonic` dependency optional and only
available under the `compile` feature. This means that the function, `texcore::compile()` and `ElementList::compile()`
are hidden under this feature.

```toml
[dependencies]
texcore = { version = "0.5", features = ["compile"] }
```

### The TexCreate Template Feature

To allow easier development with the TexCreate project, I have decided to add the `texcreate-templates` portion under
the `texcreate_template` feature.

```toml
texcore = { version = "0.5", features = ["texcreate_template"] }
```

### The Async Feature

This feature provides asynchronous options using the type, `TexAsync` which isn't a trait but a generic struct that
requires `T` to implement `Tex`. The reason of not using a trait is that asynchronous methods in a trait isn't stable
yet.
Most importantly we get the following functions from the `future` module:

- `async_latex_string<T: Tex>(t: &T) -> impl Future<Output=String> + Send`
- `Element<Any>::async_latex_string()`
- `ElementList::async_latex_string()`
- `ElementList::async_latex_split_string()`
- `ElementList::async_write()`
- `ElementList::async_split_write()`

Advantages of using the asynchronous writing operations is because they are done so concurrently, and in terms of
`async_split_write()`, the task of writing to each file is done in parallel.

```toml
texcore = { version = "0.5", features = ["async"] }
```

### The Full Feature

To enable all features seen above , you may use the `full` feature.

```toml
texcore = { version = "0.5", features = ["full"] }
```

Read documentation [here](https://docs.rs/crate/texcore/latest)


