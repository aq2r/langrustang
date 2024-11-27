mod create_literal;

use proc_macro2::TokenStream;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, ExprLit, Lit, Result, Token,
};

use crate::{YAML_DATA, YAML_LANGS};

pub fn _lang_t(tokens: TokenStream) -> TokenStream {
    lang_t_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn lang_t_parse(input: ParseStream) -> Result<TokenStream> {
    let yaml_langs = {
        let lock = YAML_LANGS.lock().unwrap();
        lock.clone()
    };

    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    // 簡単にリターンできる用のクロージャ
    let err_return = |s: String| Err(Error::new(parsed.span(), s));

    match parsed.len() {
        0 => return Err(Error::new(input.span(), "Expected string literal")),
        3.. => return Err(Error::new(input.span(), "Too many args")),
        _ => (),
    };

    // yamldata の取得 i18n が使われていなかったら返す
    let yaml_data = {
        let lock = YAML_DATA.lock().unwrap();

        match lock.as_ref() {
            Some(value) => value.clone(),
            None => {
                return err_return(
                    "langrustang::i18n is not used, please set the yaml path.".into(),
                )
            }
        }
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

    // 言語キーが all のみかどうか
    let is_allonly_key =
        localized_text.len() == 1 && localized_text.iter().any(|(lang, _)| lang == "all");

    match parsed.len() {
        // リテラルのみの指定の場合
        1 => {
            // キーが all の時のみ実行
            match is_allonly_key {
                true => create_literal::literal_only(parsed, localized_text, &key),
                false => err_return(format!("Key: {} is Localized", key)),
            }
        }

        // リテラルと言語の指定の場合
        2 => {
            // all キーのみの場合、引数が多すぎるので返す
            match is_allonly_key {
                true => err_return(format!("Key: {} is Not Localized", key)),
                false => create_literal::literal_and_lang(parsed, localized_text, yaml_langs),
            }
        }

        _ => unreachable!(),
    }
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

        let token = _lang_t(quote! { "example1", lang }).to_string();
        dbg!(token);
    }

    #[test]
    fn check_1arg_localized() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _lang_t(quote! { "example2" }).to_string();
        assert!(token.contains("Key: example2 is Localized"));
    }

    #[test]
    fn check_2arg_localized() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _lang_t(quote! { "example1", lang }).to_string();
        assert!(token.contains("Key: example1 is Not Localized"));
    }

    #[test]
    fn test_lang_lang() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _lang_t(quote! { "example2", lang }).to_string();
        let token2 = quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match lang {
                    En => "hello!",
                    Ja => "おはよう",
                    Test1 => "TEST1",
                    Zh => "你好",
                }
            }
        }
        .to_string();

        assert_eq!(token1, token2)
    }

    #[test]
    fn test_missing() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _lang_t(quote! { "example5", lang }).to_string();
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
