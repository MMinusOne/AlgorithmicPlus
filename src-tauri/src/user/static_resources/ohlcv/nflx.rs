use std::{sync::OnceLock, vec};

use uuid::Uuid;

use crate::user::static_resources::{IStaticResource, OHLCVData};

pub struct NFLX {
    id: String,
    name: String,
    load_path: String,
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

    fn load_path(&self) -> &str {
        return &self.load_path;
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
            load_path: "C:/Users/pc/AppData/Roaming/com.algorithmicplus.app/raw/ohlcv/a22cd66c-0e81-47f2-a2b6-11d20f670959".into(),
        };
    }
}
