// use std::collections::HashMap;
// use std::sync::{Arc, Mutex, OnceLock};

// use once_cell::sync::Lazy;
// use uuid::Uuid;

// use crate::commands::ChartingData;
// use crate::user::composer::{CompositionDataValue, IComposition};
// use crate::user::static_resources::mastercard::Mastercard;
// use crate::user::static_resources::{IStaticResource, StaticResource};

// pub struct SMA200Composition {
//     id: String,
//     name: String,
//     description: String,
//     composition_fields: HashMap<&'static str, i8>,
//     static_resources: HashMap<&'static str, StaticResource>,
// }

// impl IComposition for SMA200Composition {
//     fn id(&self) -> &str {
//         return &self.id;
//     }

//     fn name(&self) -> &str {
//         return &self.name;
//     }

//     fn description(&self) -> &str {
//         return &self.description;
//     }

//     fn composition_fields(&self) -> HashMap<&'static str, i8> {
//         return self.composition_fields.clone();
//     }

//     fn compose(&mut self) -> Vec<Vec<Option<CompositionDataValue>>> {
//         let composed_data: Vec<Vec<Option<CompositionDataValue>>> = vec![];

//         return composed_data;
//     }

//     fn render(&mut self) -> Option<Vec<ChartingData>> {
//         let charting_data: Vec<ChartingData> = vec![];

//         return Some(charting_data);
//     }

//     fn save(&mut self) {}
// }

// impl SMA200Composition {
//     pub fn instance() -> &'static SMA200Composition {
//         static INSTANCE: OnceLock<SMA200Composition> = OnceLock::new();
//         return INSTANCE.get_or_init(|| SMA200Composition::new());
//     }

//     fn new() -> Self {
//         return Self {
//             name: "SMA 200 Composition".into(),
//             description: "The composition for the SMA 200 strategy".into(),
//             id: Uuid::new_v4().into(),
//             composition_fields: HashMap::from([("close", 0), ("sma_200", 1)]),
//             static_resources: HashMap::from([(
//                 "mastercard",
//                 StaticResource::Mastercard(Mastercard::instance()),
//             )]),
//         };
//     }
// }
