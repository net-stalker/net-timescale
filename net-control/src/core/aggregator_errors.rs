#[derive(PartialEq, Debug, Clone)]
pub enum AggregatorError {
    AggregationError(AggregationError),
    AddingClientError(AddingClientError),
    StatusIdentifyingError(StatusIdentifyingError),
    GetBufferError(GetBufferError),
    ErasingArror(ErasingArror),
}

#[derive(PartialEq, Debug, Clone)]
pub enum AggregationError {
    ClientNotExist(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum AddingClientError {
    ClientAlreadyConnected(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum StatusIdentifyingError {
    ClientNotExist(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum GetBufferError {
    ClientNotExist(u64),
    ClientMsgIsNotEnded(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErasingArror {
    ClientNotExist(u64),
}