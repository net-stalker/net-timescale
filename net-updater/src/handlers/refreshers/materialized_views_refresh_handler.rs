use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use component_core::materialized_view::manager::manager::MaterializedViewManager;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_updater_api::api::refreshers::refresh_views::refresh_views_request::RefreshViewsRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;
use crate::core::update_error::UpdateError;

// TODO: implement Debug for MaterializedViewManager 
// #[derive(Debug)]
pub struct MaterializedViewsRefreshHandler {
    views_manager: Box<MaterializedViewManager>,
}

impl MaterializedViewsRefreshHandler {
    pub fn new(views_manager: Box<MaterializedViewManager>) -> Self {
        Self { views_manager }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for MaterializedViewsRefreshHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(UpdateError::WrongUpdatableData(
                self.get_handler_type()
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        match self.views_manager.refresh_views_blocking(&connection_pool).await {
            Ok(()) => Ok(Envelope::new(tenant_id, ResultDTO::get_data_type(), &ResultDTO::new(true, None, None).encode())),
            Err(err) => Err(UpdateError::DbError(self.get_handler_type(), err.into()).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        RefreshViewsRequestDTO::get_data_type().to_string()
    }
}
