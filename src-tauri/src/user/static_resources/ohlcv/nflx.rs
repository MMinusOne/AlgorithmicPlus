use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct NFLX {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for NFLX {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn data_type(&self) -> &str {
        return "OHLCV";
    }

    fn load_path(&self) -> PathBuf {
        return self.load_path.clone();
    }
}

impl NFLX {
    pub fn instance() -> &'static NFLX {
        static INSTANCE: OnceLock<NFLX> = OnceLock::new();
        return INSTANCE.get_or_init(|| NFLX::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "ETHUSDT".into(),
            load_path: join_app_data_dir("raw/ohlcv/3f50eaaa-d337-486c-b15d-09631a65fc00").unwrap(),
        };
    }
}
