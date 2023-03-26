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
