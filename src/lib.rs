use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, ItemFn, punctuated::{Punctuated, Pair}, Token, token::Comma};
use quote::quote;

extern crate proc_macro as pm;

macro_rules! ident_fmt {
    ($lit:literal, $($arg:expr),*) => {
        Ident::new(
            &format!($lit, $($arg),*),
            Span::call_site()
        )
    };
}

#[proc_macro_attribute]
pub fn memoize(_attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let dec = parse_macro_input!(item as ItemFn);
    let static_name = ident_fmt!("__{}_CACHE", dec.sig.ident.to_string().to_uppercase());
    let args = &dec.sig.inputs;
    let sig = &dec.sig;
    let body = dec.block.as_ref();
    let ret = match &sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };

    let vars = Punctuated::<Box<_>, Comma>::from_iter(args.clone().into_iter().map(|a| {
        match a {
            syn::FnArg::Receiver(_) => panic!("#[memoize] macro doesn't support methods"),
            syn::FnArg::Typed(t) => Pair::Punctuated(t.pat, Comma { spans: [Span::call_site()] }),
        }
    }));

    let types = Punctuated::<Box<_>, Comma>::from_iter(args.clone().into_iter().map(|a| {
        match a {
            syn::FnArg::Receiver(_) => panic!("#[memoize] macro doesn't support methods"),
            syn::FnArg::Typed(t) => Pair::Punctuated(t.ty, Comma { spans: [Span::call_site()] }),
        }
    }));

    let out = quote! {
        lazy_static! {
            static ref #static_name: Mutex<HashMap<(#types), #ret>> = HashMap::new().into();
        }
        #sig {
            fn __fn_body(#args) -> #ret #body
                
                let mut lock = #static_name.lock().expect("cache mutex has been poisoned");

                match lock.get(&(#vars)) {
                    Some(v) => v.clone(), 
                    None => {
                        let v = __fn_body(#vars);
                        lock.insert((#vars), v.clone());
                        v
                    }
                }
        } 
    };

    println!("{}", out.to_string());

    out.into()
}

