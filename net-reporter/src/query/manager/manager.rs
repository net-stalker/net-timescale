#![warn(clippy::module_inception)]

use std::collections::HashMap;
use std::sync::Arc;

use sqlx::Pool;
use sqlx::Postgres;

use net_proto_api::envelope::envelope::Envelope;

use crate::query::requester::Requester;

use super::builder::QueryManagerBuilder;


pub struct QueryManager {
    requesters: HashMap<&'static str, Box<dyn Requester>>
}

impl QueryManager {
    pub fn new(
        requesters: HashMap<&'static str, Box<dyn Requester>>
    ) -> Self {
        Self {
            requesters
        }
    }

    pub fn builder() -> QueryManagerBuilder {
        QueryManagerBuilder::default()
    }

    pub async fn handle_request(&self, enveloped_request: Envelope, connection_pool: Arc<Pool<Postgres>>) -> Result<Envelope, String> {
        let requester = self.requesters.get(enveloped_request.get_type());
        if let None = requester {
            return Err("error: Tere is no such request available".to_string());
        }

        let requester = requester.unwrap().as_ref();

        requester.request(connection_pool, enveloped_request).await
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use std::sync::Arc;
//     use std::time::Instant;

//     use sqlx::Pool;
//     use sqlx::Postgres;

//     use net_proto_api::envelope::envelope::Envelope;

//     use crate::query::requester::Requester;

//     const AMOUNT_OF_REPS: usize = 10e6 as usize;

//     struct ChartGeneratorImpl {
//         data: usize
//     }

//     impl ChartGeneratorImpl {
//         fn new(data: usize) -> Self { Self { data } }
//     }
    
//     #[async_trait::async_trait]
//     impl Requester for ChartGeneratorImpl {
//         async fn request(
//             &self,
//             connection_pool: Arc<Pool<Postgres>>,
//             data: Envelope
//         ) -> Result<Envelope, String> {todo!()}
        
//         async fn get_requesting_type(&self) -> &'static str {todo!()}
//     }

//     struct ArcQueryManager {
//         chart_generators: HashMap<&'static str, Arc<dyn Requester>>
//     }

//     impl ArcQueryManager {
//         fn new(chart_generators: HashMap<&'static str, Arc<dyn Requester>>) -> Self { Self { chart_generators } }
//     }
//     struct BoxQueryManager {
//         chart_generators: HashMap<&'static str, Box<dyn Requester>>
//     }

//     impl BoxQueryManager {
//         fn new(chart_generators: HashMap<&'static str, Box<dyn Requester>>) -> Self { Self { chart_generators } }
//     }

//     #[test]
//     fn box_arc_compare() {
//         let chart_generator_impl_a = ChartGeneratorImpl::new(0);
//         let chart_generator_impl_b = ChartGeneratorImpl::new(1);
//         let chart_generator_impl_c = ChartGeneratorImpl::new(2);
//         let mut arc_hashmap: HashMap<&'static str, Arc<dyn Requester>> = HashMap::new();
//         arc_hashmap.insert("1", Arc::new(chart_generator_impl_a));
//         arc_hashmap.insert("2", Arc::new(chart_generator_impl_b));
//         arc_hashmap.insert("3", Arc::new(chart_generator_impl_c));
//         let arc_generators: Arc<ArcQueryManager> = Arc::new(ArcQueryManager::new(
//             arc_hashmap
//         ));

//         let chart_generator_impl_a = ChartGeneratorImpl::new(0);
//         let chart_generator_impl_b = ChartGeneratorImpl::new(1);
//         let chart_generator_impl_c = ChartGeneratorImpl::new(2);
//         let mut box_hashmap: HashMap<&'static str, Box<dyn Requester>> = HashMap::new();
//         box_hashmap.insert("1", Box::new(chart_generator_impl_a));
//         box_hashmap.insert("2", Box::new(chart_generator_impl_b));
//         box_hashmap.insert("3", Box::new(chart_generator_impl_c));
//         let box_generators: Arc<BoxQueryManager> = Arc::new(BoxQueryManager::new(
//             box_hashmap
//         ));

//         println!("Arc:\n{}", std::mem::size_of::<Arc<ArcQueryManager>>());
//         let now = Instant::now();
//         for _ in 0..AMOUNT_OF_REPS {
//             let gen = arc_generators.clone();
//             let chartgen = gen.as_ref().chart_generators.get("1").unwrap();
//         }
//         println!("Elapsed: {:.2?}", now.elapsed());

//         println!("Box:\n{}", std::mem::size_of::<Arc<BoxQueryManager>>());
//         let now = Instant::now();
//         for _ in 0..AMOUNT_OF_REPS {
//             let gen = box_generators.clone();
//             let chartgen = gen.as_ref().chart_generators.get("1").unwrap();
//         }
//         println!("Elapsed: {:.2?}", now.elapsed());
//     }
// }