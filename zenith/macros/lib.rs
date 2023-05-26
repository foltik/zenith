use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_quote, ItemFn, Meta, Result, Type};

use stud::bigmac::{bail, error, expand};

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    expand!(_main(args.into(), item.into()))
}

fn _main(args: TokenStream2, item: TokenStream2) -> Result<TokenStream2> {
    const INVALID: &str = "expected `async fn main() -> Result<()>`";

    let mut main = syn::parse2::<ItemFn>(item).map_err(|err| error!(err.span(), INVALID))?;

    if main.sig.ident != "main" {
        bail!(main.sig.ident.span(), INVALID);
    }

    if main.sig.asyncness.is_none() {
        bail!(main.sig.span(), INVALID);
    }

    // TODO: don't require Result
    match &main.sig.output {
        syn::ReturnType::Default => bail!(main.sig.output.span(), INVALID),
        syn::ReturnType::Type(_, ty) => match &**ty {
            Type::Path(path)
                if *path == parse_quote!(Result<()>) || *path == parse_quote!(Result<()>) => {}
            _ => bail!(ty.span(), INVALID),
        },
    }

    main.sig.ident = parse_quote!(__main);

    // parse args
    let args = Punctuated::<Meta, syn::Token![,]>::parse_terminated.parse2(args)?;
    let mut args_ty = None;
    for meta in args {
        if let Meta::NameValue(kv) = meta {
            if kv.path.is_ident("args") {
                args_ty = Some(kv.value);
            }
        }
    }

    let init = match args_ty {
        Some(ty) => quote! {
            let args = zenith::bin::init_args::<#ty>(std::module_path!()).await?;
            __main(args).await?;
        },
        None => quote! {
            zenith::bin::init(std::module_path!()).await?;
            __main().await?;
        },
    };

    Ok(quote! {
        #main
        async fn _main() -> zenith::bin::Result<()> {
            #init
            Ok(())
        }
        fn main() -> zenith::bin::Result<()> {
            zenith::bin::run(_main())
        }
    })
}
