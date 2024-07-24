#![windows_subsystem = "windows"]

mod gui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([650.0, 650.0])
            .with_min_inner_size([650.0, 650.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Rawaccel Convert GUI",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::RawaccelConvertGui::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(gui::RawaccelConvertGui::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = eframe::web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
