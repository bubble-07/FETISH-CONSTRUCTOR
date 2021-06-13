use fetish_lib::everything::*;
use crate::params::*;

//TODO: Verify that this is all valid

fn prior_sigma(params : &PriorParams) -> f32 {
    (params.out_covariance_multiplier / params.in_precision_multiplier).sqrt()
}

pub fn sketch_alpha(params : &Params, embedding_dim : usize) -> f32 {
    (1.0f32 / prior_sigma(&params.term_prior_params)) * (2.0f32 / (embedding_dim as f32)).sqrt()
}

pub fn linear_sketched_alpha(params : &Params, full_dim : usize, sketch_dim : usize) -> f32 {
    params.lin_importance * (2.0f32 / prior_sigma(&params.term_prior_params)) * 
                            (1.0f32 / ((full_dim * sketch_dim) as f32).sqrt())
}

pub fn fourier_sketched_alpha(params : &Params, fourier_feats : usize) -> f32 {
    params.fourier_importance * (1.0f32 / (fourier_feats as f32).sqrt())
}

pub fn quadratic_sketched_alpha(params : &Params, full_dim : usize) -> f32 {
    let prior_sigma_val = prior_sigma(&params.term_prior_params);
    params.quad_importance * (2.0f32 / ((full_dim as f32) * prior_sigma_val * prior_sigma_val))
}
