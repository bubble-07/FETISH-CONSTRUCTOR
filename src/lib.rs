#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(improper_ctypes_definitions)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use crate::params::*;
use crate::context::*;
use fetish_lib::everything::*;
use std::str;

pub mod prior_directory;
pub mod alpha_formulas;
pub mod context;
pub mod elaborator;
pub mod feature_collection;
pub mod feature_space_info;
pub mod space_info;
pub mod term_model;
pub mod params;
pub mod type_id;
pub mod primitive_directory;

#[no_mangle]
pub extern "C" fn generate_serialized_context(param_json_bytes : &[u8]) -> Result<Vec<u8>, String> {
    let maybe_param_json = str::from_utf8(param_json_bytes);
    match (maybe_param_json) {
        Result::Ok(param_json) => {
            let maybe_params = serde_json::from_str::<Params>(param_json);
            match (maybe_params) {
                Result::Ok(params) => {
                    let serialized_ctxt = get_default_context(params);
                    let maybe_serialized_vec = bincode::serialize(&serialized_ctxt);
                    match (maybe_serialized_vec) {
                        Result::Ok(serialized_vec) => Result::Ok(serialized_vec),
                        Result::Err(err) => Result::Err(format!("serialization error: {}", err))
                    }
                },
                Result::Err(err) => Result::Err(format!("deserialization error: {}", err))
            }
        },
        Result::Err(err) => Result::Err(format!("utf8 error: {}", err))
    }
}

#[no_mangle]
pub extern "C" fn deserialize_serialized_context(context_bytes : &[u8]) -> Result<Context, String> {
    let maybe_serialized_ctxt = bincode::deserialize::<SerializedContext>(context_bytes);
    match (maybe_serialized_ctxt) {
        Result::Ok(serialized_ctxt) => Result::Ok(serialized_ctxt.deserialize()),
        Result::Err(err) => Result::Err(format!("deserialization error: {}", err))
    }
}
