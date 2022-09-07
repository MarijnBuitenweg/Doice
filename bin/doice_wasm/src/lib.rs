#[cfg(target_arch = "wasm32")]
use doice_lib::*;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(
        canvas_id,
        Box::new(|cc| {
            let mut app = Box::new(DoiceApp::new(cc));
            app.register_activity::<LegacyDiceRoller>();
            app.register_activity::<StarfuryYeeter>();
            app.register_activity::<DiceRoller>();
            app.register_activity::<GlobalAnalyzer>();
            app.register_activity::<SpellBrowser>();
            app.register_activity::<DiceRollPresets>();
            app.register_activity::<Notes>();
            app.register_activity::<CharacterManager>();
            app
        }),
    )
}

