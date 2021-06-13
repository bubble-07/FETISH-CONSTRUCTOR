#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use serde_json::*;
use crate::params::*;
use crate::context::*;
use fetish_lib::everything::*;

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

pub fn generate_context_json(param_json : &str) -> Result<String> {
    let params : Params = serde_json::from_str(param_json)?;
    let serialized_ctxt = get_default_context(params);
    serde_json::to_string(&serialized_ctxt)
}

pub fn deserialize_context_json(context_json : &str) -> Result<Context> {
    let serialized_ctxt : SerializedContext = serde_json::from_str(context_json)?;
    Result::Ok(serialized_ctxt.deserialize())
}
