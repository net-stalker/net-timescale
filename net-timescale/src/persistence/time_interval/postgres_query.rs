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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use crate::persistence::postgres_query::PostgresQuery;

    use super::*;
    #[test]
    fn select_time_interval_query_params() {
        let start = "2020-01-01 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let end = "2020-01-02 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let select_interval_query = TimeIntervalQuery::new(&start, &end);
        let (query, args) = select_interval_query.get_query_params();
        assert_eq!(query,
                   "
                SELECT
                    TIME_BUCKET('1 minute', \"frame_time\") AS bucket,
                    src_addr,
                    dst_addr
                FROM captured_traffic
                WHERE frame_time >= $1 AND frame_time <= $2
                GROUP BY bucket, src_addr, dst_addr;
            "
        );
        assert_eq!(format!("{:?}", args), format!("{:?}", &[&start, &end]));
    }
    #[test]
    fn timestamps_from_i64_test() {
        let start_num: i64 = 1600000000000;
        let end_num: i64 = 1610000000000;
        let start = Utc.timestamp_millis_opt(start_num).unwrap();
        let end = Utc.timestamp_millis_opt(end_num).unwrap();
        assert_eq!(start, "2020-09-13 12:26:40.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap());
        assert_eq!(end, "2021-01-07 06:13:20.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap());
    }
}