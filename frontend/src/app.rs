use eframe::egui;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaAsset {
    pub id: i64,
    pub file_path: String,
    pub imported_at: String,
}

#[derive(Serialize)]
struct InvokeArgs {}

pub struct MyApp {
    media: Arc<Mutex<Vec<MediaAsset>>>,
    axum_port: Arc<Mutex<Option<u16>>>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let app = Self {
            media: Arc::new(Mutex::new(Vec::new())),
            axum_port: Arc::new(Mutex::new(None)),
        };

        app.fetch_port();
        app.fetch_media(cc.egui_ctx.clone());
        app
    }

    fn fetch_port(&self) {
        let port_clone = self.axum_port.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let res = invoke(
                "get_axum_port",
                serde_wasm_bindgen::to_value(&InvokeArgs {}).unwrap(),
            )
            .await;
            if let Ok(port) = serde_wasm_bindgen::from_value::<u16>(res) {
                *port_clone.lock().unwrap() = Some(port);
            }
        });
    }

    fn fetch_media(&self, ctx: egui::Context) {
        let media_clone = self.media.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let res = invoke(
                "fetch_media",
                serde_wasm_bindgen::to_value(&InvokeArgs {}).unwrap(),
            )
            .await;
            if let Ok(media) = serde_wasm_bindgen::from_value::<Vec<MediaAsset>>(res) {
                *media_clone.lock().unwrap() = media;
                ctx.request_repaint();
            }
        });
    }

    fn import_directory(&self, ctx: egui::Context) {
        let media_clone = self.media.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = invoke(
                "import_directory",
                serde_wasm_bindgen::to_value(&InvokeArgs {}).unwrap(),
            )
            .await;
            let res = invoke(
                "fetch_media",
                serde_wasm_bindgen::to_value(&InvokeArgs {}).unwrap(),
            )
            .await;
            if let Ok(media) = serde_wasm_bindgen::from_value::<Vec<MediaAsset>>(res) {
                *media_clone.lock().unwrap() = media;
                ctx.request_repaint();
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Import Media").clicked() {
                self.import_directory(ctx.clone());
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let media = self.media.lock().unwrap().clone();
            let port = *self.axum_port.lock().unwrap();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for m in media {
                        ui.group(|ui| {
                            if let Some(p) = port {
                                let url = format!(
                                    "http://127.0.0.1:{}/media?path={}",
                                    p,
                                    urlencoding::encode(&m.file_path)
                                );
                                ui.add(egui::Image::new(url).max_size(egui::vec2(100.0, 100.0)));
                            }
                            ui.label(format!("ID: {}", m.id));
                        });
                    }
                });
            });
        });
    }
}
