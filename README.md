# langrustang

[日本語の説明はこちら](./README_ja.md)

Multilingual Support for Rust Program

Multilingual support is possible based on yaml.

(Not distributed on crates.io)

## How to write yaml

The yaml file is written in the following format:

```yaml
Any_key:
  all:
  any_lang_key_1: XXX
  any_lang_key_2: YYY
  any_lang_key_3: ZZZ

Any_key2:
  ...
```

Any_key can contain any characters.

The first character of any_lang_key must be a lowercase alphabet,

and the second and subsequent characters must be lowercase alphabets, numbers, or underscores.

And the last character of the language key cannot be an underscore.

## Examples

#### lang.yaml:
```yaml
lang_t_ex1:
  all: ALL_EXAMPLE

lang_t_ex2:
  ja: おはよう
  en: hello!
  zh: 你好
  anykey1: Any?
  some_key_2: Some!

lang_t_ex3:
  all: ALL

  ja: おはよう
  en: hello!
  zh: 你好

lang_t_ex4:
  ja: おはよう
  en: hello!
  zh: 你好

format_t_ex1:
  all: "Hi, {}! "

format_t_ex2:
  all: "{}, ALL!"
  ja: "{}, おはよう!"
  en: "{}, Hello!"
  zh: "{}, 你好!"
```

#### main.rs:
```rust
langrustang::i18n!("lang.yaml");  // Auto-generate `_langrustang_autogen::Lang`
use crate::_langrustang_autogen::Lang;

use langrustang::{lang_t, println_t};

fn main() {
    // The enum elements are automatically generated based on the yaml keys.
    let lang_en = Lang::En;
    let lang_ja = Lang::Ja;
    let lang_zh = Lang::Zh;
    let lang_anykey1 = Lang::Anykey1;
    let lang_somekey2 = Lang::SomeKey2;

    println!("{}", lang_t!("lang_t_ex1")); // ALL_EXAMPLE

    println!("{}", lang_t!("lang_t_ex2", lang_en)); // hello!
    println!("{}", lang_t!("lang_t_ex2", lang_ja)); // おはよう
    println!("{}", lang_t!("lang_t_ex2", lang_zh)); // 你好

    println!("{}", lang_t!("lang_t_ex3", lang_en)); // hello!
    println!("{}", lang_t!("lang_t_ex3", lang_anykey1)); // ALL

    // println!("{}", lang_t!("lang_t_ex4", lang_anykey1)); // Missing language key: ["any_key1", "some_key_2"]

    let name = "Ferris";
    println_t!("format_t_ex1", name);
    println_t!("format_t_ex2", lang_ja, name);
}
```

## lang_t!

- Example 1

If the language key is just 'all', then it will be retrieved using only the key.

```rust
println!("{}", lang_t!("lang_t_ex1")); // ALL_EXAMPLE
```

- Example 2

If there are other keys present besides `all`,

we use the auto-generated Lang Enum to extract them.

```rust
println!("{}", lang_t!("lang_t_ex2", lang_en)); // hello!
println!("{}", lang_t!("lang_t_ex2", lang_ja)); // おはよう
println!("{}", lang_t!("lang_t_ex2", lang_zh)); // 你好
```

- Example 3

If a language key is missing, specifying key will result in a compilation error.

However, if the `all` key is specified, that value will be used instead.

```rust
println!("{}", lang_t!("lang_t_ex3", lang_en)); // hello!
println!("{}", lang_t!("lang_t_ex3", lang_anykey1)); // ALL

// println!("{}", lang_t!("lang_t_ex4", lang_en)); // Compile Error!
```

## format_t!, print_t!, println_t!

If you only have the `all` key, pass the YAML key first and

the formatting arguments from the second onwards, just like a normal format.

If a language key is specified, pass the yaml key as the first argument,

the Lang Enum as the second argument, and then the thing you want to format.

```rust
let name = "Ferris";

println_t!("format_t_ex1", name); // Hi, Ferris!
println_t!("format_t_ex2", lang_ja, name); // Ferris, おはよう!
```

## error even though the code is correct

If you get an error in vscode even though your code is correct,

restarting rust-analyzer will resolve the issue.