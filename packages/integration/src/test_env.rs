use cw_multi_test::App;
use universe::species::{SapienceScale, Species};

pub fn mock_app() -> App {
    App::default()
}

pub fn get_species_by_level(level: SapienceScale) -> Species {
    match level {
        SapienceScale::None => Species {
            name: "No Brains".into(),
            sapience_level: SapienceScale::None,
        },
        SapienceScale::Low => Species {
            name: "Clown Fish".into(),
            sapience_level: SapienceScale::None,
        },
        SapienceScale::Medium => Species {
            name: "Humans".into(),
            sapience_level: SapienceScale::None,
        },
        SapienceScale::High => Species {
            name: "Exodites".into(),
            sapience_level: SapienceScale::None,
        },
    }
}
