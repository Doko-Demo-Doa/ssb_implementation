// Imports
use libc::*;
use ssb_renderer::{
    ssb_parser::data::{Ssb, SsbRender},
    rendering::SsbRenderer,
    image::{ColorType, ImageView},
    types::parameter::RenderTrigger
};
use std::{
    convert::TryFrom,
    error::Error,
    ffi::CStr,
    io::Cursor,
    ptr::null_mut,
    slice::{from_raw_parts, from_raw_parts_mut}
};


// Helpers
fn error_to_c(error: Box<Error>, error_message: *mut c_char, error_message_capacity: c_ushort) {
    if !error_message.is_null() && error_message_capacity > 0 {
        let mut msg = error.to_string();
        msg.truncate((error_message_capacity-1) as usize);
        msg.push('\0');
        unsafe {msg.as_ptr().copy_to(error_message as *mut u8, msg.len());}
    }
}

/// Get library version as C string.
#[no_mangle]
pub extern fn ssb_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Create renderer instance.
/// **script** mustn't be *null*.
/// **error_message** can be *null*.
/// Returns renderer instance or *null*.
#[no_mangle]
pub extern fn ssb_new_renderer(script: *const c_char, error_message: *mut c_char, error_message_capacity: c_ushort) -> *mut c_void {
    match ssb_new_renderer_inner(script) {
        Ok(renderer) => Box::into_raw(Box::new(renderer)) as *mut c_void,
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            null_mut()
        }
    }
}
fn ssb_new_renderer_inner(script: *const c_char) -> Result<SsbRenderer, Box::<Error>> {
    Ok(SsbRenderer::new(
        Ssb::default().parse_owned( Cursor::new(unsafe{ CStr::from_ptr(script) }.to_str()?) )
        .and_then(|ssb| SsbRender::try_from(ssb) )?
    ))
}

/// Destroy renderer instance.
/// **renderer** can be *null*.
#[no_mangle]
pub extern fn ssb_destroy_renderer(renderer: *mut c_void) {
    if !renderer.is_null() {
        unsafe {Box::from_raw(renderer);}
    }
}

/// Render on image.
/// **renderer** can be *null*.
/// **color_type** mustn't be *null*.
/// **planes** mustn't be *null* and contains enough pointers with enough data for given **color_type**.
/// **error_message** can be *null*.
/// Returns 0 on success, 1 on error.
#[no_mangle]
pub extern fn ssb_render(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    time: c_uint,
    error_message: *mut c_char, error_message_capacity: c_ushort
) -> c_int {
    match ssb_render_inner(renderer, width, height, stride, color_type, planes, time) {
        Ok(()) => 0,
        Err(error) => {
            error_to_c(error, error_message, error_message_capacity);
            1
        }
    }
}
fn ssb_render_inner(
    renderer: *mut c_void,
    width: c_ushort, height: c_ushort, stride: c_uint, color_type: *const c_char, planes: *const *mut c_uchar,
    time: c_uint
) -> Result<(), Box<Error>> {
    if !renderer.is_null() {
        unsafe {
            let color_type = ColorType::by_name( CStr::from_ptr(color_type).to_str()? )?;
            (*(renderer as *mut SsbRenderer)).render(
                ImageView::new(
                    width,
                    height,
                    stride,
                    color_type,
                    {
                        let min_data_size = height as usize * stride as usize;
                        from_raw_parts(planes, color_type.planes() as usize)
                        .iter()
                        .map(|plane| from_raw_parts_mut(*plane, min_data_size) )
                        .collect()
                    }
                )?,
                RenderTrigger::Time(time)
            )?;
        }
    }
    Ok(())
}