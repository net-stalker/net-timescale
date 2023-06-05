use chrono::{DateTime, Utc};
use postgres::types::ToSql;
use crate::persistence::postgres_query;

pub struct TimeInterval {
    pub start_interval: i64,
    pub end_interval: i64
}


pub struct TimeIntervalQuery<'a> {
    pub raw_query: &'a str,
    pub args: [&'a (dyn ToSql + Sync); 2]
}
impl<'a> TimeIntervalQuery<'a> {
    pub fn new(start: &'a DateTime<Utc>, end: &'a DateTime<Utc>) -> Self {
        TimeIntervalQuery {
            raw_query: "
                SELECT
                    TIME_BUCKET('1 minute', \"frame_time\") AS bucket,
                    src_addr,
                    dst_addr
                FROM captured_traffic
                WHERE frame_time >= $1 AND frame_time <= $2
                GROUP BY bucket, src_addr, dst_addr;
            ",
            args: [
                start,
                end
            ]
        }
    }
}
impl<'a> postgres_query::PostgresQuery<'a> for TimeIntervalQuery<'a> {
    fn get_query_params(&self) -> (&'a str, &[&'a(dyn postgres::types::ToSql + Sync)]) {
        (self.raw_query, &self.args)
    }
}