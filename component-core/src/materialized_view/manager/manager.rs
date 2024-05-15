use crate::materialized_view::core::common::MaterializedView;

pub struct NaterializerViewManager {
    materialized_views: Vec<Box<dyn MaterializedView>>
}