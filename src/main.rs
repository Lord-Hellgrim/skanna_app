#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use std::collections::HashMap;
use std::time::SystemTime;

// use sqlx::postgres::PgPoolOptions;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
//#[tokio::main]
fn main(){
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(SkannaApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(SkannaApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state

fn make_display_string(input: &mut HashMap<String, (i32, u64 /* last scanned at */)>) -> String {
    
    let mut temp = Vec::new();
    for (key, value) in input {
        temp.push((key, value.0, value.1));
    }
    temp.sort_by(|a, b| a.2.cmp(&b.2));
    let mut disp_string = String::from("");
    for item in temp.iter().rev() {
        disp_string.push_str(&item.0);
        disp_string.push('\t');
        disp_string.push_str(&item.1.to_string());
        disp_string.push('\n');
    }
    disp_string
}

struct SkannaApp {
    // Example stuff:
    label: String,
    skannabox: String,
    magnbox: String,
    listabox: String,
    value: f32,
    window_open: bool,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    app_starting: bool,
    vorulisti: HashMap<String, (i32, u64)>,
    start_time: SystemTime,
}

impl Default for SkannaApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            skannabox: String::from(""),
            magnbox: String::from("1"),
            listabox: String::from("Hér kemur svo listinn"),
            value: 2.7,
            window_open: false,
            dropped_files: Vec::new(),
            picked_path: None,
            app_starting: true,
            vorulisti: HashMap::new(),
            start_time: SystemTime::now(),
        }
    }
}

impl SkannaApp {
    /// Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //cc.egui_ctx.set_visuals(egui::Visuals::light());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        Default::default()
    }
}

impl eframe::App for SkannaApp {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            skannabox,
            listabox,
            magnbox,
            value,
            window_open,
            dropped_files,
            picked_path,
            app_starting,
            vorulisti,
            start_time,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    };
                    // This will open a menu later
                    if ui.button("Open window").clicked() {
                        self.window_open = true;
                    }
                });
            });
        });

        let header_panel = egui::TopBottomPanel::top("head_panel").min_height(10.0).max_height(1000.0).resizable(true);
        header_panel.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.
                ui.text_edit_multiline(magnbox);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical(|ui| {
                ui.heading("Skanna app Hallgríms");
                ui.horizontal(|ui| {
                    ui.label("Skanna hér");
                    ui.add_space(40.0);
                    ui.label("Magn hér");

                });

                ui.horizontal(|ui| {
                    let scan = ui.add(egui::TextEdit::singleline(skannabox).desired_width(100.0));
                    if self.app_starting {
                        scan.request_focus();
                        self.app_starting = false;
                    }
                    // THIS PART IS WHAT MY DISCORD QUESTION REFERS TO
                    ui.vertical(|ui| {
                        // Specifically this line
                        ui.add(egui::TextEdit::singleline(magnbox).desired_width(35.0));
                        //ui.text_edit_singleline(magnbox);
                    });
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        if ui.button("<enter>").clicked()
                            ||  // or
                            ctx.input(|i| i.key_pressed(egui::Key::Enter))
                            // &&  // and
                            // skannabox.trim() != ""
                        {

                            let key = skannabox.clone();
                            let timestamp: u64 = self.start_time.elapsed().expect("Some time should have elapsed here").as_secs();
                            if key != "" {
                                let incr: i32 = match magnbox.parse() {
                                    Ok(num) => num,
                                    Err(_) => 0,
                                };
                                let magn = match vorulisti.get(&key) {
                                    Some(value) => incr + value.0,
                                    None => incr,
                                };
                                vorulisti.insert(key, (magn, timestamp));
                                *listabox = make_display_string(vorulisti);
                                scan.request_focus();
                                skannabox.clear();

                            }
                        }
                        
                    });

                });
                ui.text_edit_multiline(listabox);
            });
        });

        

        
    }
}
