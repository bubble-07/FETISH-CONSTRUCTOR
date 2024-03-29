use fetish_lib::everything::*;
use std::collections::HashMap;
use crate::feature_space_info::*;
use topological_sort::TopologicalSort;
use crate::params::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializedSpaceInfoDirectory {
    pub feature_spaces : Vec<SerializedFeatureSpaceInfo>
}

impl SerializedSpaceInfoDirectory {
    pub fn deserialize(mut self) -> SpaceInfoDirectory {
        let mut feature_spaces = Vec::new();
        for serialized_feature_space_info in self.feature_spaces.drain(..) {
            let feature_space_info = serialized_feature_space_info.deserialize();
            feature_spaces.push(feature_space_info);
        }
        SpaceInfoDirectory {
            feature_spaces
        }
    }
}

pub fn get_default_space_info_directory(params : &Params,
                                        type_info_directory : &TypeInfoDirectory) -> SerializedSpaceInfoDirectory {
    let mut feature_spaces = HashMap::<TypeId, SerializedFeatureSpaceInfo>::new();
    
    let mut topo_sort = TopologicalSort::<TypeId>::new();
    for type_id in 0..type_info_directory.get_total_num_types() {
        match type_info_directory.get_type(type_id) {
            Type::FuncType(arg_type_id, ret_type_id) => {
                topo_sort.add_dependency(arg_type_id, type_id);
                topo_sort.add_dependency(ret_type_id, type_id);
            },
            Type::VecType(dim) => {
                let feat_space = build_uncompressed_feature_space(params, dim);
                feature_spaces.insert(type_id, feat_space);
            }
        };
    }
    
    while (topo_sort.len() > 0) {
        let mut type_ids : Vec<TypeId> = topo_sort.pop_all();
        for func_type_id in type_ids.drain(..) {
            if let Type::FuncType(arg_type_id, ret_type_id) = type_info_directory.get_type(func_type_id) {
                let in_feat_info = feature_spaces.get(&arg_type_id).unwrap();
                let out_feat_info = feature_spaces.get(&ret_type_id).unwrap();
                let func_feat_info = build_function_feature_space(params, in_feat_info, out_feat_info);

                feature_spaces.insert(func_type_id, func_feat_info);
            }
        }
    }

    let mut vectorized_feature_spaces = Vec::new();
    for type_id in 0..type_info_directory.get_total_num_types() {
        let feat_space = feature_spaces.remove(&type_id).unwrap();
        vectorized_feature_spaces.push(feat_space);
    }
    SerializedSpaceInfoDirectory {
        feature_spaces : vectorized_feature_spaces
    }
}
