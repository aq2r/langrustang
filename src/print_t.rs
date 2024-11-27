use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{parse::Parser, Error};

use crate::format_t::format_t_parse;

pub fn _print_t(tokens: TokenStream) -> TokenStream {
    let tokens = format_t_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error);

    format_into_print(tokens)
}

// 再帰的に format! を print! に変換
fn format_into_print(tokens: TokenStream) -> TokenStream {
    let output = tokens.into_iter().map(|token| match token {
        TokenTree::Group(group) => {
            let stream = format_into_print(group.stream().into()).into();
            TokenTree::Group(Group::new(group.delimiter(), stream))
        }

        TokenTree::Ident(ident) if ident == "format" => {
            TokenTree::Ident(Ident::new("print", ident.span()))
        }

        token => token,
    });

    TokenStream::from(quote! { #(#output)* })
}
