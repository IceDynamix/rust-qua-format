# qua_format

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
