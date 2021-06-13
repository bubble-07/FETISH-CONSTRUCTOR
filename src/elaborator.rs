use fetish_lib::everything::*;
use crate::params::*;

pub struct ElaboratorPrior {
    pub prior_params : PriorParams 
}
impl PriorSpecification for ElaboratorPrior {
    fn get_in_precision_multiplier(&self, _feat_dims : usize) -> f32 {
        self.prior_params.in_precision_multiplier
    }
    fn get_out_covariance_multiplier(&self, out_dims : usize) -> f32 { 
        //We'll directly tinker with the mean covariance schmear's size
        let pseudo_observations = self.get_out_pseudo_observations(out_dims);
        pseudo_observations * self.prior_params.out_covariance_multiplier
    }
    fn get_out_pseudo_observations(&self, out_dims : usize) -> f32 {
        //The +4 is because we need to ensure that we always have
        //a valid covariance schmear for this model. See Wikipedia
        //page on the Inverse-Wishart distribution's variance
        (out_dims as f32) * self.prior_params.error_covariance_prior_observations_per_dimension + 4.0f32
    }
}

