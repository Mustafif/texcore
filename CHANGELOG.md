```
diff --git a/Cargo.toml b/Cargo.toml
index 103f329..02b757e 100755
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -1,6 +1,6 @@
 [package]
 name = "texcore"
-version = "0.5.4"
+version = "0.6.0"
 edition = "2021"
 authors = ["Mustafif Khan"]
 description = "Write latex in rust, and either compile it to pdf or output the tex code"
diff --git a/src/element.rs b/src/element.rs
index 397fd01..bda635d 100755
--- a/src/element.rs
+++ b/src/element.rs
@@ -1,3 +1,4 @@
+use std::collections::linked_list::{Iter, IterMut};
 use crate::Level::*;
 use crate::TextType::*;
 use crate::Type::*;
@@ -6,8 +7,8 @@ use crate::*;
 use crate::feature;
 use serde::{Deserialize, Serialize};
 use std::collections::LinkedList;
-use std::fs::File;
-use std::io::{Error, Write};
+use std::fs::write;
+use std::io::Error;
 use std::path::PathBuf;
 feature! {
     #![feature = "compile"]
@@ -386,11 +387,6 @@ pub struct ElementList<T: Tex> {
 }
 
 impl ElementList<Any> {
-    #[cfg(feature = "parallel")]
-    pub fn par_iter(&self) -> impl ParallelIterator<Item=&Element<Any>> {
-        use rayon::prelude::*;
-        self.list.par_iter()
-    }
     /// Creates a new empty list
     pub fn new(metadata: &Metadata) -> Self {
         Self {
@@ -398,6 +394,14 @@ impl ElementList<Any> {
             list: LinkedList::new(),
         }
     }
+    /// A forward iterator of elements in the list
+    pub fn iter(&self) -> Iter<'_, Element<Any>> {
+        self.list.iter()
+    }
+    /// A mutable forward iterator of elements in the list
+    pub fn iter_mut(&mut self) -> IterMut<'_, Element<Any>> {
+        self.list.iter_mut()
+    }
     /// Changes the metadata
     pub fn change_metadata(&mut self, metadata: Metadata) {
         self.metadata = metadata
@@ -436,6 +440,7 @@ impl ElementList<Any> {
     pub fn fpop(&mut self) -> Option<Element<Any>> {
         self.list.pop_front()
     }
+
     /// Walks the list and returns a combined latex string
     pub fn to_latex_string(&self) -> String {
         let mut meta = Vec::new();
@@ -446,12 +451,8 @@ impl ElementList<Any> {
         if self.metadata.maketitle {
             document.push(r"\maketitle".to_owned());
         }
-        for i in self.list.iter() {
-            match i.level {
-                Document => document.push(i.value.to_latex_string()),
-                Packages => packages.push(i.value.to_latex_string()),
-                Meta => meta.push(i.value.to_latex_string()),
-            }
+        for i in self.iter() {
+            iter_push(i, &mut document, &mut packages, &mut meta)
         }
         document.push(r"\end{document}".to_owned());
         let result = vec![meta.join("\n"), packages.join("\n"), document.join("\n")];
@@ -468,12 +469,8 @@ impl ElementList<Any> {
         if self.metadata.maketitle {
             document.push(r"\maketitle".to_owned());
         }
-        for i in self.list.iter() {
-            match i.level {
-                Document => document.push(i.value.to_latex_string()),
-                Packages => packages.push(i.value.to_latex_string()),
-                Meta => meta.push(i.value.to_latex_string()),
-            }
+        for i in self.iter() {
+            iter_push(i, &mut document, &mut packages, &mut meta)
         }
         document.push(r"\end{document}".to_owned());
         let result = vec![meta.join("\n"), document.join("\n")];
@@ -492,11 +489,12 @@ impl ElementList<Any> {
         main: PathBuf,
         structure: PathBuf,
         input: Input,
-    ) -> Result<(), Error> {
+    ) {
         let (main_tex, str_tex) = self.to_latex_split_string(input);
-        write_file(main, main_tex.as_bytes())?;
-        write_file(structure, str_tex.as_bytes())?;
-        Ok(())
+        std::thread::scope(|s| {
+            s.spawn(move || { write_file(main, main_tex.as_bytes()).expect("Couldn't write main file") });
+            s.spawn(move || { write_file(structure, str_tex.as_bytes()).expect("Couldn't write structure file") });
+        })
     }
     feature! {
         #![feature = "parallel"]
@@ -506,6 +504,7 @@ impl ElementList<Any> {
             let latex = pool.install(|| self.to_latex_string());
             pool.install(|| write_file(main, latex.as_bytes()).expect("Couldn't write latex file in pool"));
         }
+        /// A parallel alternate to `write_split()`
         pub fn par_write_split(&self, main: PathBuf, structure: PathBuf, input: Input){
             let pool = ThreadPoolBuilder::default().build().expect("Couldn't build pool");
             let (main_tex, str_tex) = pool.install(|| self.to_latex_split_string(input));
@@ -514,14 +513,19 @@ impl ElementList<Any> {
                 || write_file(structure, str_tex.as_bytes()).expect("Couldn't write structure file in pool")
             );
         }
+        pub fn par_iter(&self) -> impl ParallelIterator<Item=&Element<Any>> {
+            use rayon::prelude::*;
+            self.list.par_iter()
+        }
     }
 
     #[cfg(feature = "compile")]
     /// Compiles the list into a pdf file
     pub fn compile(&self, path: PathBuf) -> Result<(), Error> {
+        use std::fs::File;
         let mut file = File::create(path)?;
         let latex = self.to_latex_string();
-        let pdf = latex_to_pdf(&latex)?;
+        let pdf = latex_to_pdf(latex)?;
         file.write_all(&pdf)?;
         Ok(())
     }
@@ -552,9 +556,17 @@ impl Default for ElementList<Any> {
     }
 }
 
+
 // A helper function to write bytes to a file
 fn write_file(path: PathBuf, bytes: &[u8]) -> Result<(), Error> {
-    let mut file = File::create(path)?;
-    file.write_all(bytes)?;
+    write(path, bytes)?;
     Ok(())
 }
+
+fn iter_push(i: &Element<Any>, document: &mut Vec<String>, packages: &mut Vec<String>, meta: &mut Vec<String>) {
+    match i.level {
+        Document => document.push(i.value.to_latex_string()),
+        Packages => packages.push(i.value.to_latex_string()),
+        Meta => meta.push(i.value.to_latex_string()),
+    }
+}
diff --git a/src/future.rs b/src/future.rs
index 17ccd92..edd3132 100644
--- a/src/future.rs
+++ b/src/future.rs
@@ -20,7 +20,7 @@ impl<'a, T: Tex> TexAsync<'a, T> {
         Self(t)
     }
     /// Takes ownership and returns a future of the LaTeX String
-    fn async_latex_string(self) -> impl Future<Output = String> + Send {
+    fn async_latex_string(self) -> impl Future<Output=String> + Send {
         // get the latex string from the value `T`
         let s = self.0.to_latex_string();
         // turn the string into a `Future` that is immediately ready
@@ -33,7 +33,7 @@ impl<'a, T: Tex> TexAsync<'a, T> {
 }
 
 /// An asynchronous version of `Tex::to_latex_string()`
-pub fn async_latex_string<T: Tex>(t: &T) -> impl Future<Output = String> + Send {
+pub fn async_latex_string<T: Tex>(t: &T) -> impl Future<Output=String> + Send {
     let ta = TexAsync::new(t);
     ta.async_latex_string()
 }
@@ -51,36 +51,29 @@ impl ElementList<Any> {
         let mut meta = Vec::new();
         let mut packages = Vec::new();
         let mut document = Vec::new();
-        let mut list = self.clone();
+        let list = self.clone();
         spawn(async move {
             meta.push(async_latex_string(&list.metadata()).await);
             document.push(r"\begin{document}".to_owned());
             if list.metadata().maketitle {
                 document.push(r"\maketitle".to_owned());
             }
-            while let Some(i) = list.fpop() {
-                match i.level {
-                    Document => document.push(i.async_latex_string().await),
-                    Packages => packages.push(i.async_latex_string().await),
-                    Meta => meta.push(i.async_latex_string().await),
-                }
+            for i in list.iter() {
+                iter_push(i, &mut document, &mut packages, &mut meta).await
             }
 
             document.push(r"\end{document}".to_owned());
-            let mut result = Vec::new();
-            result.push(meta.join("\n"));
-            result.push(packages.join("\n"));
-            result.push(document.join("\n"));
+            let result = vec![meta.join("\n"), packages.join("\n"), document.join("\n")];
             result.join("\n")
         })
-        .await
-        .unwrap()
+            .await
+            .unwrap()
     }
     pub async fn async_latex_split_string(&self, input: Input) -> (String, String) {
         let mut meta = Vec::new();
         let mut packages = Vec::new();
         let mut document = Vec::new();
-        let mut list = self.clone();
+        let list = self.clone();
         spawn(async move {
             meta.push(async_latex_string(&list.metadata()).await);
             meta.push(async_latex_string(&input).await);
@@ -88,22 +81,16 @@ impl ElementList<Any> {
             if list.metadata().maketitle {
                 document.push(r"\maketitle".to_owned());
             }
-            while let Some(i) = list.fpop() {
-                match i.level {
-                    Document => document.push(i.async_latex_string().await),
-                    Packages => packages.push(i.async_latex_string().await),
-                    Meta => meta.push(i.async_latex_string().await),
-                }
+            for i in list.iter() {
+                iter_push(i, &mut document, &mut packages, &mut meta).await
             }
 
             document.push(r"\end{document}".to_owned());
-            let mut result = Vec::new();
-            result.push(meta.join("\n"));
-            result.push(document.join("\n"));
+            let result = vec![meta.join("\n"), document.join("\n")];
             (result.join("\n"), packages.join("\n"))
         })
-        .await
-        .unwrap()
+            .await
+            .unwrap()
     }
     /// Asynchronously version of `write()`
     ///
@@ -115,8 +102,8 @@ impl ElementList<Any> {
                 .await
                 .expect("Couldn't write to file");
         })
-        .await
-        .unwrap();
+            .await
+            .unwrap();
         Ok(())
     }
     /// Asynchronous version of `write_split()`
@@ -132,11 +119,10 @@ impl ElementList<Any> {
         let task_m = spawn(async move { write_file(main, main_data.as_bytes()).await });
 
         let task_s = spawn(async move { write_file(structure, str_data.as_bytes()).await });
-        match join!(task_m, task_s) {
-            (r1, r2) => {
-                r1??;
-                r2??;
-            }
+        let (r1, r2) = join!(task_m, task_s);
+        {
+            r1??;
+            r2??;
         }
         Ok(())
     }
@@ -147,3 +133,11 @@ async fn write_file(path: PathBuf, bytes: &[u8]) -> Result<()> {
     file.write_all(bytes).await?;
     Ok(())
 }
+
+async fn iter_push(i: &Element<Any>, document: &mut Vec<String>, packages: &mut Vec<String>, meta: &mut Vec<String>) {
+    match i.level {
+        Document => document.push(i.async_latex_string().await),
+        Packages => packages.push(i.async_latex_string().await),
+        Meta => meta.push(i.async_latex_string().await),
+    }
+}
\ No newline at end of file
diff --git a/src/macros.rs b/src/macros.rs
index a459ae0..9e67d63 100644
--- a/src/macros.rs
+++ b/src/macros.rs
@@ -18,3 +18,18 @@ macro_rules! feature {
         )*
     }
 }
+
+
+/// Provide a way to wrap an item under not a feature
+#[macro_export]
+macro_rules! not_feature {
+    (
+        #![$meta: meta]
+        $($item: item)*
+    ) => {
+        $(
+            #[cfg(not($meta))]
+            $item
+        )*
+    }
+}
\ No newline at end of file
diff --git a/src/template.rs b/src/template.rs
index e0a1c1b..2826a87 100644
--- a/src/template.rs
+++ b/src/template.rs
@@ -1,3 +1,4 @@
+use std::fmt::{Display, Formatter};
 use crate::{Any, Element, ElementList, Input, Metadata, Tex};
 use serde::{Deserialize, Serialize};
 use serde_json::{from_str, to_string_pretty};
@@ -6,7 +7,6 @@ use std::fs::read_to_string;
 use std::io::Result;
 use std::path::PathBuf;
 
-
 /// A TexCreate-template that will be used to store and create TexCreate projects
 #[derive(Debug, Deserialize, Serialize)]
 pub struct Template {
@@ -22,7 +22,7 @@ impl Template {
         Self {
             name: name.to_string(),
             description: description.to_string(),
-            version: Version::new(),
+            version: Version::default(),
             element_list: ElementList::new(metadata),
         }
     }
@@ -33,7 +33,7 @@ impl Template {
     }
     /// Creates a new Template by deserializing a string
     pub fn from_string(content: &str) -> Self {
-        from_str(&content).unwrap()
+        from_str(content).unwrap()
     }
     /// Serializes a Template into a JSON string
     pub fn to_json_string(&self) -> String {
@@ -84,12 +84,12 @@ pub struct Version {
 }
 
 impl Version {
-    /// Creates Template with default `v1.0.0`
-    pub fn new() -> Self {
+    /// Creates a new version using a major, minor and patch values
+    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
         Self {
-            major: 1,
-            minor: 0,
-            patch: 0,
+            major,
+            minor,
+            patch,
         }
     }
     /// Increases major version by 1
@@ -110,8 +110,22 @@ impl Version {
         self.minor = minor;
         self.patch = patch;
     }
-    /// Returns the version as a string: `v.major.minor.patch`
-    pub fn to_string(&self) -> String {
-        format!("v{}.{}.{}", self.major, self.minor, self.patch)
+}
+
+/// Creates Template with default `v1.0.0`
+impl Default for Version {
+    fn default() -> Self {
+        Self {
+            major: 1,
+            minor: 0,
+            patch: 0,
+        }
+    }
+}
+
+impl Display for Version {
+    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
+        let s = format!("v{}.{}.{}", self.major, self.minor, self.patch);
+        f.write_str(&s)
     }
 }
```
