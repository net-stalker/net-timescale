use std::path::Path;

use ion_schema::authority::DocumentAuthority;
use ion_schema::authority::FileSystemDocumentAuthority;
use ion_schema::external::ion_rs::element::Element;
use ion_schema::system::SchemaSystem;

pub struct IonSchemaValidator;

impl IonSchemaValidator {
    pub fn validate(data: &[u8], schema_id: &str) -> bool {
        let owned_elements = Element::read_all(data).unwrap();

        let document_authorities: Vec<Box<dyn DocumentAuthority>> = vec![Box::new(
            FileSystemDocumentAuthority::new(Path::new(".isl")),
        )];
        let mut schema_system = SchemaSystem::new(document_authorities);
        let schema = schema_system.load_schema(schema_id).unwrap();

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