mod create_literal;

use proc_macro2::TokenStream;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, ExprLit, Lit, Result, Token,
};

use crate::YAML_DATA;

pub fn _format_t(tokens: TokenStream) -> TokenStream {
    format_t_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

pub fn format_t_parse(input: ParseStream) -> Result<TokenStream> {
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
                    "langrustang::i18n is not used, please set the yaml path.".into(),
                )
            }
        }
    };

    match parsed.len() {
        0 => return Err(Error::new(input.span(), "Expected string literal")),
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

    // 言語キーが all のみかどうか
    let is_allonly_key =
        localized_text.len() == 1 && localized_text.iter().any(|(lang, _)| lang == "all");

    match is_allonly_key {
        true => create_literal::allkey_only(parsed, localized_text),
        false => create_literal::not_allkey_only(parsed, localized_text),
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

        let token = _format_t(quote! { "example1" }).to_string();
        dbg!(token);
    }

    #[test]
    fn allonly_arg_1() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _format_t(quote! { "example1" }).to_string();
        let token2 = quote! { format!("ALL_EXAMPLE") }.to_string();

        assert_eq!(token1, token2)
    }

    #[test]
    fn allonly_arg_2to() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _format_t(quote! { "example1", arg1, arg2 }).to_string();
        let token2 = quote! { format!("ALL_EXAMPLE", arg1, arg2) }.to_string();

        assert_eq!(token1, token2)
    }

    #[test]
    fn lang_arg_1_not_all() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _format_t(quote! { "example5", lang }).to_string();
        assert!(dbg!(token).contains("Missing language key"))
    }

    #[test]
    fn lang_arg_1_all() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _format_t(quote! { "example4", lang }).to_string();
        let token2 = quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match lang {
                    En => format!("ALL"),
                    Ja => format!("おはよう"),
                    Test1 => format!("ALL"),
                    Zh => format!("ALL"),
                }
            }
        }
        .to_string();

        assert_eq!(token1, token2)
    }
    #[test]
    fn lang_arg_2to_all() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _format_t(quote! { "example4", lang, arg1, arg2 }).to_string();
        let token2 = quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match lang {
                    En => format!("ALL", arg1, arg2),
                    Ja => format!("おはよう", arg1, arg2),
                    Test1 => format!("ALL", arg1, arg2),
                    Zh => format!("ALL", arg1, arg2),
                }
            }
        }
        .to_string();

        assert_eq!(token1, token2)
    }

    #[test]
    fn lang_arg_2to_not_all() {
        _i18n(quote! {"files/test_file.yaml"});

        let token1 = _format_t(quote! { "example2", lang, arg1, arg2 }).to_string();
        let token2 = quote! {
            {
                use crate::_langrustang_autogen::Lang::*;

                match lang {
                    En => format!("hello!", arg1, arg2),
                    Ja => format!("おはよう", arg1, arg2),
                    Test1 => format!("TEST1", arg1, arg2),
                    Zh => format!("你好", arg1, arg2),
                }
            }
        }
        .to_string();

        assert_eq!(token1, token2)
    }

    #[test]
    fn expect_str() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _format_t(quote! {}).to_string();
        assert!(dbg!(token).contains("Expected string literal"))
    }

    #[test]
    fn expect_lang() {
        _i18n(quote! {"files/test_file.yaml"});

        let token = _format_t(quote! { "example2" }).to_string();
        assert!(dbg!(token).contains("Expected lang"))
    }
}
