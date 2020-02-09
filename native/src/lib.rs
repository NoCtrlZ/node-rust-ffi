#[macro_use]
extern crate neon;
extern crate steganography;

use neon::prelude::*;
use steganography::encoder::*;
use steganography::util::*;

fn steg(mut cx: FunctionContext,) -> JsResult<JsString> {
    let message = "This is a steganography demo!".to_string();
    let payload = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image("example.png".to_string());
    let enc = Encoder::new(payload, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, "hidden_message.png".to_string());
    Ok(cx.string("steg"))
}

register_module!(mut cx, { cx.export_function("steg", steg) });
