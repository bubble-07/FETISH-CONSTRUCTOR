use fetish_lib::everything::*;
use crate::space_info::*;
use crate::params::*;
use crate::type_id::*;
use crate::params::*;
use crate::primitive_directory::*;
use crate::prior_directory::*;

pub fn get_default_context(params : &Params) -> Context {
    let type_info_directory = get_default_type_info_directory(params.dim);
    let prior_directory = get_default_prior_directory(params, &type_info_directory);
    let space_info_directory = get_default_space_info_directory(params, &type_info_directory);
    let primitive_directory =  get_default_primitive_directory(&type_info_directory);
    Context {
        type_info_directory,
        space_info_directory,
        primitive_directory,
        prior_directory
    }
}

