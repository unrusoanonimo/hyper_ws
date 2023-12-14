use derivative::Derivative;
use http::Response;
use hyper::Body;

use crate::{
    util::{AppError, ExtendedRequest},
    ModulesSendable,
};

use super::PreroutingResolution;
pub enum FilterAction {
    None,
    Return(Response<Body>),
    Modify(Box<dyn FnOnce(&mut Response<Body>) -> Result<(), AppError>>),
}
pub trait HttpRequestFilter: Send + Sync + 'static {
    fn filter(
        &self,
        req: &ExtendedRequest,
        modules: &mut ModulesSendable,
    ) -> Result<FilterAction, AppError>;
}

#[derive(Derivative, impl_new::New)]
#[derivative(Debug)]
pub struct FilterModule {
    #[derivative(Debug = "ignore")]
    list: Vec<Box<dyn HttpRequestFilter>>,
}
impl FilterModule {
    pub fn add(&mut self, filter: impl HttpRequestFilter) {
        self.list.push(Box::new(filter));
    }
    pub fn evaluate_filters(
        &self,
        req: &ExtendedRequest,
        modules: &mut ModulesSendable,
    ) -> Result<PreroutingResolution, AppError> {
        let mut modifiers: Vec<Box<dyn FnOnce(&mut Response<Body>) -> Result<(), AppError>>> =
            vec![];
        for v in self.list.iter() {
            match v.filter(req, modules)? {
                FilterAction::Modify(f) => modifiers.push(f),
                FilterAction::None => (),
                FilterAction::Return(r) => return Ok(PreroutingResolution::Return(r)),
            };
        }
        Ok(PreroutingResolution::Modify(modifiers))
    }
}

impl Default for FilterModule {
    fn default() -> Self {
        Self::new([])
    }
}
