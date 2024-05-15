use crate::materialized_view::core::common::MaterializedView;

use super::manager::MaterializedViewManager;

#[derive(Default)]
pub struct MaterializedViewManagerBuilder {
    materialized_views: Vec<Box<dyn MaterializedView>>
}

impl MaterializedViewManagerBuilder {
    pub fn build(self) -> MaterializedViewManager {
        MaterializedViewManager::new(self.materialized_views)
    }

    pub fn add_materialized_view(
        mut self,
        materialized_view: Box<dyn MaterializedView>
    ) -> Self {
        self.materialized_views.push(materialized_view);
        self
    }
}