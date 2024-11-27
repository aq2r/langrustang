use std::{collections::HashSet, path::PathBuf};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    Error, Ident, LitStr, Result,
};

use crate::{
    parse_yaml::{LangYaml, LocalizedText},
    YAML_DATA, YAML_LANGS, YAML_PATH,
};

pub fn _i18n(tokens: TokenStream) -> TokenStream {
    i18n_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn i18n_parse(input: ParseStream) -> Result<TokenStream> {
    let literal: LitStr = input.parse()?;
    let input_filepath = literal.value();

    let err_return = |s: String| Err(Error::new(literal.span(), s));

    // yaml かどうか確認
    if !input_filepath.ends_with(".yaml") {
        return err_return("expected .yaml file path".into());
    }

    // ファイルが存在するか確認
    let pathbuf = PathBuf::from(&input_filepath);
    let display_path = pathbuf.canonicalize().unwrap_or_else(|_| pathbuf.clone());

    if !pathbuf.exists() {
        return err_return(format!("File is not found: {:?}", display_path));
    }

    // yamlパスの設定
    {
        let mut lock = YAML_PATH.lock().unwrap();
        *lock = input_filepath.clone();
    }

    let yaml_string = match std::fs::read_to_string(&input_filepath) {
        Ok(s) => s,
        Err(_) => return err_return(format!("Failed to open file: {:?}", display_path)),
    };

    // 初期化
    let lang_yaml = {
        let mut lock = YAML_DATA.lock().unwrap();

        let yaml: LangYaml = match serde_yaml::from_str(&yaml_string) {
            Ok(yaml) => yaml,
            Err(_) => {
                return err_return(format!("Failed to parse yaml"));
            }
        };

        *lock = Some(yaml.clone());
        yaml
    };

    let mut langs_set = HashSet::new();
    for (_, localized) in lang_yaml.iter() {
        for (lang, _) in localized.iter() {
            if lang.to_ascii_lowercase() != "all" {
                langs_set.insert(lang.clone());
            }
        }
    }

    {
        let mut lock = YAML_LANGS.lock().unwrap();
        *lock = langs_set.clone()
    };

    let mut langs = vec![];
    for i in langs_set {
        let mut chars = i.chars();

        // 1文字目が ascii alphabet 以外じゃないかどうか確かめておく
        let first_char = match chars.nth(0) {
            Some(c) if c.is_ascii_alphabetic() => c.to_ascii_uppercase(),
            Some(_) => return err_return("The first character of the language key contains something other than ascii_alphabet.".into()),
            None => return err_return("Failed to get char".into()),
        };

        // 残りの文字が ascii alphabet, または ascii numeric か確かめておく
        let mut rest: Vec<char> = vec![];
        for c in chars {
            if c.is_ascii_alphanumeric() {
                rest.push(c);
            } else {
                return err_return(
                    "Language keys cannot be anything other than ascii_alphabet or ascii_discrit."
                        .into(),
                );
            }
        }
        let rest_string: String = rest.iter().collect();
        let lang_string = format!("{}{}", first_char, rest_string);

        // All は enum に追加しない
        if lang_string == "All" {
            continue;
        }

        langs.push(lang_string);
    }

    // 被りがないかチェック
    let hashset: HashSet<_> = langs.iter().collect();
    if langs.len() != hashset.len() {
        return err_return("The same key exists in yaml.".into());
    }

    let langs_ident: Vec<Ident> = langs
        .iter()
        .map(|s| Ident::new(s, Span::call_site()))
        .collect();

    Ok(quote! {
        pub mod _langrustang_gen {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum Lang {
                #(
                    #langs_ident,
                )*
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::*;

    #[test]
    #[ignore]
    fn dbg() {
        let token = _i18n(quote! { "files/test_file.yaml" }).to_string();
        dbg!(token);
    }

    #[test]
    fn check_no_exists() {
        let token = _i18n(quote! { "../files/no_exists.yaml" }).to_string();
        assert!(token.contains("File is not found: ") && token.contains("files/no_exists.yaml"));
    }

    #[test]
    fn check_not_yaml() {
        let token = _i18n(quote! { "./not_yaml.jpg" }).to_string();
        assert!(token.contains("expected .yaml file path"));
    }

    #[test]
    fn check_literal() {
        let token = _i18n(quote! { not_literal }).to_string();
        assert!(token.contains("expected string literal"));
    }
}
