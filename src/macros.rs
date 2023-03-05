/// Current features available under TexCore
/// - compile
/// - texcreate_template
/// - async
/// - parallel
/// - full

/// Provide a way to wrap an item under a feature
#[macro_export]
macro_rules! feature {
    (
        #![$meta: meta]
        $($item: item)*
    ) => {
        $(
            #[cfg($meta)]
            $item
        )*
    }
}
