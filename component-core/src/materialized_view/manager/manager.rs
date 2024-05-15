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
            view.create(pool).await?;
        }

        Ok(())
    }

    pub async fn refresh_views_blocking(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<(), Error> {
        for view in &self.materialized_views {
            view.refresh_blocking(pool).await?;
        }

        Ok(())
    }

    pub async fn refresh_views_concurrently(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<(), Error> {
        for view in &self.materialized_views {
            view.refresh_concurrently(pool).await?;
        }

        Ok(())
    }
}