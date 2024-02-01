# json2lua

Convert JSON to Lua table

<div>
  <a href="https://crates.io/crates/json2lua"><img alt='Version badge' src='https://img.shields.io/crates/v/json2lua.svg'></a>
  <a href="https://crates.io/crates/json2lua"><img alt='Downloads badge' src='https://img.shields.io/crates/d/json2lua.svg'></a>
  <a href="https://crates.io/crates/json2lua"><img alt='License badge' src='https://img.shields.io/crates/l/json2lua.svg'></a>
  <a href="https://docs.rs/json2lua"><img alt="Docs badge" src="https://img.shields.io/docsrs/json2lua"></a>
</div>

## Example:

```rust
use json2lua::parse;

let json = r#"{
  "string": "abc",
  "int": 123,
  "bool": true,
  "null": null
}"#;

let lua = parse(json).unwrap();
// Output:
// {
//   ["string"] = "abc",
//   ["int"] = 123,
//   ["bool"] = true,
//   ["null"] = nil,
// }
```
