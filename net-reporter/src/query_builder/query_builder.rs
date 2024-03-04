pub struct QueryBuilder {
    query: String,
    option_counter: usize,
}

impl QueryBuilder {
    pub fn new(initial_query: &str, option_counter: usize) -> Self {
        QueryBuilder {
            query: initial_query.to_string(),
            option_counter,
        }
    }

    pub fn add_option_filter(mut self, option_filter: Option<&str>, to_replace: usize) -> Self {
        if let Some(option_filter) = option_filter {
            let filter = option_filter.replace("{}", format!("${}", self.option_counter).as_str());
            self.option_counter += 1;
            self.query = self.query.replacen("{}", filter.as_str(), to_replace);
        }
        self
    }

    pub fn build_query(self) -> String {
        self.query
    }
}