use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::none::None;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_updater_api::api::refreshers::refresh_views::refresh_views_request::RefreshViewsRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;
use crate::core::update_error::UpdateError;
use crate::utils::get_materialized_views;
use crate::utils::udpate_materialized_views;

#[derive(Default)]
pub struct MaterializedViewsRefreshHandler {}

impl MaterializedViewsRefreshHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for MaterializedViewsRefreshHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let updatable_type = self.get_handler_type()
            .split('-')
            .collect::<Vec<_>>()
            .join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(UpdateError::WrongUpdatableData(updatable_type.clone()).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(UpdateError::TranscationError(err.to_string()).into()),
        };

        let views_name = match get_materialized_views::get_materialized_views_transaction(&mut transaction).await {
            Ok(views_name) => views_name,
            Err(err) => return Err(UpdateError::DbError(updatable_type, err).into()),
        };

        for view_name in views_name.iter() {
            if let Err(err) =  udpate_materialized_views::udpate_materialized_view_transaction(
                &mut transaction,
                view_name.as_str()
            ).await {
                return Err(UpdateError::DbError(updatable_type, err).into());
            }
        }
        match transaction.commit().await {
            Ok(_) => Ok(Envelope::new(tenant_id, None::get_data_type(), &None::default().encode())),
            Err(err) => Err(UpdateError::TranscationError(err.to_string()).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        RefreshViewsRequestDTO::get_data_type().to_string()
    }
}
