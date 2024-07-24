use rawaccel_convert::types::{AccelArgs, AccelMode, CapMode};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RawaccelConvertSettings {
    pub dark_mode: bool,

    pub sens_multiplier_string: String,
    pub curve_type_string: String,
    pub acceleration_string: String,
    pub cap_output_string: String,
    pub cap_input_string: String,
    pub input_offset_string: String,
    pub exponent_classic_string: String,
}

impl Default for RawaccelConvertSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,

            sens_multiplier_string: "1.0".to_string(),
            curve_type_string: "Off".to_string(),
            acceleration_string: "0.005".to_string(),
            cap_output_string: "1.5".to_string(),
            cap_input_string: "15".to_string(),
            input_offset_string: "0".to_string(),
            exponent_classic_string: "2".to_string(),
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
                    add_sens_multiplier(self, ui);
                    ui.end_row();

                    add_curve_type(self, ui);
                    ui.end_row();

                    match self.accel_args.mode {
                        AccelMode::Linear => {
                            add_gain(self, ui);
                            ui.end_row();

                            match self.accel_args.cap_mode {
                                rawaccel_convert::types::CapMode::InputOutput => {
                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_input(self, ui);
                                    ui.end_row();

                                    add_cap_output(self, ui);
                                    ui.end_row();
                                },
                                rawaccel_convert::types::CapMode::Input => {
                                    add_acceleration(self, ui);
                                    ui.end_row();
        
                                    add_cap_type(self, ui);
                                    ui.end_row();
        
                                    add_cap_input(self, ui);
                                    ui.end_row();
                                },
                                rawaccel_convert::types::CapMode::Output => {
                                    add_acceleration(self, ui);
                                    ui.end_row();
        
                                    add_cap_type(self, ui);
                                    ui.end_row();
        
                                    add_cap_output(self, ui);
                                    ui.end_row();
                                },
                            }

                            add_input_offset(self, ui);
                            ui.end_row();
                        },
                        AccelMode::Classic => {
                            add_gain(self, ui);
                            ui.end_row();

                            match self.accel_args.cap_mode {
                                rawaccel_convert::types::CapMode::InputOutput => {
                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_input(self, ui);
                                    ui.end_row();

                                    add_cap_output(self, ui);
                                    ui.end_row();
                                },
                                rawaccel_convert::types::CapMode::Input => {
                                    add_acceleration(self, ui);
                                    ui.end_row();
        
                                    add_cap_type(self, ui);
                                    ui.end_row();
        
                                    add_cap_input(self, ui);
                                    ui.end_row();
                                },
                                rawaccel_convert::types::CapMode::Output => {
                                    add_acceleration(self, ui);
                                    ui.end_row();
        
                                    add_cap_type(self, ui);
                                    ui.end_row();
        
                                    add_cap_output(self, ui);
                                    ui.end_row();
                                },
                            }

                            add_input_offset(self, ui);
                            ui.end_row();

                            add_power_classic(self, ui);
                            ui.end_row();
                        },
                        AccelMode::Jump => {
                            add_gain(self, ui);
                            ui.end_row();

                        },
                        AccelMode::Natural => {
                            add_gain(self, ui);
                            ui.end_row();

                        },
                        AccelMode::Synchronous => {
                            add_gain(self, ui);
                            ui.end_row();

                        },
                        AccelMode::Power => {
                            add_gain(self, ui);
                            ui.end_row();

                        },
                        AccelMode::Noaccel => {},
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}

fn add_sens_multiplier(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.sens_multiplier_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.sens_multiplier = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Sens Multiplier").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.sens_multiplier_string,
        ),
    );

}

fn add_curve_type(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    ui.add_sized(
        ui.available_size(),
        egui::Label::new("Curve Type").selectable(false),
    );
    ui.push_id("curve_type_dropdown", |ui| {
        egui::ComboBox::from_label("")
            .selected_text({
                match rawaccel_convert_gui.accel_args.mode {
                    AccelMode::Noaccel => "Off".to_string(),
                    _ => format!("{:?}", rawaccel_convert_gui.accel_args.mode)
                }
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Noaccel, "Off");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Linear, "Linear");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Classic, "Classic");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Jump, "Jump");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Natural, "Natural");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Synchronous, "Synchronous");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.mode, AccelMode::Power, "Power");
            }
        );
    });
}

fn add_gain(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    ui.add_sized(
        ui.available_size(),
        egui::Label::new("Gain").selectable(false),
    );
    ui.checkbox(&mut rawaccel_convert_gui.accel_args.gain, "");
}

fn add_acceleration(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.acceleration_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.acceleration = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Acceleration").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.acceleration_string,
        ),
    );
}

fn add_cap_type(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Cap Type")).selectable(false),
    );
    ui.push_id("cap_type_dropdown", |ui| {
        egui::ComboBox::from_label("")
            .selected_text({
                match rawaccel_convert_gui.accel_args.cap_mode {
                    CapMode::InputOutput => "Both".to_string(),
                    _ => format!("{:?}", rawaccel_convert_gui.accel_args.cap_mode),
                }
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.cap_mode, CapMode::Input, "Input");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.cap_mode, CapMode::Output, "Output");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.cap_mode, CapMode::InputOutput, "Both");
            }
        );
    });
}

fn add_cap_input(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.cap_input_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.x = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Cap: Input").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.cap_input_string,
        ),
    );
}

fn add_cap_output(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.cap_output_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.y = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Cap: Output").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.cap_output_string,
        ),
    );
}

fn add_input_offset(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.input_offset_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.input_offset = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Input Offset").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.input_offset_string,
        ),
    );
}

fn add_power_classic(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.exponent_classic_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.exponent_classic = ok,
        Err(_) => {color = ui.visuals().error_fg_color;},
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Power").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(
            &mut rawaccel_convert_gui.settings.exponent_classic_string,
        ),
    );
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
