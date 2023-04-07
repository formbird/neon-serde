#![doc = include_str!("../readme.md")]

#[macro_use]
extern crate serde;

pub mod de;
pub mod errors;
pub mod ser;

#[cfg(feature = "dates")]
pub mod dates;
mod macros;

pub use de::{from_value, from_value_opt};
pub use ser::to_value;

#[cfg(test)]
mod tests {
    #![allow(clippy::let_unit_value)]
    use neon::prelude::*;

    use super::*;

    #[test]
    fn test_it_compiles() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Handle<'j, JsValue> = cx.argument::<JsValue>(0)?;
                from_value(&mut cx, arg)
                    .or_else(|e| cx.throw_error(e.to_string()))
                    .unwrap()
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();
            Ok(result)
        }

        let _ = check;
    }

    #[test]
    fn test_it_compiles_2() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> JsResult<'j, JsValue> {
            let result: () = {
                let arg: Option<Handle<'j, JsValue>> = cx.argument_opt(0);
                from_value_opt(&mut cx, arg)
                    .or_else(|e| cx.throw_error(e.to_string()))
                    .unwrap()
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();
            Ok(result)
        }

        let _ = check;
    }
}
