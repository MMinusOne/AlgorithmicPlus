use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct ETHUSDT {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for ETHUSDT {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn load_path(&self) -> PathBuf {
        return self.load_path.clone();
    }
}

impl ETHUSDT {
    pub fn instance() -> &'static ETHUSDT {
        static INSTANCE: OnceLock<ETHUSDT> = OnceLock::new();
        return INSTANCE.get_or_init(|| ETHUSDT::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "ETHUSDT".into(),
            load_path: join_app_data_dir("raw/ohlcv/6c3b733f-006b-4bc0-9cd7-cd4c3082beae").unwrap(),
        };
    }
}
