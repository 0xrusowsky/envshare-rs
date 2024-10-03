use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::api::{
    healthcheck::__path_healthcheck_endpoint,
    secrets::{
        endpoints::{__path_get_secret, __path_post_secret},
        types::{Secret, TrackedSecret},
    },
};

// -- OPENAPI TYPES ---------------------------------------

#[derive(OpenApi)]
#[openapi(
paths(
    healthcheck_endpoint,
    get_secret,
    post_secret
),
components(
    schemas(Secret, TrackedSecret, Secret)
),
modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );
        let components = openapi.components.as_mut().unwrap(); // safe because there registered components.
        components.add_security_scheme("bearer_token", scheme)
    }
}
