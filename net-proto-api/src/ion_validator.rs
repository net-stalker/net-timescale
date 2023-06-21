use std::sync::Arc;

use ion_schema::external::ion_rs::element::Element;

pub struct IonSchemaValidator;

impl IonSchemaValidator {
    pub fn validate(data: &[u8], schema: Arc<ion_schema::schema::Schema>) -> bool {
        let owned_elements = Element::read_all(data).unwrap();

        let mut type_ref = schema.get_types();
        
        for owned_element in owned_elements {
            let type_definition = type_ref.next().unwrap();
            let validation_result = type_definition.validate(&owned_element);

            if validation_result.is_err() {
                return false;
            }
        }
        
        true
    }
}

#[macro_export]
macro_rules! generate_schema {
    ($schema_text:expr) => {
        ion_schema::system::SchemaSystem::new(
            vec![Box::new(
                ion_schema::authority::MapDocumentAuthority::new(
                    [("schema", $schema_text)]
                )
            )]
        ).load_schema("schema")
    };
}

#[macro_export]
macro_rules! load_schema {
    ($schemas_root:expr, $schema_id:expr $(,)?) => {
        ion_schema::system::SchemaSystem::new(
            vec![Box::new(
                ion_schema::authority::FileSystemDocumentAuthority::new(
                    std::path::Path::new($schemas_root)
                )
            )]      
        ).load_schema($schema_id)
    };
}


#[cfg(test)]
mod tests {
    use crate::encoder_api::Encoder;
    use crate::envelope::envelope::Envelope;
    use crate::ion_validator::IonSchemaValidator;


    #[test]
    fn schema_generation_test() {
        assert!(generate_schema!(
            r#"
                schema_header::{}

                type::{
                    name: envelope,
                    type: struct,
                    fields: {
                        type: string,
                        data: blob
                    },
                }

                schema_footer::{}
            "#
        ).is_ok());
    }

    #[test]
    fn validation_test() {
        let envelope = Envelope::new("ENVELOPE_TYPE".into(), "ENVELOP_DATA".into());

        let schema = generate_schema!(
            r#"
                schema_header::{}

                type::{
                    name: envelope,
                    type: struct,
                    fields: {
                        type: string,
                        data: blob
                    },
                }

                schema_footer::{}
            "#
        );
        assert!(schema.is_ok());

        assert!(IonSchemaValidator::validate(&envelope.encode(), schema.unwrap()));
    }
}