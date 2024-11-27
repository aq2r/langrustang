use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Expr, Ident, Result};

use crate::{i18n::check_yaml, lang_yaml::LocalizedText, YAML_LANGS};

/// allキーのみの時の処理
pub fn allkey_only(
    parsed: Punctuated<Expr, Comma>,
    localized_text: &LocalizedText,
) -> Result<TokenStream> {
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    let value = match localized_text.get("all") {
        Some(v) => v,
        None => return err_return("Failed to get all key".into()),
    };

    // リテラルのみならその値を format! に渡す
    match parsed.len() {
        1 => return Ok(quote! { format!( #value ) }),
        2.. => (),
        _ => unreachable!(),
    };

    // 最初のリテラル以外の引数を取得
    let mut args = Vec::new();
    for (i, expr) in parsed.into_iter().enumerate() {
        if i != 0 {
            args.push(expr);
        }
    }

    Ok(quote! { format!( #value, #(#args),* ) })
}

/// allキー以外もあるの時の処理
pub fn not_allkey_only(
    parsed: Punctuated<Expr, Comma>,
    localized_text: &LocalizedText,
) -> Result<TokenStream> {
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    // 言語が指定されていなかった場合
    if parsed.len() == 1 {
        return err_return("Expected lang".into());
    }

    let yaml_langs = {
        let lock = YAML_LANGS.lock().unwrap();
        lock.clone()
    };

    // all を含むかで分岐
    let is_containts_all = localized_text
        .iter()
        .any(|(key, _)| key.to_ascii_lowercase() == "all");

    // format に渡す引数と lang を取得
    let mut args = Vec::new();
    for (i, expr) in parsed.iter().enumerate() {
        if i != 0 {
            args.push(expr);
        }
    }
    let lang_expr = args.remove(0);

    if is_containts_all {
        let mut idents = vec![];
        let mut strings = vec![];

        let mut sorted_yaml_langs: Vec<_> = yaml_langs.iter().collect();
        sorted_yaml_langs.sort();

        for i in sorted_yaml_langs {
            let enum_key = Ident::new(&check_yaml::to_enumval_format(i), Span::call_site());

            // 存在しないキーの場合、allキーのリテラルを適応する
            idents.push(enum_key);
            strings.push(
                localized_text
                    .get(i)
                    .unwrap_or_else(|| localized_text.get("all").unwrap()),
            );
        }

        let match_arms: Vec<_> = idents
            .iter()
            .zip(strings.iter())
            .map(|(i, s)| {
                quote! {
                    #i => format!(#s #(, #args)* )
                }
            })
            .collect();

        Ok(quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match #lang_expr {
                    #(#match_arms),* ,
                }
            }
        })
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
        let mut idents = vec![];
        let mut strings = vec![];

        let mut sorted_yaml_langs: Vec<_> = yaml_langs.iter().collect();
        sorted_yaml_langs.sort();

        for i in sorted_yaml_langs {
            let enum_key = Ident::new(&check_yaml::to_enumval_format(i), Span::call_site());

            // 存在しないキーの場合、allキーのリテラルを適応する
            idents.push(enum_key);
            strings.push(
                localized_text
                    .get(i)
                    .unwrap_or_else(|| localized_text.get("all").unwrap()),
            );
        }

        let match_arms: Vec<_> = idents
            .iter()
            .zip(strings.iter())
            .map(|(i, s)| {
                quote! {
                    #i => format!(#s #(, #args)* )
                }
            })
            .collect();

        Ok(quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match #lang_expr {
                    #(#match_arms),* ,
                }
            }
        })
    }
}
