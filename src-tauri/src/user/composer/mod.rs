// pub mod sma_200_composition;
// use crate::commands::ChartingData;
// use crate::user::composer::sma_200_composition::SMA200Composition;
// use core::fmt;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::error::Error;
// use std::sync::{Arc, LazyLock, Mutex};

// #[derive(Debug)]
// pub struct CompositionError {
//     message: String,
// }

// impl Error for CompositionError {}

// impl CompositionError {
//     fn new(message: &str) -> Self {
//         return CompositionError {
//             message: message.into(),
//         };
//     }
// }

// impl fmt::Display for CompositionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         return write!(f, "Composition Error: {}", self.message);
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum CompositionDataValue {
//     I32(i32),
//     I16(i16),
//     I8(i8),
//     String(String),
//     Bool(bool),
// }

// pub trait IComposition: Send + Sync {
//     fn id(&self) -> &str;
//     fn name(&self) -> &str;
//     fn description(&self) -> &str;
//     fn composition_fields(&self) -> HashMap<&'static str, i8>;
//     fn compose(&mut self) -> Vec<Vec<Option<CompositionDataValue>>>;
//     fn safe_compose(&mut self) -> Result<Vec<Vec<Option<CompositionDataValue>>>, Box<dyn Error>> {
//         let composition_data = self.compose();
//         let composition_fields = self.composition_fields();
//         let composition_fields_length = composition_fields.len();

//         for data_point in &composition_data {
//             if data_point.len() != composition_fields_length {
//                 return Err(Box::new(CompositionError::new(
//                     "Composition data point size not equal to composition fields size",
//                 )));
//             }
//         }

//         Ok(composition_data)
//     }
//     fn render(&mut self) -> Option<Vec<ChartingData>>;
//     fn save(&mut self);
// }


// pub static COMPOSED_STORIES: Vec<&'static dyn IComposition> = vec![
//     SMA200Composition::instance()
// ];