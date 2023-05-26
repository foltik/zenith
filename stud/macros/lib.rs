use bigmacs::expand;
use proc_macro::TokenStream;

/////////////////// Args ///////////////////

#[cfg(feature = "args")]
mod args;

#[cfg(feature = "args")]
#[proc_macro_derive(Parser)]
pub fn derive_parser(item: TokenStream) -> TokenStream {
    expand!(args::derive_parser(item.into()))
}
