use rawaccel_convert::types::{AccelArgs, AccelMode, CapMode, CurvegenResult, PointScaling};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct RawaccelConvertSettings {
    pub dark_mode: bool,

    pub point_count_string: String,

    pub dpi_string: String,
    pub sens_multiplier_string: String,
    pub curve_type_string: String,

    pub acceleration_string: String,
    pub cap_output_string: String,
    pub cap_input_string: String,
    pub input_offset_string: String,
    pub exponent_classic_string: String,

    pub smooth_string: String,
    pub input_string: String,
    pub output_string: String,

    pub decay_string: String,
    pub limit_string: String,

    pub gamma_string: String,
    pub motivity_string: String,
    pub syncspeed_string: String,

    pub scale_string: String,
    pub exponent_power_string: String,
    pub output_offset_string: String,

    pub lookup_table_string: String,
}

impl Default for RawaccelConvertSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,

            point_count_string: "64".to_string(),

            //global
            dpi_string: "1200".to_string(),
            sens_multiplier_string: "1.0".to_string(),
            curve_type_string: "Off".to_string(),

            //linear/classic
            acceleration_string: "0.005".to_string(),
            cap_output_string: "1.5".to_string(),
            cap_input_string: "15".to_string(),
            input_offset_string: "0".to_string(),
            exponent_classic_string: "2".to_string(),

            //jump
            smooth_string: "0.5".to_string(),
            input_string: "15".to_string(),
            output_string: "1.5".to_string(),

            //natural
            decay_string: "0.1".to_string(),
            limit_string: "1.5".to_string(),

            //synchronous
            gamma_string: "1".to_string(),
            motivity_string: "1.5".to_string(),
            syncspeed_string: "5".to_string(),

            //power
            scale_string: "1".to_string(),
            exponent_power_string: "0.05".to_string(),
            output_offset_string: "0".to_string(),

            //lookup
            lookup_table_string: "".to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RawaccelConvertGui {
    settings: RawaccelConvertSettings,

    #[serde(skip)]
    accel_args: AccelArgs,
    #[serde(skip)]
    export_accel_args_cache: AccelArgs,

    #[serde(skip)]
    points: String,
    #[serde(skip)]
    libinput_steps: String,
    #[serde(skip)]
    curvegen: CurvegenResult,
    #[serde(skip)]
    curvegen_export: CurvegenResult,
    #[serde(skip)]
    export_point_scaling: PointScaling,
}

impl Default for RawaccelConvertGui {
    fn default() -> Self {
        Self {
            settings: RawaccelConvertSettings::default(),

            accel_args: AccelArgs::default(),
            export_accel_args_cache: AccelArgs::default(),

            points: String::default(),
            libinput_steps: String::default(),
            curvegen: CurvegenResult {
                points: vec![],
                step_size: 1.0,
            },
            curvegen_export: CurvegenResult {
                points: vec![],
                step_size: 1.0,
            },
            export_point_scaling: PointScaling::Sens,
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

                        if ui.button("Reset").clicked() {
                            self.settings = RawaccelConvertSettings::default();
                            self.accel_args = AccelArgs::default();
                            ui.close_menu();
                        }
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
                egui::Grid::new("rawaccel_convert_gui_grid").show(ui, |ui| {
                    add_dpi(self, ui);
                    ui.end_row();

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
                                }
                                rawaccel_convert::types::CapMode::Input => {
                                    add_acceleration(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_input(self, ui);
                                    ui.end_row();
                                }
                                rawaccel_convert::types::CapMode::Output => {
                                    add_acceleration(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_output(self, ui);
                                    ui.end_row();
                                }
                            }

                            add_input_offset(self, ui);
                            ui.end_row();

                            self.accel_args.exponent_classic = 2.0;
                        }
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
                                }
                                rawaccel_convert::types::CapMode::Input => {
                                    add_acceleration(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_input(self, ui);
                                    ui.end_row();
                                }
                                rawaccel_convert::types::CapMode::Output => {
                                    add_acceleration(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_output(self, ui);
                                    ui.end_row();
                                }
                            }

                            add_input_offset(self, ui);
                            ui.end_row();

                            add_power_classic(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Jump => {
                            add_gain(self, ui);
                            ui.end_row();

                            add_smooth(self, ui);
                            ui.end_row();

                            add_input(self, ui);
                            ui.end_row();

                            add_output(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Natural => {
                            add_gain(self, ui);
                            ui.end_row();

                            add_decay(self, ui);
                            ui.end_row();

                            add_input_offset(self, ui);
                            ui.end_row();

                            add_limit(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Synchronous => {
                            add_gain(self, ui);
                            ui.end_row();

                            add_gamma(self, ui);
                            ui.end_row();

                            add_smooth(self, ui);
                            ui.end_row();

                            add_motivity(self, ui);
                            ui.end_row();

                            add_syncspeed(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Motivity => {
                            add_gain(self, ui);
                            ui.end_row();

                            add_growthrate(self, ui);
                            ui.end_row();

                            add_motivity(self, ui);
                            ui.end_row();

                            add_midpoint(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Power => {
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
                                }
                                rawaccel_convert::types::CapMode::Input => {
                                    add_scale(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_input(self, ui);
                                    ui.end_row();
                                }
                                rawaccel_convert::types::CapMode::Output => {
                                    add_scale(self, ui);
                                    ui.end_row();

                                    add_cap_type(self, ui);
                                    ui.end_row();

                                    add_cap_output(self, ui);
                                    ui.end_row();
                                }
                            }

                            add_exponent(self, ui);
                            ui.end_row();

                            add_output_offset(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Lookup => {
                            add_lookup_table_box(self, ui);
                            ui.end_row();

                            add_apply_as(self, ui);
                            ui.end_row();
                        }
                        AccelMode::Noaccel => {}
                    }
                });
                add_points_dump(self, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot_accel_args = self.accel_args.clone();
            let plot_points = self.curvegen.points.clone();
            let plot_bounds = get_bounds(self, &plot_accel_args);
            let mut plot = egui_plot::Plot::new("lines_demo")
                .legend(egui_plot::Legend::default())
                .show_axes(true)
                .show_grid(true);
            plot = plot
                .coordinates_formatter(
                    egui_plot::Corner::LeftBottom,
                    egui_plot::CoordinatesFormatter::default(),
                )
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false);
            plot.show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                    plot_bounds.0,
                    plot_bounds.1,
                ));
                plot_ui.line(
                    egui_plot::Line::new(egui_plot::PlotPoints::new(convert_points(plot_points)))
                        .color(egui::Color32::from_rgb(100, 100, 200))
                        .style(egui_plot::LineStyle::Solid),
                );
            })
            .response
        });
    }
}

fn convert_points(points: Vec<rawaccel_convert::types::Point>) -> Vec<[f64; 2]> {
    let mut egui_points = vec![];
    for point in points {
        egui_points.push([point.x, point.y]);
    }
    return egui_points;
}

fn get_point(x: f64, args: &AccelArgs) -> f64 {
    let y = args.sens_multiplier
        * match &args.mode {
            AccelMode::Linear => rawaccel_convert::accel_curves::classic::classic(x, &args),
            AccelMode::Classic => rawaccel_convert::accel_curves::classic::classic(x, &args),
            AccelMode::Jump => rawaccel_convert::accel_curves::jump::jump(x, &args),
            AccelMode::Natural => rawaccel_convert::accel_curves::natural::natural(x, &args),
            AccelMode::Synchronous => {
                rawaccel_convert::accel_curves::synchronous::synchronous(x, &args)
                    .unwrap_or_default()
            }
            AccelMode::Power => rawaccel_convert::accel_curves::power::power(x, &args),
            AccelMode::Motivity => {
                rawaccel_convert::accel_curves::motivity::motivity(x, args).unwrap_or_default()
            }
            AccelMode::Lookup => {
                rawaccel_convert::accel_curves::lookup::lookup(x, args).unwrap_or_default()
            }
            AccelMode::Noaccel => rawaccel_convert::accel_curves::noaccel::noaccel(x, &args),
        };
    match args.point_scaling {
        PointScaling::Sens => return y,
        PointScaling::Velocity
        | PointScaling::Libinput
        | PointScaling::LibinputDebug
        | PointScaling::LookupVelocity
        | PointScaling::LookupSens => {
            let converted_point = rawaccel_convert::convert_curve::sensitivity_point_to_velocity(
                rawaccel_convert::types::Point { x, y },
            );
            return converted_point.y;
        }
        PointScaling::Gain => {
            unimplemented!()
        }
    }
}

fn get_bounds(
    rawaccel_convert_gui: &mut RawaccelConvertGui,
    args: &AccelArgs,
) -> ([f64; 2], [f64; 2]) {
    match args.mode {
        AccelMode::Lookup => {
            let plot_min_x = 0.1;
            let plot_max_x = (args.dpi.clone() / 20) as f64;
            let mut plot_min_y = f64::MAX;
            let mut plot_max_y = 0.0;

            for point in &rawaccel_convert_gui.curvegen.points {
                if point.x < plot_min_x || point.x > plot_max_x {
                    continue;
                }
                if point.y < plot_min_y {
                    plot_min_y = point.y;
                }
                if point.y > plot_max_y {
                    plot_max_y = point.y;
                }
            }
            return (
                [plot_min_x, plot_min_y * 0.9],
                [plot_max_x, plot_max_y * 1.1],
            );
        }
        _ => {
            let plot_min_x = match args.mode {
                AccelMode::Power | AccelMode::Lookup => 0.1,
                _ => 0.0,
            };
            let plot_max_x = (args.dpi.clone() / 20) as f64;
            return (
                [plot_min_x, get_point(plot_min_x, args) * 0.9],
                [plot_max_x, get_point(plot_max_x, args) * 1.1],
            );
        }
    }
}

fn add_dpi(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.dpi_string.parse::<u32>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.dpi = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("DPI").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.dpi_string),
    );
}

fn add_sens_multiplier(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .sens_multiplier_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.sens_multiplier = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Sens Multiplier").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.sens_multiplier_string),
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
                    AccelMode::Lookup => "Look Up Table".to_string(),
                    _ => format!("{:?}", rawaccel_convert_gui.accel_args.mode),
                }
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Noaccel,
                    "Off",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Linear,
                    "Linear",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Classic,
                    "Classic",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Jump,
                    "Jump",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Natural,
                    "Natural",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Synchronous,
                    "Synchronous",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Power,
                    "Power",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Motivity,
                    "Motivity",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.mode,
                    AccelMode::Lookup,
                    "Look Up Table",
                );
            });
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
    match rawaccel_convert_gui
        .settings
        .acceleration_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.acceleration = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Acceleration").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.acceleration_string),
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
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.cap_mode,
                    CapMode::Input,
                    "Input",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.cap_mode,
                    CapMode::Output,
                    "Output",
                );
                ui.selectable_value(
                    &mut rawaccel_convert_gui.accel_args.cap_mode,
                    CapMode::InputOutput,
                    "Both",
                );
            });
    });
}

fn add_cap_input(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .cap_input_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.x = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Cap: Input").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.cap_input_string),
    );
}

fn add_cap_output(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .cap_output_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.y = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Cap: Output").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.cap_output_string),
    );
}

fn add_input_offset(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .input_offset_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.input_offset = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Input Offset").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.input_offset_string),
    );
}

fn add_power_classic(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .exponent_classic_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.exponent_classic = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Power").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.exponent_classic_string),
    );
}

fn add_smooth(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.smooth_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.smooth = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Smooth").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.smooth_string),
    );
}

fn add_input(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.input_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.x = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Input").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.input_string),
    );
}

fn add_output(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.output_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.cap.y = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Output").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.output_string),
    );
}

fn add_decay(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.decay_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.decay_rate = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Decay Rate").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.decay_string),
    );
}

fn add_limit(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.limit_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.limit = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Limit").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.limit_string),
    );
}

fn add_gamma(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.gamma_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.gamma = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Gamma").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.gamma_string),
    );
}

fn add_growthrate(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.gamma_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.gamma = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Growth Rate").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.gamma_string),
    );
}

fn add_motivity(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.motivity_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.motivity = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Motivity").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.motivity_string),
    );
}

fn add_syncspeed(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .syncspeed_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.sync_speed = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("SyncSpeed").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.syncspeed_string),
    );
}

fn add_midpoint(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .syncspeed_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.sync_speed = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Midpoint").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.syncspeed_string),
    );
}

fn add_scale(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui.settings.scale_string.parse::<f64>() {
        Ok(ok) => rawaccel_convert_gui.accel_args.scale = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Scale").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.scale_string),
    );
}

fn add_exponent(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .exponent_power_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.exponent_power = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Exponent").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.exponent_power_string),
    );
}

fn add_output_offset(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert_gui
        .settings
        .output_offset_string
        .parse::<f64>()
    {
        Ok(ok) => rawaccel_convert_gui.accel_args.output_offset = ok,
        Err(_) => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Output Offset").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.output_offset_string),
    );
}

fn add_lookup_table_box(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    let mut color = ui.visuals().text_color();
    match rawaccel_convert::args_parser::parse_lookup_table(
        &rawaccel_convert_gui.settings.lookup_table_string,
    ) {
        Some(some) => rawaccel_convert_gui.accel_args.lookup_data = some,
        None => {
            color = ui.visuals().error_fg_color;
        }
    }
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("LUT Points").color(color)).selectable(false),
    );
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::singleline(&mut rawaccel_convert_gui.settings.lookup_table_string)
            .char_limit(usize::MAX),
    );
}

fn add_apply_as(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    ui.add_sized(
        ui.available_size(),
        egui::Label::new(egui::RichText::new("Apply As")).selectable(false),
    );
    ui.push_id("apply_as_dropdown", |ui| {
        egui::ComboBox::from_label("")
            .selected_text({
                match rawaccel_convert_gui.accel_args.gain {
                    true => "Velocity".to_string(),
                    false => "Sens".to_string(),
                }
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.gain, false, "Sens");
                ui.selectable_value(&mut rawaccel_convert_gui.accel_args.gain, true, "Velocity");
            });
    });
}

fn add_points_dump(rawaccel_convert_gui: &mut RawaccelConvertGui, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_sized(
            [ui.available_width(), 1.0],
            egui::Label::new("Graph View").selectable(false),
        );

        let previous_point_scaling = rawaccel_convert_gui.accel_args.point_scaling.clone();

        ui.push_id("point_scaling_graph_dropdown", |ui| {
            egui::ComboBox::from_label("")
                .width(ui.available_width())
                .selected_text(format!(
                    "{:?}",
                    rawaccel_convert_gui.accel_args.point_scaling
                ))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.accel_args.point_scaling,
                        PointScaling::Sens,
                        "Sens",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.accel_args.point_scaling,
                        PointScaling::Velocity,
                        "Velocity",
                    );
                })
        });

        ui.add_sized(
            [ui.available_width(), 1.0],
            egui::Label::new("Export Points").selectable(false),
        );

        ui.push_id("export_point_scaling_dropdown", |ui| {
            egui::ComboBox::from_label("")
                .width(ui.available_width())
                .selected_text(format!("{:?}", rawaccel_convert_gui.export_point_scaling))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::Sens,
                        "Sens",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::Velocity,
                        "Velocity",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::Libinput,
                        "Libinput",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::LibinputDebug,
                        "LibinputDebug",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::LookupVelocity,
                        "LookupVelocity",
                    );
                    ui.selectable_value(
                        &mut rawaccel_convert_gui.export_point_scaling,
                        PointScaling::LookupSens,
                        "LookupSens",
                    );
                })
        });

        egui::Grid::new("point_count_grid").show(ui, |ui| {
            let mut color = ui.visuals().text_color();
            match rawaccel_convert_gui
                .settings
                .point_count_string
                .parse::<u32>()
            {
                Ok(ok) => match rawaccel_convert_gui.export_point_scaling {
                    PointScaling::Libinput | PointScaling::LibinputDebug => {
                        if ok != 64 {
                            color = ui.visuals().error_fg_color;
                            rawaccel_convert_gui.settings.point_count_string = "64".to_string();
                        }
                    }
                    _ => {
                        if ok > 1 {
                            rawaccel_convert_gui.accel_args.point_count = ok;
                        } else {
                            color = ui.visuals().error_fg_color;
                        }
                    }
                },
                Err(_) => {
                    color = ui.visuals().error_fg_color;
                }
            }
            match rawaccel_convert_gui.export_point_scaling {
                PointScaling::Libinput | PointScaling::LibinputDebug => {
                    ui.hyperlink_to(
                        format!("Libinput accel guide"),
                        "https://github.com/Kuuuube/rawaccel_convert/blob/master/docs/libinput.md",
                    );
                }
                _ => {
                    ui.add_sized(
                        ui.available_size(),
                        egui::Label::new(egui::RichText::new("Max Number of Points").color(color))
                            .selectable(false),
                    );
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(
                            &mut rawaccel_convert_gui.settings.point_count_string,
                        ),
                    );
                }
            };
        });

        ui.add_sized(
            [ui.available_width(), 1.0],
            egui::Label::new("Points").selectable(false),
        );

        egui::ScrollArea::vertical()
            .max_height(100.0)
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width(), 1.0],
                    egui::TextEdit::multiline(&mut rawaccel_convert_gui.points),
                )
            });

        match rawaccel_convert_gui.export_point_scaling {
            PointScaling::Libinput | PointScaling::LibinputDebug => {
                ui.add_sized(
                    [ui.available_width(), 1.0],
                    egui::Label::new("Steps").selectable(false),
                );
                ui.add_sized(
                    [ui.available_width(), 1.0],
                    egui::TextEdit::singleline(&mut rawaccel_convert_gui.libinput_steps),
                );
            }
            _ => {}
        }

        let generate_points = ui.add_sized(
            [ui.available_width(), 1.0],
            egui::Button::new("Generate Points"),
        );
        if previous_point_scaling != rawaccel_convert_gui.accel_args.point_scaling
            || rawaccel_convert_gui.export_accel_args_cache != rawaccel_convert_gui.accel_args
        {
            rawaccel_convert_gui.export_accel_args_cache = rawaccel_convert_gui.accel_args.clone();

            rawaccel_convert_gui.accel_args.optimize_curve =
                match rawaccel_convert_gui.export_point_scaling {
                    PointScaling::Sens
                    | PointScaling::Velocity
                    | PointScaling::Gain
                    | PointScaling::LookupVelocity
                    | PointScaling::LookupSens => true,
                    PointScaling::Libinput | PointScaling::LibinputDebug => false,
                };

            //graph curve
            rawaccel_convert_gui.curvegen =
                rawaccel_convert::generate_curve::generate_curve(&rawaccel_convert_gui.accel_args);
        }

        if generate_points.clicked() {
            let mut export_accel_args = rawaccel_convert_gui.accel_args.clone();
            export_accel_args.point_scaling = rawaccel_convert_gui.export_point_scaling.clone();
            rawaccel_convert_gui.curvegen_export =
                rawaccel_convert::generate_curve::generate_curve(&export_accel_args);
            rawaccel_convert_gui.libinput_steps =
                rawaccel_convert_gui.curvegen_export.step_size.to_string();
            rawaccel_convert_gui.points = match export_accel_args.point_scaling {
                rawaccel_convert::types::PointScaling::Libinput => {
                    let mut output_string = String::default();
                    for point in &rawaccel_convert_gui.curvegen_export.points {
                        output_string += &(point.y.to_string() + " ");
                    }
                    output_string
                }
                rawaccel_convert::types::PointScaling::LookupVelocity | rawaccel_convert::types::PointScaling::LookupSens  => {
                    let mut output_string = String::default();
                    for point in &rawaccel_convert_gui.curvegen_export.points {
                        output_string += &format!("{},{};\n", point.x, point.y);
                    }
                    output_string
                }
                _ => {
                    format!("{:?}", rawaccel_convert_gui.curvegen_export.points)
                }
            }
        }
    });
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
