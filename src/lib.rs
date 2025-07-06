mod types;

use wasm_bindgen::prelude::*;
use crate::types::{DateRange, PublicHolidayV3Dto};


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn schedule(date_range: DateRange, public_holidays: Box<[PublicHolidayV3Dto]>) {
    for val in public_holidays.iter() {

        log(&format!(
            "Holiday: {} on {}",
            val.name().unwrap_or("undefined".to_string()),
            val.date()
        ));
    }

    log(&format!("{:?}", date_range));
}