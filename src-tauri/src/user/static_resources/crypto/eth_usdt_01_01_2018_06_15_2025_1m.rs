use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct ETHUSDT_7YEARS_1M {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for ETHUSDT_7YEARS_1M {
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

impl ETHUSDT_7YEARS_1M {
    pub fn instance() -> &'static ETHUSDT_7YEARS_1M {
        static INSTANCE: OnceLock<ETHUSDT_7YEARS_1M> = OnceLock::new();
        return INSTANCE.get_or_init(|| ETHUSDT_7YEARS_1M::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "ETHUSDT_7YEARS_1M".into(),
            load_path: join_app_data_dir("raw/ohlcv/ethusdt_01_01_2018_06_15_2025_1m").unwrap(),
        };
    }
}
