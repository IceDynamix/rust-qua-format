# qua_format

[![issues](https://img.shields.io/github/issues/IceDynamix/rust-qua-format)](https://github.com/IceDynamix/rust-qua-format/issues) [![license](https://img.shields.io/crates/l/qua_format)](https://github.com/IceDynamix/rust-qua-format/blob/main/LICENSE) [![version](https://img.shields.io/crates/v/qua_format)](https://crates.io/crates/qua_format) [![documentation](https://img.shields.io/docsrs/qua_format)](https://docs.rs/qua_format/0.1.0/qua_format/)

[Documentation](https://docs.rs/qua_format/0.1.0/qua_format/)

Parse .qua files into structs, based on the .qua format of
[Quaver](https://quavergame.com/). The .qua file format uses the YAML format, so
`serde_yaml` is used for parsing.

# Example

```rust
use qua_format::Qua;
use std::fs::File;

let path = "123.qua";
let mut qua = Qua::from_file(path).unwrap();

qua.title = "Never Gonna Give You Up".to_string();

let new_file = File::create("test.qua").unwrap();
qua.to_writer(new_file).unwrap();
```
