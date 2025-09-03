use std::{path::PathBuf, sync::OnceLock};

use uuid::Uuid;

use crate::{
    user::static_resources::{IStaticResource, OHLCVData},
    utils::paths::join_app_data_dir,
};

pub struct NVDA_15YEARS_1D {
    id: String,
    name: String,
    load_path: PathBuf,
}

impl IStaticResource<OHLCVData> for NVDA_15YEARS_1D {
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

impl NVDA_15YEARS_1D {
    pub fn instance() -> &'static NVDA_15YEARS_1D {
        static INSTANCE: OnceLock<NVDA_15YEARS_1D> = OnceLock::new();
        return INSTANCE.get_or_init(|| NVDA_15YEARS_1D::new());
    }

    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "NVDA_15YEARS_1D".into(),
            load_path: join_app_data_dir("raw/ohlcv/nvda_01_01_2010_06_15_2025_1d").unwrap(),
        };
    }
}
