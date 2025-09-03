use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct BTCUSDT_4YEARS_4H {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for BTCUSDT_4YEARS_4H {
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

impl BTCUSDT_4YEARS_4H {
    pub fn instance() -> &'static BTCUSDT_4YEARS_4H {
        static INSTANCE: OnceLock<BTCUSDT_4YEARS_4H> = OnceLock::new();
        return INSTANCE.get_or_init(|| BTCUSDT_4YEARS_4H::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "BTCUSDT_4YEARS_4H".into(),
            load_path: join_app_data_dir("raw/ohlcv/btcusdt_01_01_2021_06_15_2025_4h").unwrap(),
        };
    }
}
