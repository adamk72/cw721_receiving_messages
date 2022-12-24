use cw_multi_test::App;
use universe::species::{SapienceScale, Species};

pub fn mock_app() -> App {
    App::default()
}

pub fn get_species_by_level(name: &str, level: SapienceScale) -> Species {
    match level {
        SapienceScale::None => Species {
            name: name.into(),
            sapience_level: SapienceScale::None,
        },
        SapienceScale::Low => Species {
            name: name.into(),
            sapience_level: SapienceScale::Low,
        },
        SapienceScale::Medium => Species {
            name: name.into(),
            sapience_level: SapienceScale::Medium,
        },
        SapienceScale::High => Species {
            name: name.into(),
            sapience_level: SapienceScale::High,
        },
    }
}
