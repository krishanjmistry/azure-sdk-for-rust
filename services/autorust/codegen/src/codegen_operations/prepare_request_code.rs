use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::spec::WebVerb;

use super::{function_params::FunctionParams, set_request_param_code::SetRequestParamsCode, web_operation_gen::WebOperationGen};
/// Set all body and parameters for the request.
#[derive(Clone)]
pub struct PrepareRequestCode {
    pub has_param_api_version: bool,
    pub has_param_x_ms_version: bool,
    pub api_version: String,
    consumes: String,
    pub parameters: FunctionParams,
    has_body_parameter: bool,
    verb: WebVerb,
}

impl PrepareRequestCode {
    pub fn new(operation: &WebOperationGen, parameters: &FunctionParams, consumes: String) -> Self {
        // let is_post = operation.0.verb == WebVerb::Post;
        Self {
            has_param_api_version: parameters.has_api_version(),
            has_param_x_ms_version: parameters.has_x_ms_version(),
            api_version: operation.api_version().to_string(),
            consumes,
            parameters: parameters.clone(),
            has_body_parameter: operation.0.has_body_parameter(),
            verb: operation.0.verb.clone(),
        }
    }
}

impl ToTokens for PrepareRequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let api_version = if self.has_param_x_ms_version {
            let version = &self.api_version;
            quote! {
                req.insert_header(azure_core::headers::VERSION, #version);
            }
        } else {
            quote! {}
        };

        // params
        let build_request_params = SetRequestParamsCode {
            content_type: self.consumes.clone(),
            params: self.parameters.clone(),
        };
        // tokens.extend(build_request_params.into_token_stream());

        let body_tokens = if !self.has_body_parameter {
            quote! {
                let req_body = azure_core::EMPTY_BODY;
            }
        } else {
            quote! {}
        };

        // if it is a post and there is no body, set the Content-Length to 0
        let content_length_token = if self.verb == WebVerb::Post && !self.has_body_parameter {
            quote! {
                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
            }
        } else {
            quote! {}
        };

        let verb = verb_to_tokens(&self.verb);
        tokens.extend(quote! {
            pub fn prepare_request(&self) -> azure_core::Result<azure_core::Request> {
                let url = self.url()?;
                let mut req = azure_core::Request::new(url, #verb);
                #build_request_params
                #body_tokens
                #api_version
                #content_length_token
                req.set_body(req_body);
                Ok(req)
            }
        });
    }
}

fn verb_to_tokens(verb: &WebVerb) -> TokenStream {
    match verb {
        WebVerb::Get => quote! { azure_core::Method::Get },
        WebVerb::Post => quote! { azure_core::Method::Post },
        WebVerb::Put => quote! { azure_core::Method::Put },
        WebVerb::Patch => quote! { azure_core::Method::Patch },
        WebVerb::Delete => quote! { azure_core::Method::Delete },
        WebVerb::Options => quote! { azure_core::Method::Option },
        WebVerb::Head => quote! { azure_core::Method::Head },
    }
}
