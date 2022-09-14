/// Do not use
/// ```ignore
/// director::off!(state: Self, ..)
/// ```
/// This must be a dead lock.
#[macro_export]
macro_rules! off {
    (state: $state:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
            match director::___::paste! { $state ::lock() }.get_option() {
                $( $pattern )|+ $( if $guard )? => false,
                _ => true
            }

    };
    ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        !matches!($expression, $( $pattern )|+ $( if $guard )?)
    };
}

/// Do not use
/// ```ignore
/// director::on!(state: Self, ..)
/// ```
/// This must be a dead lock.
#[macro_export]
macro_rules! on {
    (state:$state:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match director::___::paste! { $state ::lock() }.get_option() {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }

    };
    ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        matches!($expression, $( $pattern )|+ $( if $guard )?)
    };
}
