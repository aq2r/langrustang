# langrustang
Rustの多言語対応用プログラム

yamlを使って多言語対応ができます。

(crates.io での配布はしていません)

<br>

(ほぼ自分用ですが) このクレートを使用するには、

Cargo.toml の dependencies に以下を追記します。

```toml
# バージョンは適宜変更してください
langrustang = { git = "https://github.com/aq2r/langrustang", tag = "v1.1.3" }
```

## How to write yaml

yaml ファイルの書き方:

```yaml
Any_key:
  all:
  any_lang_key_1: XXX
  any_lang_key_2: YYY
  any_lang_key_3: ZZZ

Any_key2:
  ...
```

Any_key は任意の文字を使うことができます。

any_lang_key の最初の文字は小文字のアルファベットである必要があります。

2番目以降の文字は小文字のアルファベット、数字、またはアンダースコアである必要があります。

また、言語キーの最後の文字をアンダースコアにすることはできません。

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
langrustang::i18n!("lang.yaml");  // `_langrustang_autogen::Lang` を自動生成する
use crate::_langrustang_autogen::Lang;

use langrustang::{lang_t, println_t};

fn main() {
    // enum の要素は、yamlに書いたキーによって自動で生成されます
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

言語キーが `all` のみの場合は、そのキーを使用して取得できます。

```rust
println!("{}", lang_t!("lang_t_ex1")); // ALL_EXAMPLE
```

- Example 2

`all` キー以外に他のキーがある場合、

自動生成された Lang Enum を使用して取得します。

```rust
println!("{}", lang_t!("lang_t_ex2", lang_en)); // hello!
println!("{}", lang_t!("lang_t_ex2", lang_ja)); // おはよう
println!("{}", lang_t!("lang_t_ex2", lang_zh)); // 你好
```

- Example 3

足りない言語キーがある場合、キーを指定するとコンパイルエラーになります。

ただし、`all` キーが指定されている場合は、その値が代わりに使用されます。

```rust
println!("{}", lang_t!("lang_t_ex3", lang_en)); // hello!
println!("{}", lang_t!("lang_t_ex3", lang_anykey1)); // ALL

// println!("{}", lang_t!("lang_t_ex4", lang_en)); // Compile Error!
```

## format_t!, print_t!, println_t!

`all` キーだけの場合は、最初に yaml のキーを渡してから、

通常のフォーマットと同じように、2番目以降の format の引数を渡します。

言語キーが他に指定されている場合は、yaml のキーを最初に渡してから、

2番目の引数に Lang Enum を渡して、それから format の引数を渡します。

```rust
let name = "Ferris";

println_t!("format_t_ex1", name); // Hi, Ferris!
println_t!("format_t_ex2", lang_ja, name); // Ferris, おはよう!
```

## コードが正しいのにエラーが発生する

コードが正しいのに vscode でエラーが発生する場合は、

rust-analyzer を再起動すると解決します。