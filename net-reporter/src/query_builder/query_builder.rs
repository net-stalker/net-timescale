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

    pub fn add_static_filter<Value>(mut self, option_filter: Option<Value>, filter: &str, to_replace: usize) -> Self {
        if let Some(_) = option_filter {
            let filter = filter.replace("{}", format!("${}", self.option_counter).as_str());
            self.option_counter += 1;
            self.query = self.query.replacen("{}", filter.as_str(), to_replace);
        } else {
            self.query = self.query.replacen("{}", "", to_replace);
        }
        self
    }

    pub fn add_dynamic_filter(
        mut self,
        include: Option<bool>,
        to_replace: usize,
        include_filter: &str,
        exclude_filter: &str, 
    ) -> Self {
        let counter = format!("${}", self.option_counter);
        match include {
            Some(true) => {
                self.query = self.query.replacen(
                    "{}",
                    include_filter.to_string().replace("{}", counter.as_str()).as_str(),
                    to_replace
                );
                self.option_counter += 1;
            },
            Some(false) => {
                self.query = self.query.replacen(
                    "{}",
                    exclude_filter.to_string().replace("{}", counter.as_str()).as_str(),
                    to_replace
                );
                self.option_counter += 1;
            },
            None => {
                self.query = self.query.replacen("{}", "", to_replace);
            }
        }
        self
    }

    pub fn build_query(self) -> String {
        self.query
    }
}