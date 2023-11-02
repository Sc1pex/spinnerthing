#![feature(more_float_constants)]
mod app;

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "spinners",
                web_options,
                Box::new(|cc| Box::new(app::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
