# neon-serde

This crate is a utility to easily convert values between a`Handle<JsValue>` from
the [neon](https://github.com/neon-bindings/neon) crate
and any value implementing `serde::{Serialize, Deserialize}`

This is a fork of [katyo/neon-serde](https://github.com/katyo/neon-serde) project.

## Versions support

neon-serde is tested on node `16` and `18`

## Usage

#### `neon_serde::from_value`

Convert a `Handle<JsValue>` to a type implementing `serde::Deserialize`

#### `neon_serde::to_value`

Convert a value implementing `serde::Serialize` to a `Handle<JsValue>`

## Example

```rust
use serde::{Serialize, Deserialize};
use neon::prelude::*;

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

fn deserialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let arg0_value: AnObject = match neon_serde::from_value(&mut cx, arg0) {
        Ok(value) => value,
        Err(e) => return Err(e.throw(cx))
    };
    println!("{:?}", arg0_value);

    Ok(JsUndefined::new(&mut cx).upcast())
}

fn serialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let value = AnObject {
        a: 1,
        b: vec![2f64, 3f64, 4f64],
        c: "a string".into()
    };

    neon_serde::to_value(&mut cx, &value)
        .or_else(|e| Err(e.throw(cx)))
}
```

## Limitations

### Data ownership

All Deserialize Values must own all their data (they must have the trait `serde::DererializeOwned`)
