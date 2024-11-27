use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::de::value;
use syn::token::Comma;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, ExprLit, Ident, Lit, Result, Token,
};

use crate::lang_yaml::{LangYaml, LocalizedText};
use crate::{YAML_DATA, YAML_LANGS, YAML_PATH};

pub fn _lang_t(tokens: TokenStream) -> TokenStream {
    lang_t_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn lang_t_parse(input: ParseStream) -> Result<TokenStream> {
    let yaml_path = {
        let lock = YAML_PATH.lock().unwrap();
        lock.clone()
    };

    let yaml_langs = {
        let lock = YAML_LANGS.lock().unwrap();
        lock.clone()
    };

    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    // 簡単にリターンできる用のクロージャ
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    // yamldata の取得 i18n が使われていなかったら返す
    let yaml_data = {
        let lock = YAML_DATA.lock().unwrap();

        match lock.as_ref() {
            Some(value) => value.clone(),
            None => {
                return err_return(
                    "langrustang::i18n is not used,  please set the yaml path.".into(),
                )
            }
        }
    };

    match parsed.len() {
        0 => return err_return("Expected string literal".into()),
        3.. => return err_return("Too many args".into()),
        _ => (),
    };

    // 指定された文字列リテラルを取得
    let key = {
        match parsed.get(0) {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            })) => lit_str.value(),

            _ => return err_return("Failed get param".into()),
        }
    };

    // 存在しないキーなら返す
    let Some(localized_text) = yaml_data.get(&key) else {
        return err_return(format!("Unknown Key: {}", key));
    };

    match parsed.len() {
        // リテラルのみの指定の場合
        1 => {
            // all だけかを確かめる
            for (localized_key, _) in localized_text.iter() {
                if !(localized_key.to_ascii_lowercase() == "all") {
                    return err_return(format!("Key: {} is Localized", key));
                }
            }

            let value = match localized_text.get("all") {
                Some(v) => v,
                None => return err_return("Failed to get all key".into()),
            };
            return Ok(quote! { #value });
        }

        // リテラルと言語の指定の場合
        2 => literal_and_lang(parsed, localized_text, yaml_langs),

        _ => unreachable!(),
    }
}

fn literal_and_lang(
    parsed: Punctuated<Expr, Comma>,
    localized_text: &LocalizedText,
    yaml_langs: HashSet<String>,
) -> Result<TokenStream> {
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    // all を含むかで分岐
    let is_containts_all = localized_text
        .iter()
        .any(|(key, _)| key.to_ascii_lowercase() == "all");

    // 第2引数の取得
    let lang_expr = parsed.get(1).unwrap();

    if is_containts_all {
        let mut idents = vec![];
        let mut strings = vec![];

        for i in &yaml_langs {
            let enum_key = Ident::new(&to_enum_elem_format(i), Span::call_site());

            idents.push(enum_key);
            strings.push(
                localized_text
                    .get(i)
                    .unwrap_or_else(|| localized_text.get("all").unwrap()),
            );
        }

        Ok(quote! {
            {
                use crate::_langrustang_gen::Lang::*;

                match #lang_expr {
                    #( #idents => #strings, )*
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

            return err_return(format!("Missing language key: {:?}", missing));
        }

        let mut idents = vec![];
        let mut strings = vec![];

        for i in &yaml_langs {
            let enum_key = Ident::new(&to_enum_elem_format(i), Span::call_site());

            idents.push(enum_key);
            strings.push(localized_text[i].clone());
        }

        Ok(quote! {
            {
                use crate::_langrustang_gen::Lang::*;

                match #lang_expr {
                    #( #idents => #strings, )*
                }
            }
        })
    }
}

// 最初大文字、それ以降小文字に変換
fn to_enum_elem_format(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let mut chars = text.chars();

    let first_char = chars.next().unwrap();
    let first_char = first_char.to_ascii_uppercase();

    let rest: String = chars.collect();
    let rest = rest.to_ascii_lowercase();

    format!("{}{}", first_char, rest)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use crate::i18n::_i18n;

    use super::*;

    #[test]
    #[ignore]
    fn dbg() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _lang_t(quote! { "example2", lang }).to_string();
        dbg!(token);
    }

    #[test]
    fn test_missing() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _lang_t(quote! { "example4", lang }).to_string();
        assert!(token1.contains("Missing language key"));
    }

    #[test]
    fn test_lang_all() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _lang_t(quote! { "example1" }).to_string();
        let token2 = quote! { "ALL_EXAMPLE" }.to_string();

        assert_eq!(token1, token2)
    }
}
