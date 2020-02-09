#[macro_use]
extern crate neon;
extern crate steganography;

use neon::prelude::*;
use steganography::encoder::*;
use steganography::util::*;

fn steg(mut cx: FunctionContext) -> JsResult<JsString> {
    let message = cx.argument::<JsString>(0)?.value();
    let input = cx.argument::<JsString>(1)?.value();
    let output = cx.argument::<JsString>(2)?.value();
    let payload = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image(input);
    let enc = Encoder::new(payload, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, output);
    Ok(cx.string("done"))
}

register_module!(mut cx, { cx.export_function("steg", steg) });

// fn args(mut cx: FunctionContext) -> JsResult<JsString> {
//     let arg0 = cx.argument::<JsString>(0)?.value();
//     Ok(cx.string(arg0))
// }

// register_module!(mut cx, { cx.export_function("args", args) });
