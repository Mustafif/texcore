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

/// Provide a way to wrap an item under not a feature
#[macro_export]
macro_rules! not_feature {
    (
        #![$meta: meta]
        $($item: item)*
    ) => {
        $(
            #[cfg(not($meta))]
            $item
        )*
    }
}

#[macro_export]
macro_rules! async_unstable {
    (
        #![$meta: meta]
        $($item: item)*
    ) => {
        #![feature(async_iterator)]
        feature!{
            #![feature = "async_unstable"]
            $($item)*
        }
    }
}