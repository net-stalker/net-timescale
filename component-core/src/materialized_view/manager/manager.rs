use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;

use crate::materialized_view::core::common::MaterializedView;

use super::builder::MaterializedViewManagerBuilder;

pub struct MaterializedViewManager {
    materialized_views: Vec<Box<dyn MaterializedView>>
}

impl MaterializedViewManager {
    pub fn new(materialized_views: Vec<Box<dyn MaterializedView>>) -> Self {
        Self {
            materialized_views
        }
    }

    pub fn builder() -> MaterializedViewManagerBuilder {
        MaterializedViewManagerBuilder::default()
    }

    pub async fn create_views(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<(), Error> {
        for view in &self.materialized_views {
            let creation_result = view.create(pool).await;
            if let Err(e) = creation_result {
                return Err(e);
            }
        }

        Ok(())
    }

    pub async fn refresh_views_blocking(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<(), Error> {
        for view in &self.materialized_views {
            let creation_result = view.refresh_blocking(pool).await;
            if let Err(e) = creation_result {
                return Err(e);
            }
        }

        Ok(())
    }

    pub async fn refresh_views_concurrently(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<(), Error> {
        for view in &self.materialized_views {
            let creation_result = view.refresh_concurrently(pool).await;
            if let Err(e) = creation_result {
                return Err(e);
            }
        }

        Ok(())
    }
}