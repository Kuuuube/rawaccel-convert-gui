use rawaccel_convert::types::{AccelArgs, AccelMode};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RawaccelConvertSettings {
    pub dark_mode: bool,

    pub sens_multiplier_string: String,
    pub curve_type_string: String,
}

impl Default for RawaccelConvertSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,

            sens_multiplier_string: "1.0".to_string(),
            curve_type_string: "Linear".to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RawaccelConvertGui {
    settings: RawaccelConvertSettings,

    #[serde(skip)]
    accel_args: AccelArgs,
}

impl Default for RawaccelConvertGui {
    fn default() -> Self {
        Self {
            settings: RawaccelConvertSettings::default(),

            accel_args: AccelArgs::default(),
        }
    }
}

impl RawaccelConvertGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //restore state
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        return Default::default();
    }
}

impl eframe::App for RawaccelConvertGui {
    //save state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.settings.dark_mode);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Settings", |ui| {
                        light_dark_buttons(self, ui);

                        ui.menu_button("Reset", |ui| {
                            if ui.button("Confirm").clicked() {
                                self.settings = RawaccelConvertSettings::default();
                                ui.close_menu();
                            }
                        });
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unselectable_warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "right_sidepanel")
            .min_width(250.0)
            .max_width(250.0)
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("hentaigana_selection_grid").show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::Label::new("Sens Multiplier").selectable(false),
                    );
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(
                            &mut self.settings.sens_multiplier_string,
                        ),
                    );
                    if response.changed() {
                        
                    }
                    ui.end_row();

                    ui.add_sized(
                        ui.available_size(),
                        egui::Label::new("Curve Type").selectable(false),
                    );
                    egui::ComboBox::from_label("")
                        .selected_text({
                            match self.accel_args.mode {
                                AccelMode::Noaccel => "Off".to_string(),
                                _ => format!("{:?}", self.accel_args.mode)
                            }
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Noaccel, "Off");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Linear, "Linear");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Classic, "Classic");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Jump, "Jump");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Natural, "Natural");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Synchronous, "Synchronous");
                            ui.selectable_value(&mut self.accel_args.mode, AccelMode::Power, "Power");
                        }
                    );

                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}

fn unselectable_warn_if_debug_build(ui: &mut egui::Ui) {
    if cfg!(debug_assertions) {
        ui.add(
            egui::Label::new(
                egui::RichText::new("⚠ Debug build ⚠")
                    .small()
                    .color(ui.visuals().warn_fg_color),
            )
            .selectable(false),
        );
    }
}

fn light_dark_buttons(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.light_dark_radio_buttons(ui);
    rawaccel_convert_gui.settings.dark_mode = visuals.dark_mode;
    set_theme(ui.ctx(), visuals.dark_mode);
}

fn set_theme(ctx: &egui::Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(egui::Visuals::dark());
    } else {
        ctx.set_visuals(egui::Visuals::light());
    }
}
