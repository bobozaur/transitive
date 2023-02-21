#[macro_export]
macro_rules! impl_from {
    ($source:ident to $target:ident) => {
        impl From<$source> for $target {
            fn from(val: $source) -> $target {
                $target
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from {
    ($source:ident to $target:ident err $err:ident) => {
        impl TryFrom<$source> for $target {
            type Error = $err;

            fn try_from(val: $source) -> Result<Self, Self::Error> {
                Ok($target)
            }
        }
    };
}
