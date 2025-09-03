use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct SOLUSDT_4YEARS_15M {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for SOLUSDT_4YEARS_15M {
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

impl SOLUSDT_4YEARS_15M {
    pub fn instance() -> &'static SOLUSDT_4YEARS_15M {
        static INSTANCE: OnceLock<SOLUSDT_4YEARS_15M> = OnceLock::new();
        return INSTANCE.get_or_init(|| SOLUSDT_4YEARS_15M::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "SOLUSDT_4YEARS_15M".into(),
            load_path: join_app_data_dir("raw/ohlcv/solusdt_01_01_2021_06_15_2025_15m").unwrap(),
        };
    }
}
