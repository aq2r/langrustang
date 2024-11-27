use std::collections::HashSet;

use super::LangYaml;

/// rust の enum に使える文字列かチェック
pub fn check_yaml(yaml: &LangYaml) -> Result<(), String> {
    let mut lang_keys = HashSet::new();

    for (_, localized) in yaml.iter() {
        for (lang, _) in localized.iter() {
            // 1文字目が ascii alphabet 以外じゃないかどうか確かめておく
            let mut chars = lang.chars();

            match chars.nth(0) {
                Some(c) if c.is_ascii_alphabetic() => c.to_ascii_uppercase(),
                Some(_) => return Err("The first character of the language key contains something other than ascii_alphabet.".into()),
                None => return Err("Failed to get char".into()),
            };

            // 残りの文字が ascii alphabet, または ascii numeric または数字、アンダースコか確かめておく
            for c in chars {
                if !(c.is_ascii_alphanumeric() || c == '_') {
                    return Err("Language keys cannot be anything other than ascii_alphabet or ascii_discrit."
                                            .into());
                }
            }

            // すべて小文字、または数字、アンダースコアかチェック
            let mut chars = lang.chars();
            let is_disit_and_lowercase =
                chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

            if !is_disit_and_lowercase {
                return Err("The language key must be in all lowercase.".into());
            }

            // 最後の文字がアンダースコアでないかチェック
            let mut chars = lang.chars().rev();
            if let Some('_') = chars.next() {
                return Err(
                    "The last character of the language key cannot be an underscore.".into(),
                );
            }

            // all は除外する
            if lang != "all" {
                lang_keys.insert(lang);
            }
        }
    }

    Ok(())
}

/// 1文字目は大文字、それ以降は小文字に変換する、アンダーバーがあればその次の文字を大文字にする
pub fn to_enumval_format(text: &str) -> String {
    let mut chars = text.chars();
    let first_char = match chars.next() {
        Some(c) => c.to_ascii_uppercase(),
        None => return String::new(),
    };

    let mut rest: Vec<char> = vec![];
    let mut is_upper = false;
    for i in chars {
        match i {
            '_' => is_upper = true,

            _ => match is_upper {
                true => {
                    rest.push(i.to_ascii_uppercase());
                    is_upper = false
                }
                false => rest.push(i),
            },
        }
    }

    let rest_str: String = rest.iter().collect();

    format!("{}{}", first_char, rest_str)
}
