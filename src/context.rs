use fetish_lib::everything::*;
use crate::space_info::*;
use crate::params::*;
use crate::type_id::*;
use crate::params::*;
use crate::primitive_directory::*;
use crate::prior_directory::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializedContext {
    params : Params,
    space_info_directory : SerializedSpaceInfoDirectory
}

impl SerializedContext {
    pub fn deserialize(self) -> Context {
        let type_info_directory = get_default_type_info_directory(self.params.dim);
        let prior_directory = get_default_prior_directory(&self.params, &type_info_directory);
        let primitive_directory = get_default_primitive_directory(&type_info_directory);
        Context {
            type_info_directory,
            space_info_directory : self.space_info_directory.deserialize(),
            primitive_directory,
            prior_directory
        }
    }
}

pub fn get_default_context(params : Params) -> SerializedContext {
    let type_info_directory = get_default_type_info_directory(params.dim);
    let space_info_directory = get_default_space_info_directory(&params, &type_info_directory);
    SerializedContext {
        params,
        space_info_directory
    }
}

