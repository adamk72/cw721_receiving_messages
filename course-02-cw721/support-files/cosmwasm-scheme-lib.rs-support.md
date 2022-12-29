---
main file: lib.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
#[proc_macro_attribute]
pub fn cw_serde(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = cw_serde::cw_serde_impl(input).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}

```