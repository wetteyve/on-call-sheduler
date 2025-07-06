use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    start: String,
    end: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalPublicHolidayV3Dto {
    pub date: String,
    pub local_name: Option<String>,
    pub name: Option<String>,
    pub country_code: Option<String>,
    pub fixed: Option<bool>,
    pub global: bool,
    pub counties: Option<Vec<String>>,
    pub launch_year: Option<i32>,
    pub types: Option<Vec<String>>,
}

#[wasm_bindgen]
pub struct PublicHolidayV3Dto {
    inner: InternalPublicHolidayV3Dto,
}

#[wasm_bindgen]
impl PublicHolidayV3Dto {
    // Constructor example: create from raw values (adjust params as you like)
    #[wasm_bindgen(constructor)]
    pub fn new(
        date: String,
        local_name: Option<String>,
        name: Option<String>,
        country_code: Option<String>,
        fixed: Option<bool>,
        global: bool,
        counties: JsValue,     // Accept JS arrays or null
        launch_year: Option<i32>,
        types: JsValue,        // Accept JS arrays or null
    ) -> Result<PublicHolidayV3Dto, JsValue> {
        // Convert JsValue to Option<Vec<String>>
        let counties_vec: Option<Vec<String>> = if counties.is_null() || counties.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(counties)?)
        };

        let types_vec: Option<Vec<String>> = if types.is_null() || types.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(types)?)
        };

        Ok(PublicHolidayV3Dto {
            inner: InternalPublicHolidayV3Dto {
                date,
                local_name,
                name,
                country_code,
                fixed,
                global,
                counties: counties_vec,
                launch_year,
                types: types_vec,
            }
        })
    }

    // Getter for date (simple string)
    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.inner.date.clone()
    }

    // Getters for Option<String> fields
    #[wasm_bindgen(getter)]
    pub fn local_name(&self) -> Option<String> {
        self.inner.local_name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn country_code(&self) -> Option<String> {
        self.inner.country_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn fixed(&self) -> Option<bool> {
        self.inner.fixed
    }

    #[wasm_bindgen(getter)]
    pub fn global(&self) -> bool {
        self.inner.global
    }

    #[wasm_bindgen(getter)]
    pub fn counties(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.counties).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(getter)]
    pub fn launch_year(&self) -> Option<i32> {
        self.inner.launch_year
    }

    #[wasm_bindgen(getter)]
    pub fn types(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.types).unwrap_or(JsValue::NULL)
    }
}

