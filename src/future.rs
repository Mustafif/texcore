use std::path::PathBuf;
use futures::Future;
use futures::future::ready;
use tokio::{join, spawn};
use tokio::io::{Result, AsyncWriteExt};
use tokio::fs::File;
use crate::{Any, Input, Tex};
use crate::ElementList;
use crate::Element;
use crate::Level::*;

/// A type to provide asynchronous support to TeX elements
///
/// Requires `T` to implement the `Tex` trait
#[derive(Debug, Clone)]
struct TexAsync<'a, T: Tex>(&'a T);

impl<'a, T: Tex> TexAsync<'a, T> {
    fn new(t: &'a T) -> Self {
        Self(t)
    }
    /// Takes ownership and returns a future of the LaTeX String
    fn async_latex_string(self) -> impl Future<Output=String> + Send {
        // get the latex string from the value `T`
        let s = self.0.to_latex_string();
        // turn the string into a `Future` that is immediately ready
        let future = ready(s);
        // we will need to return the value in `Box` because
        // `impl Future<Output=String` is the same as `Pin<Box<dyn Future<Output=String>>>`
        // we also need to make sure that the future implements `Unpin` and `Send`
        Box::pin(future)
    }
}

/// An asynchronous version of `Tex::to_latex_string()`
pub fn async_latex_string<T: Tex>(t: &T) -> impl Future<Output=String> + Send {
    let ta = TexAsync::new(t);
    ta.async_latex_string()
}

// asynchronous method for Element<Any>
impl Element<Any> {
    pub async fn async_latex_string(&self) -> String {
        async_latex_string(&self.value).await
    }
}

// asynchronous methods for ElementList<Any>
impl ElementList<Any> {
    pub async fn async_latex_string(&self) -> String {
        let mut meta = Vec::new();
        let mut packages = Vec::new();
        let mut document = Vec::new();
        let mut list = self.clone();
        spawn(async move {
            meta.push(async_latex_string(&list.metadata()).await);
            document.push(r"\begin{document}".to_owned());
            if list.metadata().maketitle {
                document.push(r"\maketitle".to_owned());
            }
            while let Some(i) = list.fpop() {
                match i.level {
                    Document => document.push(i.async_latex_string().await),
                    Packages => packages.push(i.async_latex_string().await),
                    Meta => meta.push(i.async_latex_string().await),
                }
            }

            document.push(r"\end{document}".to_owned());
            let mut result = Vec::new();
            result.push(meta.join("\n"));
            result.push(packages.join("\n"));
            result.push(document.join("\n"));
            result.join("\n")
        }).await.unwrap()
    }
    pub async fn async_latex_split_string(&self, input: Input) -> (String, String) {
        let mut meta = Vec::new();
        let mut packages = Vec::new();
        let mut document = Vec::new();
        let mut list = self.clone();
        spawn(async move {
            meta.push(async_latex_string(&list.metadata()).await);
            meta.push(async_latex_string(&input).await);
            document.push(r"\begin{document}".to_owned());
            if list.metadata().maketitle {
                document.push(r"\maketitle".to_owned());
            }
            while let Some(i) = list.fpop() {
                match i.level {
                    Document => document.push(i.async_latex_string().await),
                    Packages => packages.push(i.async_latex_string().await),
                    Meta => meta.push(i.async_latex_string().await),
                }
            }

            document.push(r"\end{document}".to_owned());
            let mut result = Vec::new();
            result.push(meta.join("\n"));
            result.push(document.join("\n"));
            (result.join("\n"), packages.join("\n"))
        }).await.unwrap()
    }
    /// Asynchronously version of `write()`
    ///
    /// Writes a file in a separate thread
    pub async fn async_write(&self, main: PathBuf) -> Result<()> {
        let s = self.async_latex_string().await;
        spawn(async move {
            write_file(main, s.as_bytes()).await.expect("Couldn't write to file");
        }).await.unwrap();
        Ok(())
    }
    /// Asynchronous version of `write_split()`
    ///
    /// Writes the two files concurrently using separate threads running in parallel
    pub async fn async_write_split(&self, main: PathBuf, structure: PathBuf, input: Input) -> Result<()> {
        let (main_data, str_data) = self.async_latex_split_string(input).await;
        let task_m = spawn(async move {
            write_file(main, main_data.as_bytes()).await
        });

        let task_s = spawn(async move {
            write_file(structure, str_data.as_bytes()).await
        });
        match join!(task_m, task_s) {
            (r1, r2) => {
                r1??;
                r2??;
            }
        }
        Ok(())
    }
}

async fn write_file(path: PathBuf, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(bytes).await?;
    Ok(())
}