/// Provide a way to wrap an item under a feature
#[macro_export]
macro_rules! feature {
    (
        $f: literal
        $($item: item)*
    ) => {
        $(
            #[cfg(feature = $f)]
            $item
        )*
    }
}

/// Provide a way to wrap an item under not a feature
#[macro_export]
macro_rules! not_feature {
    (
        $f: literal
        $($item: item)*
    ) => {
        $(
            #[cfg(not(feature = $f))]
            $item
        )*
    }
}
/// A macro to implement the `ExtraOptions` trait to an Element type.
/// The reason we chose to use a declarative macro instead of a derive is simple:
/// - We only require the identifier
/// - The method to_latex_string() and field latex must exist
/// - Removes the need of a separate crate for a procedural macro
#[macro_export]
macro_rules! options_implement {
    ($($id: ident), +) => {
        $(
            impl ExtraOptions for $id{
                fn modify_element(&mut self, options: Vec<Options>) {
                    self.latex = self.to_latex_string();
                     for option in options {
                         self.latex = option.modify(&self.latex);
                     }
                }
            }
        )+
    }
}