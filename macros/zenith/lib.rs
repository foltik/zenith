use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Result, ItemFn, Type, parse_quote};
use quote::quote;

use macros::{error, expand, bail};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    expand!(_main(item.into()))
}

fn _main(item: TokenStream2) -> Result<TokenStream2> {
    const INVALID: &str = "expected `async fn main() -> Result<()>`";

    let mut main = syn::parse2::<ItemFn>(item)
        .map_err(|err| error!(err.span(), INVALID))?;

    if main.sig.ident != "main" {
        bail!(main.sig.ident.span(), INVALID);
    }

    if main.sig.asyncness.is_none() {
        bail!(main.sig.span(), INVALID);
    }

    match &main.sig.output {
        syn::ReturnType::Default => bail!(main.sig.output.span(), INVALID),
        syn::ReturnType::Type(_, ty) => match &**ty {
            Type::Path(path) if *path == parse_quote!(Result<()>) || *path == parse_quote!(zenith::Result<()>) => {},
            _ => bail!(ty.span(), INVALID),
        }
    }

    main.sig.ident = parse_quote!(__main);

    Ok(quote! {
        #main
        async fn _main() -> zenith::Result<()> {
            zenith::init().await?;
            __main().await
        }
        fn main() -> zenith::Result<()> {
            zenith::smol::block_on(_main())
        }
    })
}
