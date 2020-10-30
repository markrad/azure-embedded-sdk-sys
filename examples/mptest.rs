extern crate azure_embedded_sdk_sys as azsys;

use std::str;
use std::ptr;
use std::slice;
use std::process;

fn get_span_from_vector(v: Vec<u8>) -> azsys::az_span {
    let result: azsys::az_span = azsys::az_span {
        _internal: azsys::az_span__bindgen_ty_1 {
            ptr: v.as_ptr() as *mut u8,
            size: v.capacity() as i32,
        }
    };

    result
}

fn main() {
    
    let mut mp: azsys::az_iot_message_properties = azsys::az_iot_message_properties {
        _internal: azsys::az_iot_message_properties__bindgen_ty_1 {
            properties_buffer: azsys::az_span  {
                _internal: azsys::az_span__bindgen_ty_1 {
                    ptr: ptr::null_mut(),
                    size: 0,
                },
            },
            properties_written: 0,
            current_property_index: 0,
        }
    };

    let buffer: Vec<u8> = Vec::with_capacity(200);

    let rc = unsafe { azsys::az_iot_message_properties_init(&mut mp, get_span_from_vector(buffer), 0) };

    if rc != azsys::az_result_core_AZ_OK {
        process::exit(4);
    }
}