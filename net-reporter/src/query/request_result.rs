use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;

use net_core_api::api::result::result::ResultDTO;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequestResult {
    is_ok: bool,
    description: Option<String>,
    response: Option<Envelope>,
}

impl RequestResult {
    pub fn new(
        is_ok: bool,
        description: Option<String>,
        response: Option<Envelope>
    ) -> Self {
        Self {
            is_ok,
            description,
            response
        }
    }

    pub fn error(description: Option<String>) -> Self {
        RequestResult::new(
            false,
            description,
            None
        )
    }

    pub fn ok(description: Option<String>, response: Option<Envelope>) -> Self {
        RequestResult::new(
            true,
            description,
            response
        )
    }

    pub fn enveloped_error(description: Option<String>) -> Envelope {
        RequestResult::error(description).into_envelope()
    }

    fn into_envelope(self) -> Envelope {
        Envelope::new(
            None,
            None,
            ResultDTO::get_data_type(),
            &<RequestResult as Into<ResultDTO>>::into(self).encode()
        )
    }
}

impl From<RequestResult> for ResultDTO {
    fn from(value: RequestResult) -> Self {
        ResultDTO::new(
            value.is_ok,
            value.description.as_deref(),
            value.response
        )
    }
}