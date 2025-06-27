use crate::ports::ether::ether_error_port::EtherErrorPort;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EtherError {
    pub stack: String,
    pub message: String,
}

impl EtherError {
    pub fn new_unknown() -> Self {
        Self {
            message: String::from("Something went wrong"),
            stack: String::new(),
        }
    }
}

impl EtherErrorPort for EtherError {
    fn message(&self) -> String {
        self.message.to_string()
    }

    fn stack(&self) -> String {
        self.stack.to_string()
    }
}
