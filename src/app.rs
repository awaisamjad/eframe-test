use egui::{text::LayoutJob, Color32};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).

        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                if ui.button("Choose .md File").clicked() {
                    println!("Md file button clicked")
                };

                if ui.button("Switch").clicked() {
                    println!("Switch button clicked")
                };

                if ui.button("Save .html File").clicked() {
                    println!("HTML file button clicked")
                };
            });
            
            if ui.button("View Live").clicked(){
                println!("View Live Button Clicked")
            };

            let md_text = LayoutJob::single_section(
                "# Hello\n## my name is Awais Amjad".to_string(),
                egui::TextFormat {
                    color: Color32::GREEN,
                    background: Color32::WHITE,
                    ..Default::default()
                },
            );
            
            let html_text = LayoutJob::single_section(
                "<h1>Hello</h1>\n<h2>my name is Awais Amjad</h2>".to_string(),
                egui::TextFormat {
                    color: Color32::GREEN,
                    background: Color32::WHITE,
                    ..Default::default()
                },
            );

            ui.horizontal(|ui|{
                ui.label(md_text);
                ui.label(html_text)
        })
        });
    }
}
