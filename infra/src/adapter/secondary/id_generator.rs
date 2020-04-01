use domain::executor::ports::secondary::IdGeneratorPort;
use uuid::Uuid;

pub struct UUIDGeneratorAdapter;

impl IdGeneratorPort for UUIDGeneratorAdapter {
    fn generate_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

impl UUIDGeneratorAdapter {
    pub fn new() -> UUIDGeneratorAdapter {
        UUIDGeneratorAdapter {}
    }
}