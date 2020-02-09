#[macro_use]
extern crate neon;
extern crate steganography;

use neon::prelude::*;
use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;

fn encode(mut cx: FunctionContext) -> JsResult<JsString> {
    let message = cx.argument::<JsString>(0)?.value();
    let input = cx.argument::<JsString>(1)?.value();
    let output = cx.argument::<JsString>(2)?.value();
    let payload = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image(input);
    let enc = Encoder::new(payload, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, output);
    Ok(cx.string("encode"))
}

fn decode(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value();
    let encoded_image = file_as_image_buffer(input);
    let dec = Decoder::new(encoded_image);
    let out_buffer = dec.decode_alpha();
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                        .filter(|b| {
                                            *b != 0xff_u8
                                        })
                                        .collect();
    let message = bytes_to_str(clean_buffer.as_slice());
    Ok(cx.string(message.to_string()))
}

register_module!(mut cx, {
    cx.export_function("encode", encode)?;
    cx.export_function("decode", decode)?;
    Ok(())
});
