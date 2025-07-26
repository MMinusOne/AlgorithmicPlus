use std::collections::HashMap;

use uuid::Uuid;

use crate::commands::ChartingData;
use crate::user::composer::{IComposition, CompositionDataValue};

pub struct SMA200Composition {
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, i8>,
    id: String,
}

impl IComposition for SMA200Composition {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn composition_fields(&self) -> HashMap<&'static str, i8> {
        return self.composition_fields.clone();
    }

    fn compose(&mut self) -> Vec<Vec<Option<CompositionDataValue>>> {
        let composed_data: Vec<Vec<Option<CompositionDataValue>>> = vec![];

        return composed_data;
    }

    fn render(&mut self) -> Option<Vec<ChartingData>> {
        let charting_data: Vec<ChartingData> = vec![];

        return Some(charting_data);
    }
}

impl SMA200Composition {
    fn new() -> Self {
        return Self {
            name: "SMA 200 Composition".into(),
            description: "The composition for the SMA 200 strategy".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([("close", 0), ("sma_200", 1)]),
        };
    }
}
