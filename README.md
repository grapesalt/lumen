# Lumen

Lumen is a fast and minimal syntax highlighting library for Rust, powered by [Logos](https://github.com/maciejhirsz/logos). I built this because I couldn't find a syntax highlighting library that didn't force you to bundle 10-15MB of XML files or drag in a ton of dependencies. I just wanted something that was light, simple and fast. I also wanted something that was largely compile time, rather than runtime.

Use Lumen if you want small binaries for things like WASM, embedded systems, or CLI tools and if you're okay writing your own themes. Don't use Lumen if you need 100+ languages right now, want pre-made themes, or need something battle-tested. This is new, so expect some rough edges.

## Adding a new language

Adding a new language is really easy:

```rust
use logos::Logos;
use lumen::token::TokenKind;
use lumen_derive::LumenLanguage;

// The macro handles the rest.
#[derive(Logos, LumenLanguage, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum MyLangToken {
    #[lumen_token(Keyword)]
    #[regex(r"if|else|while")]
    Keyword,

    #[lumen_token(String)]
    #[regex(r#""[^"]*""#)]
    String,

    #[lumen_token(Number)]
    #[regex(r"[0-9]+")]
    Number,
}
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for details.
