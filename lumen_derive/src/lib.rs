use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Meta, parse_macro_input};

#[proc_macro_attribute]
pub fn lumen_token(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    item
}

#[proc_macro_derive(LumenLanguage, attributes(lumen_token))]
pub fn derive_lumen_language(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(input, "LumenLanguage can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut match_arms = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let token_kind = extract_lumen_token_attr(&variant.attrs);
        let token_kind = match token_kind {
            Some(kind) => kind,
            None => {
                return syn::Error::new_spanned(
                    variant,
                    format!(
                        "Variant '{}' is missing #[lumen_token(TokenKind)] attribute",
                        variant_name
                    ),
                )
                .to_compile_error()
                .into();
            }
        };

        let pattern = match &variant.fields {
            Fields::Unit => quote! { #name::#variant_name },
            Fields::Unnamed(_) => quote! { #name::#variant_name (..) },
            Fields::Named(_) => quote! { #name::#variant_name { .. } },
        };

        match_arms.push(quote! {
            #pattern => TokenKind::#token_kind,
        });
    }

    let expanded = quote! {
        impl crate::token::LanguageToken for #name {
            fn to_token(&self) -> crate::token::Token {
                let kind = match self {
                    #(#match_arms)*
                };
                // We can't get the span here, so we return a dummy token
                // The actual span is handled in the highlight function
                crate::token::Token::new(kind, 0..0)
            }
        }

        pub fn highlight(source: &str) -> crate::Tokens {
            use ::logos::Logos;

            let mut tokens = Vec::new();
            let lexer = #name::lexer(source);

            for (token_result, span) in lexer.spanned() {
                if let Ok(token) = token_result {
                    let kind = match token {
                        #(#match_arms)*
                    };
                    tokens.push(crate::token::Token::new(kind, span));
                }
            }

            crate::Tokens::new(tokens, source)
        }
    };

    TokenStream::from(expanded)
}

fn extract_lumen_token_attr(attrs: &[Attribute]) -> Option<syn::Ident> {
    for attr in attrs {
        if !attr.path().is_ident("lumen_token") {
            continue;
        }

        if let Meta::List(meta_list) = &attr.meta {
            let tokens = &meta_list.tokens;
            if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                return Some(ident);
            }
        }
    }

    None
}
