use std::sync::{LazyLock, OnceLock};

pub struct CommunicationLogger;

//TODO: frontend message
//TODO: singleton pattern

impl CommunicationLogger {
   pub fn instance() -> &'static CommunicationLogger {
        static INSTANCE: OnceLock<CommunicationLogger> = OnceLock::new();
        return INSTANCE.get_or_init(|| CommunicationLogger::new());
    }

    pub fn new() -> Self { 
        return Self;
    }

    pub fn success(&self, message: &str) {
        println!("{:?}", message);
    }

    pub fn warning(&self, message: &str) {}

    pub fn info(&self, message: &str) {}

    pub fn error(&self, message: &str) {
        println!("{:?}", message);
    }
}

pub static LOGGER : LazyLock<&CommunicationLogger>= LazyLock::new(|| CommunicationLogger::instance());
