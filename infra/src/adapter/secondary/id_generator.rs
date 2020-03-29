use domain::executor::ports::secondary::IdGeneratorPort;
use uuid::Uuid;

pub struct IdGeneratorAdapter;

impl IdGeneratorPort for IdGeneratorAdapter {
    fn generate_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

impl IdGeneratorAdapter {
    pub fn new() -> IdGeneratorAdapter {
        IdGeneratorAdapter{}
    }
}