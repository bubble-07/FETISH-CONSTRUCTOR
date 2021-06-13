use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub cauchy_scaling : f32,
    pub fourier_coverage_multiplier : usize,
    pub fourier_importance : f32,
    pub quad_padding_multiplier : usize,
    pub quad_importance : f32,
    pub lin_importance : f32,
    pub dim : usize,
    pub dim_taper_start : usize,
    pub elaborator_prior_params : PriorParams,
    pub term_prior_params : PriorParams
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriorParams {
    pub error_covariance_prior_observations_per_dimension : f32,
    pub out_covariance_multiplier : f32,
    pub in_precision_multiplier : f32
}

pub fn log_tapered_linear(k : usize, x : usize) -> usize {
    if (x < k) {
        x
    } else {
        let k_float = k as f32;
        let x_float = x as f32;
        let result_float = (x_float / k_float).ln() * k_float + k_float;
        let ret = result_float.ceil() as usize;
        ret
    }
}

pub fn get_reduced_dimension(params : &Params, full_dim : usize) -> usize {
    log_tapered_linear(params.dim_taper_start, full_dim)
}

pub fn num_fourier_features(params : &Params, in_dimension : usize) -> usize {
    params.fourier_coverage_multiplier * get_reduced_dimension(params, in_dimension)
}
 
pub fn num_quadratic_features(params : &Params, in_dimension : usize) -> usize {
    let demanded_dims : usize = params.quad_padding_multiplier * in_dimension * in_dimension;
    get_reduced_dimension(params, demanded_dims)
}

pub fn num_sketched_linear_features(params : &Params, in_dimension : usize) -> usize {
    get_reduced_dimension(params, in_dimension)
}
