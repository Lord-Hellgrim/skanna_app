#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use std::collections::{HashMap, BTreeMap};
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

fn make_display_string(input: &mut HashMap<String, (i32, u64)>) -> String {
    
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

struct Product {
    id: String,
    name: String,
    cost: f64,
    price: f64,
    stock: i64,
}

impl Product {
    pub fn new(id: String) -> Product {
        Product {
            id: id,
            name: "Standard".to_owned(),
            cost: 10.0,
            price: 10.0,
            stock: 50,
        }
    }
}

struct SkannaApp {
    // Example stuff:
    skannabox: String,
    magnbox: String,
    listabox: String,
    picked_path: Option<String>,
    app_starting: bool,
    vorulisti: HashMap<String, (i32, u64)>,
    product_list: Vec<Product>,
    start_time: SystemTime,
    app_switch: u8,
    current_warehouse: String,
    add_product: String,
}

impl Default for SkannaApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            skannabox: String::from(""),
            magnbox: String::from("1"),
            listabox: String::from("Hér kemur svo listinn"),
            picked_path: None,
            app_starting: true,
            vorulisti: HashMap::new(),
            product_list: Vec::from([
                Product {
                    id: "0113035".to_owned(),
                    name: "Undirlegg".to_owned(),
                    cost: 300.0,
                    price: 1000.0,
                    stock: 100,
                },
                Product {
                    id: "18572054".to_owned(),
                    name: "Flísalím".to_owned(),
                    cost: 2000.0,
                    price: 4500.0,
                    stock: 42,
                },
            ]),
            start_time: SystemTime::now(),
            app_switch: 0,
            current_warehouse: "WH01".to_owned(),
            add_product: "".to_owned(),
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
            skannabox,
            listabox,
            magnbox,
            picked_path,
            app_starting,
            vorulisti,
            product_list,
            start_time,
            app_switch,
            current_warehouse,
            add_product,
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
                    
                });
            });
        });

        let header_panel = egui::TopBottomPanel::top("head_panel").min_height(10.0).max_height(1000.0).resizable(true);
        header_panel.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.
                ui.horizontal(|ui| {
                    let purchase_order_switch = ui.add_sized([120., 40.], egui::Button::new("Purchase Order"));
                    if purchase_order_switch.clicked() {
                        *app_switch = 0;
                        println!("{}", app_switch);
                    }
                    let sales_order_switch = ui.add_sized([120., 40.], egui::Button::new("Sales Order"));
                    if sales_order_switch.clicked() {
                        *app_switch = 1;
                        println!("{}", app_switch);
                    }
                    let transfer_order_switch = ui.add_sized([120., 40.], egui::Button::new("Transfer Order"));
                    if transfer_order_switch.clicked() {
                        *app_switch = 2;
                        println!("{}", app_switch);
                    }
                    let correction_order_switch = ui.add_sized([120., 40.], egui::Button::new("Correction Order"));
                    if correction_order_switch.clicked() {
                        *app_switch = 3;
                        println!("{}", app_switch);
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {

            ui.vertical(|ui| {
                ui.add_space(50.0);
                let purchase_order_switch = ui.add_sized([120., 40.], egui::Button::new("Action 1"));
                if purchase_order_switch.clicked() {
                    println!("Sale!");
                }
                let sales_order_switch = ui.add_sized([120., 40.], egui::Button::new("Action 2"));
                if sales_order_switch.clicked() {
                    println!("Purchase!");
                }
                let transfer_order_switch = ui.add_sized([120., 40.], egui::Button::new("Action 3"));
                if transfer_order_switch.clicked() {
                    println!("Transfer!");
                }
                let correction_order_switch = ui.add_sized([120., 40.], egui::Button::new("Action 4"));
                if correction_order_switch.clicked() {
                    println!("Correction!");
                }
            });

        });

        if self.app_switch == 0 {
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
                        ui.vertical(|ui| {
                            ui.add(egui::TextEdit::singleline(magnbox).desired_width(35.0));
                        });
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
    
                            if ui.button("<enter>").clicked()
                                ||  // or
                                ctx.input(|i| i.key_pressed(egui::Key::Enter))
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
                                } else {
                                    scan.request_focus();
                                }
                            }
                            
                        });
    
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(egui::TextEdit::multiline(listabox).desired_rows(35));
                    });
                });
            });
        } else if self.app_switch == 1 {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(add_product));
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    product_list.push(Product::new(add_product.clone()));
                }
                for product in product_list {
                    ui.horizontal(|ui| {
                        ui.label(&product.id);
                        ui.label(&product.name);
                        ui.label(&product.cost.to_string());
                        ui.label(&product.price.to_string());
                        ui.label(&product.stock.to_string());
                    });
                }
            });
        } else if self.app_switch == 2 {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("LOVE");
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("YOU");
            });
        }
    }
}
