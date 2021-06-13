use fetish_lib::everything::*;
use crate::alpha_formulas::*;
use crate::feature_collection::*;
use crate::params::*;

pub fn build_function_feature_space(params : &Params, arg_space : &SerializedFeatureSpaceInfo, 
                                    ret_space : &SerializedFeatureSpaceInfo) -> SerializedFeatureSpaceInfo {
    //Simulates FeatureSpaceInfo#get_sketched_dimensions
    let ret_sketched_dimensions = match (&ret_space.sketcher) {
                                  Option::None => ret_space.base_dimensions,
                                  Option::Some(sketch) => sketch.get_output_dimension()
                                  };
    let base_dimensions = arg_space.feature_dimensions * ret_sketched_dimensions;
    build_compressed_feature_space(params, base_dimensions)
}

pub fn build_compressed_feature_space(params : &Params, base_dimensions : usize) -> SerializedFeatureSpaceInfo {
    let reduced_dimensions = get_reduced_dimension(params, base_dimensions);
    let alpha = sketch_alpha(params, base_dimensions);
    let sketcher = Option::Some(LinearSketch::new(base_dimensions, reduced_dimensions, alpha));

    let feature_collections = get_default_feature_collections(params, reduced_dimensions);
    let feature_dimensions = feature_collections.get_total_feat_dims();
    SerializedFeatureSpaceInfo {
        base_dimensions,
        feature_dimensions,
        feature_collections,
        sketcher
    }
}

pub fn build_uncompressed_feature_space(params : &Params, base_dimensions : usize) -> SerializedFeatureSpaceInfo {
    let feature_collections = get_default_feature_collections(params, base_dimensions);
    let feature_dimensions = feature_collections.get_total_feat_dims();
    SerializedFeatureSpaceInfo {
        base_dimensions,
        feature_dimensions,
        feature_collections,
        sketcher : Option::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedFeatureSpaceInfo {
    base_dimensions : usize,
    feature_dimensions : usize,
    feature_collections : SerializedFeatureCollections,
    sketcher : Option<LinearSketch>
}

impl SerializedFeatureSpaceInfo {
    pub fn deserialize(self) -> FeatureSpaceInfo {
        FeatureSpaceInfo {
            base_dimensions : self.base_dimensions,
            feature_dimensions : self.feature_dimensions,
            feature_collections : self.feature_collections.deserialize(),
            sketcher : self.sketcher
        }
    }
}
