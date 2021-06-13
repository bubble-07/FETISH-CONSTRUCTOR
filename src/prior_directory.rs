use fetish_lib::everything::*;
use crate::params::*;
use std::collections::HashMap;
use crate::term_model::*;
use crate::elaborator::*;

pub fn get_default_prior_directory(params : &Params,
                                   type_info_directory : &TypeInfoDirectory) -> PriorDirectory {
    let mut priors = HashMap::new();
    for type_id in 0..type_info_directory.get_total_num_types() {
        if (!type_info_directory.is_vector_type(type_id)) {
            let model_prior_specification = TermModelPriorSpecification {
                prior_params : params.term_prior_params.clone()
            };
            let elaborator_prior_specification = ElaboratorPrior {
                prior_params : params.elaborator_prior_params.clone()
            };
            let prior_info = PriorInfo {
                model_prior_specification : Box::new(model_prior_specification),
                elaborator_prior_specification : Box::new(elaborator_prior_specification)
            };
            priors.insert(type_id, prior_info);
        }
    }
    PriorDirectory {
        priors
    }
}
