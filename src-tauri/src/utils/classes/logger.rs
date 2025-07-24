pub struct CommunicationLogger;

//TODO: frontend message
//TODO: singleton pattern

impl CommunicationLogger {
    pub fn success(&self, message: &str) {}

    pub fn warning(&self, message: &str) {}

    pub fn info(&self, message: &str) {}

    pub fn error(&self, message: &str) {
        println!("{:?}", message);
    }
}

pub static LOGGER: CommunicationLogger = CommunicationLogger;
