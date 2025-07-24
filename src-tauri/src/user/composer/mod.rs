use crate::utils::classes::composition::IComposition;
use std::sync::Mutex;

pub static COMPOSED_STORIES: Mutex<Vec<Box<dyn IComposition>>>
= Mutex::new(vec![]);

