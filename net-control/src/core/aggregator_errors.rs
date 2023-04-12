use std::fmt::Display;

//TODO: Get rid of the public fields


#[derive(PartialEq, Debug, Clone)]
pub struct AggregatorError {
    pub kind: AggregatorErrorKind,
    pub context: AggregatorErrorContext,
}

impl Display for AggregatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("ERROR:\n\tKIND: [{}]\n\tCONTEXT: [{}]", self.kind.to_string(), self.context.to_string()).fmt(f)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AggregatorErrorKind {
    ClientNotExist,
    ClientMsgIsNotEnded,
    ClientAlreadyExists,
}

impl ToString for AggregatorErrorKind {
    fn to_string(&self) -> String {
        match self {
            AggregatorErrorKind::ClientNotExist => String::from("Client is not exists"),
            AggregatorErrorKind::ClientMsgIsNotEnded => String::from("Client msg is empty or not exists"),
            AggregatorErrorKind::ClientAlreadyExists => String::from("Client already exists"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct  AggregatorErrorContext {
    pub context: AggregationContext,
    pub user: u64
}

impl ToString for AggregatorErrorContext {
    fn to_string(&self) -> String {
        format!("Error while: {}. Current user is: {}", self.context.to_string(), self.user)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AggregationContext {
    AggregationError,
    AddingClientError,
    StatusIdentifyingError,
    GetBufferError,
    ErasingArror,
}

impl ToString for AggregationContext {
    fn to_string(&self) -> String {
        match self {
            AggregationContext::AggregationError => String::from("Aggregation"),
            AggregationContext::AddingClientError => String::from("Adding client"),
            AggregationContext::StatusIdentifyingError => String::from("Status identifying"),
            AggregationContext::GetBufferError => String::from("Getting buffer"),
            AggregationContext::ErasingArror => String::from("Erasing buffer"),
        }
    }
}