use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Expr, Ident, Result};

use crate::{i18n::check_yaml, lang_yaml::LocalizedText};

pub fn literal_only(
    parsed: Punctuated<Expr, Comma>,
    localized_text: &LocalizedText,
    key: &str,
) -> Result<TokenStream> {
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    // キーを2つ以上指定していた場合
    if parsed.len() == 2 {
        return err_return(format!("Key: {} is Not Localized", key));
    }

    let value = match localized_text.get("all") {
        Some(v) => v,
        None => return err_return("Failed to get all key".into()),
    };

    return Ok(quote! { #value });
}

pub fn literal_and_lang(
    parsed: Punctuated<Expr, Comma>,
    localized_text: &LocalizedText,
    yaml_langs: HashSet<String>,
) -> Result<TokenStream> {
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    // 引数が無かったり多すぎたりした場合
    match parsed.len() {
        0 => return err_return("Expected string literal".into()),
        3.. => return err_return("Too many args".into()),
        _ => (),
    };

    // all を含むかで分岐
    let is_containts_all = localized_text
        .iter()
        .any(|(key, _)| key.to_ascii_lowercase() == "all");

    // 第2引数の取得
    let lang_expr = parsed.get(1).unwrap();

    // ソートしてから渡す
    let mut sorted_langs: Vec<_> = yaml_langs.iter().map(|i| i.as_str()).collect();
    sorted_langs.sort();

    let mut idents = vec![];
    let mut strings = vec![];

    if is_containts_all {
        for i in sorted_langs {
            let enum_key = Ident::new(&check_yaml::to_enumval_format(i), Span::call_site());

            // 存在しないキーの場合、allキーのリテラルを適応する
            idents.push(enum_key);
            strings.push(
                localized_text
                    .get(i)
                    .unwrap_or_else(|| localized_text.get("all").unwrap())
                    .as_str(),
            );
        }
    } else {
        // 数が足りているかチェックして足りなければ返す
        if yaml_langs.len() > localized_text.len() {
            let localized_lang: HashSet<_> =
                localized_text.iter().map(|(k, _)| k.clone()).collect();

            let mut missing: HashSet<_> = yaml_langs.difference(&localized_lang).collect();
            missing.remove(&"all".to_string());

            let mut sorted_missing: Vec<_> = missing.iter().collect();
            sorted_missing.sort();

            return err_return(format!("Missing language key: {:?}", sorted_missing));
        }

        for i in sorted_langs {
            let enum_key = Ident::new(&check_yaml::to_enumval_format(i), Span::call_site());

            idents.push(enum_key);
            strings.push(localized_text[i].as_str());
        }
    }

    Ok(quote! {
        {
            use crate::_langrustang_autogen::Lang::*;

            match #lang_expr {
                #( #idents => #strings, )*
            }
        }
    })
}
