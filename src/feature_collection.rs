use fetish_lib::everything::*;
use crate::params::*;
use crate::alpha_formulas::*;
use ndarray::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

fn gen_cauchy_random(params : &Params) -> (impl Fn(&mut ThreadRng, usize) -> Array1<f32>) {
    let cauchy_scaling = params.cauchy_scaling;
    move | rng : &mut ThreadRng, dims : usize | generate_cauchy_random(rng, cauchy_scaling, dims)
}

pub fn get_default_feature_collections(params : &Params,
                                       in_dimensions : usize) -> SerializedFeatureCollections {
    let quadratic_feats = num_quadratic_features(params, in_dimensions);
    let fourier_feats = num_fourier_features(params, in_dimensions);
    let linear_feats = num_sketched_linear_features(params, in_dimensions);

    let linear_alpha = linear_sketched_alpha(params, in_dimensions, linear_feats);
    let fourier_alpha = fourier_sketched_alpha(params, fourier_feats);
    let quadratic_alpha = quadratic_sketched_alpha(params, in_dimensions);

    let fourier_generator = gen_cauchy_random(params);

    let linear_feature_collection = SketchedLinearFeatureCollection::new(in_dimensions, linear_feats, linear_alpha);
    let quadratic_collection = QuadraticFeatureCollection::new(in_dimensions, quadratic_feats, quadratic_alpha);
    let fourier_feature_collection = FourierFeatureCollection::new(in_dimensions, fourier_feats, 
                                                           fourier_alpha, fourier_generator);

    SerializedFeatureCollections {
        linear_feature_collection,
        quadratic_feature_collection : quadratic_collection.serialize(),
        fourier_feature_collection
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedFeatureCollections {
    linear_feature_collection : SketchedLinearFeatureCollection,
    quadratic_feature_collection : SerializableQuadraticFeatureCollection,
    fourier_feature_collection : FourierFeatureCollection
}

impl SerializedFeatureCollections {
    pub fn deserialize(self) -> Vec<Box<dyn FeatureCollection>> {
        vec![Box::new(self.linear_feature_collection),
             Box::new(self.quadratic_feature_collection.deserialize()),
             Box::new(self.fourier_feature_collection)]
    }
    pub fn get_total_feat_dims(&self) -> usize {
        self.linear_feature_collection.get_dimension() +
        self.quadratic_feature_collection.get_dimension() +
        self.fourier_feature_collection.get_dimension()
    }
}
