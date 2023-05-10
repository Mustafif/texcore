# __TexCore__

![Crates.io](https://img.shields.io/crates/d/texcore)
![Lines of code](https://img.shields.io/tokei/lines/github/mkproj/texcore)
![Crates.io](https://img.shields.io/crates/v/texcore)
![GitHub top language](https://img.shields.io/github/languages/top/MKProj/texcore)

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/mustafif09Q)

> The __TexCore__ library is under the [MIT License](LICENSE)

TexCore is a library that allows a developer to write `LaTeX` using native `Rust` types. We also provide functions
to compile the code using a Rust built LaTeX compiler, `tectonic`.

To add to your project:

```toml
[dependencies]
texcore = "0.7"
```

### The Compile Feature

To allow this library to be able to run on Windows systems, I have made the `tectonic` dependency optional and only
available under the `compile` feature. This means that the function, `texcore::compile()` and `ElementList::compile()`
are hidden under this feature.

```toml
[dependencies]
texcore = { version = "0.7", features = ["compile"] }
```

### The TexCreate Template Feature

To allow easier development with the TexCreate project, I have decided to add the `texcreate-templates` portion under
the `texcreate_template` feature.

```toml
texcore = { version = "0.7", features = ["texcreate_template"] }
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
texcore = { version = "0.7", features = ["async"] }
```

### The Parallel Feature

This features utilizes the `rayon` crate to allow `ElementList` to contain the following functions:

- `ElementList::par_write()`
- `ElementList::par_write_split()`
- `ElementList::par_iter()`
- `ElementList::par_iter_mut()`

```toml
texcore = { version = "0.7", features = ["parallel"] }
```

### The Full Feature

To enable all features seen above , you may use the `full` feature.

```toml
texcore = { version = "0.7", features = ["full"] }
```

### Added Modules in 0.7

- The `bundle` module provides types to easily add into your document:
    - Add images using the `graphicx` package under the module, `texcore::bundle::graphicx`
    - Add mathematical symbols or equations under the module, `texcore::bundle::math`
- Be able to add extra parameters to elements using the `modify_element()` function
    - This is under the `texcore::extra_ops` module

Read documentation [here](https://docs.rs/crate/texcore/latest)


