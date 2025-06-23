pub struct CommunicationLogger;

//TODO: frontend message

impl CommunicationLogger {
    pub fn success(&self, message: &str) {}

    pub fn warning(&self, message: &str) {}

    pub fn info(&self, message: &str) {}

    pub fn error(&self, message: &str) {
        println!("{:?}", message);
    }
}

pub static LOGGER: CommunicationLogger = CommunicationLogger;
