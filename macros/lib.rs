#[macro_export]
macro_rules! error {
    ($span:expr, $msg:expr) => {{
        use syn::spanned::Spanned;
        syn::Error::new($span.span(), $msg)
    }};
    ($span:expr, $fmt:expr, $($arg:expr),*) => {{
        use syn::spanned::Spanned;
        syn::Error::new($span.span(), format!($fmt, $($arg),*))
    }};
}

#[macro_export]
macro_rules! err {
    ($span:expr, $msg:expr) => {
        Err($crate::error!($span, $msg))
    };
    ($span:expr, $fmt:expr, $($arg:expr),*) => {
        Err($crate::error!($span, $fmt, $($arg),*))
    };
}

#[macro_export]
macro_rules! bail {
    ($span:expr, $msg:expr) => {
        return $crate::err!($span, $msg)
    };
    ($span:expr, $fmt:expr, $($arg:expr),*) => {
        return $crate::err!($span, $fmt, $($arg),*)
    };
}

#[macro_export]
macro_rules! expand {
    ($result:expr) => {
        match $result {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        }
    };
}
