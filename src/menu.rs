use eframe::egui;
use egui::Align;
use egui_alignments::{column, row, center_horizontal};
use std::path::Path;

use crate::{Settings, start_application};


fn draw_plane(
    painter: &egui::Painter,
    position: egui::Pos2,
    scale: f32,
    angle: f32,
) {
    let (sin_a, cos_a) = angle.sin_cos();

    // Local aircraft coordinates:
    //
    //          nose
    //           ^
    //           |
    //          (0,-60)
    //           |
    //       \   |   /
    //        \  |  /
    //         \ | /
    //          \|/
    //      -----+-----
    //           |
    //           |
    //          tail
    //
    // Rotate and translate a local point.
    let transform = |p: egui::Pos2| -> egui::Pos2 {
        position
            + egui::vec2(
                (p.x * cos_a - p.y * sin_a) * scale,
                (p.x * sin_a + p.y * cos_a) * scale,
            )
    };

    let plane_color =
        egui::Color32::from_rgba_unmultiplied(70, 160, 210, 200);

    let plane_stroke = egui::Stroke::new(
        1.0 * scale,
        egui::Color32::from_rgba_unmultiplied(140, 215, 240, 150),
    );

    // ------------------------------------------------------------
    // FUSELAGE
    // ------------------------------------------------------------
    //
    // Long narrow body with a rounded nose and tapered rear.
    //
    //                 nose
    //                  *
    //                /   \
    //              /       \
    //             |         |
    //             |         |
    //             |         |
    //              \       /
    //               \_____/
    //
    let fuselage = vec![
        // Rounded nose
        transform(egui::pos2(0.0, -62.0)),
        transform(egui::pos2(5.0, -60.0)),
        transform(egui::pos2(9.0, -55.0)),
        transform(egui::pos2(11.0, -48.0)),

        // Right side of fuselage
        transform(egui::pos2(11.0, -20.0)),
        transform(egui::pos2(10.0, 10.0)),
        transform(egui::pos2(8.0, 35.0)),
        transform(egui::pos2(6.0, 52.0)),

        // Tail
        transform(egui::pos2(4.0, 59.0)),
        transform(egui::pos2(0.0, 63.0)),
        transform(egui::pos2(-4.0, 59.0)),

        // Left side of fuselage
        transform(egui::pos2(-6.0, 52.0)),
        transform(egui::pos2(-8.0, 35.0)),
        transform(egui::pos2(-10.0, 10.0)),
        transform(egui::pos2(-11.0, -20.0)),
        transform(egui::pos2(-11.0, -48.0)),

        // Back around nose
        transform(egui::pos2(-9.0, -55.0)),
        transform(egui::pos2(-5.0, -60.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        fuselage,
        plane_color,
        plane_stroke,
    ));

    // ------------------------------------------------------------
    // MAIN LEFT WING
    // ------------------------------------------------------------
    //
    // Swept-back wing:
    //
    //                 fuselage
    //                    |
    //                    |
    //             ______| 
    //            /      |
    //           /       |
    //          /________|
    //
    let left_wing = vec![
        transform(egui::pos2(-8.0, -20.0)),   // wing root, front
        transform(egui::pos2(-17.0, -16.0)),  // start of sweep
        transform(egui::pos2(-58.0, 20.0)),   // wing tip
        transform(egui::pos2(-53.0, 27.0)),   // wing tip rear
        transform(egui::pos2(-13.0, 8.0)),    // trailing edge
        transform(egui::pos2(-8.0, 15.0)),    // root rear
    ];

    painter.add(egui::Shape::convex_polygon(
        left_wing,
        plane_color,
        plane_stroke,
    ));

    // ------------------------------------------------------------
    // MAIN RIGHT WING
    // ------------------------------------------------------------

    let right_wing = vec![
        transform(egui::pos2(8.0, -20.0)),
        transform(egui::pos2(17.0, -16.0)),
        transform(egui::pos2(58.0, 20.0)),
        transform(egui::pos2(53.0, 27.0)),
        transform(egui::pos2(13.0, 8.0)),
        transform(egui::pos2(8.0, 15.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        right_wing,
        plane_color,
        plane_stroke,
    ));

    // ------------------------------------------------------------
    // LEFT TAILPLANE
    // ------------------------------------------------------------

    let left_tail = vec![
        transform(egui::pos2(-6.0, 36.0)),
        transform(egui::pos2(-13.0, 39.0)),
        transform(egui::pos2(-30.0, 53.0)),
        transform(egui::pos2(-27.0, 58.0)),
        transform(egui::pos2(-7.0, 51.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        left_tail,
        plane_color,
        plane_stroke,
    ));

    // ------------------------------------------------------------
    // RIGHT TAILPLANE
    // ------------------------------------------------------------

    let right_tail = vec![
        transform(egui::pos2(6.0, 36.0)),
        transform(egui::pos2(13.0, 39.0)),
        transform(egui::pos2(30.0, 53.0)),
        transform(egui::pos2(27.0, 58.0)),
        transform(egui::pos2(7.0, 51.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        right_tail,
        plane_color,
        plane_stroke,
    ));

    // ------------------------------------------------------------
    // VERTICAL TAIL FIN
    // ------------------------------------------------------------
    //
    // Since we're looking from above, this is represented as
    // a triangular fin extending toward the rear.
    //
    let vertical_tail = vec![
        transform(egui::pos2(-4.0, 40.0)),
        transform(egui::pos2(0.0, 22.0)),
        transform(egui::pos2(4.0, 40.0)),
        transform(egui::pos2(0.0, 58.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        vertical_tail,
        egui::Color32::from_rgba_unmultiplied(55, 135, 185, 110),
        egui::Stroke::NONE,
    ));

    // ------------------------------------------------------------
    // SUBTLE FUSELAGE HIGHLIGHT
    // ------------------------------------------------------------

    let highlight = vec![
        transform(egui::pos2(0.0, -57.0)),
        transform(egui::pos2(2.5, -52.0)),
        transform(egui::pos2(2.5, 38.0)),
        transform(egui::pos2(0.0, 55.0)),
        transform(egui::pos2(-2.5, 38.0)),
        transform(egui::pos2(-2.5, -52.0)),
    ];

    painter.add(egui::Shape::convex_polygon(
        highlight,
        egui::Color32::from_rgba_unmultiplied(180, 230, 245, 50),
        egui::Stroke::NONE,
    ));
}


fn draw_landscape(painter: &egui::Painter, rect: egui::Rect, time: f32) {
    let horizon = rect.top() + rect.height() * 0.85;
    let terrain_height = rect.height() * 0.15;
    let line_color = egui::Color32::from_rgba_unmultiplied(90, 150, 180, 75);
    let distant_color = egui::Color32::from_rgba_unmultiplied(70, 125, 155, 45);

    let ridge = |points: &[(f32, f32)], color: egui::Color32| {
        let translated: Vec<egui::Pos2> = points
            .iter()
            .map(|(x, y)| egui::pos2(rect.left() + x * rect.width(), horizon - y * terrain_height))
            .collect();
        painter.add(egui::Shape::line(
            translated,
            egui::Stroke::new(1.0, color),
        ));
    };

    ridge(
        &[
            (-0.10, 0.01), (0.05, 0.14), (0.14, 0.06), (0.25, 0.25),
            (0.34, 0.09), (0.46, 0.19), (0.56, 0.05), (0.68, 0.22),
            (0.78, 0.08), (0.91, 0.18), (1.10, 0.02),
        ],
        distant_color,
    );
    ridge(
        &[
            (-0.08, 0.0), (0.08, 0.08), (0.18, 0.03), (0.30, 0.18),
            (0.38, 0.06), (0.51, 0.29), (0.60, 0.10), (0.73, 0.20),
            (0.84, 0.05), (0.96, 0.14), (1.08, 0.0),
        ],
        line_color,
    );

    let vanishing_point = egui::pos2(rect.center().x, horizon);

    let foreground_ridge = |points: &[(f32, f32)]| {
        let translated: Vec<egui::Pos2> = points
            .iter()
            .map(|(x, y)| egui::pos2(rect.left() + x * rect.width(), rect.top() + y * rect.height()))
            .collect();
        painter.add(egui::Shape::line(
            translated,
            egui::Stroke::new(1.2, line_color),
        ));
    };

    // Foreground mountain planes rise out of the lower-left corner.
    let mountain_profile = [
        (-0.06, 1.02), (0.02, 0.98), (0.11, 0.96), (0.20, 0.93),
        (0.29, 0.89), (0.37, 0.91), (0.46, 0.88), (0.56, 0.90),
        (0.68, 0.90), (0.82, 0.91), (1.06, 0.92),
    ];
    foreground_ridge(&mountain_profile);

    for depth in 1..=5 {
        let amount = depth as f32 / 6.0;
        let left = egui::pos2(
            rect.left() + amount * rect.width() * 0.30,
            rect.bottom() - amount * terrain_height * 0.70,
        );
        let peak = egui::pos2(
            rect.left() + rect.width() * (0.37 - amount * 0.05),
            horizon + terrain_height * (0.01 + amount * 0.07),
        );
        painter.line_segment(
            [vanishing_point, left],
            egui::Stroke::new(1.0, distant_color),
        );
        painter.line_segment(
            [vanishing_point, peak],
            egui::Stroke::new(1.0, distant_color),
        );
    }

    // Contours follow the mountain profile as the surface recedes.
    for layer in 1..=5 {
        let amount = layer as f32 / 6.0;
        let contour: Vec<egui::Pos2> = mountain_profile
            .iter()
            .map(|(x, y)| {
                let ridge_y = rect.top() + y * rect.height();
                let contour_y = ridge_y + amount * (rect.bottom() - ridge_y);
                egui::pos2(rect.left() + x * rect.width(), contour_y)
            })
            .collect();
        painter.add(egui::Shape::line(
            contour,
            egui::Stroke::new(1.0, distant_color),
        ));
    }

    for x in -8..=8 {
        let bottom_x = rect.left() + rect.width() * (x as f32 / 8.0 + 0.5);
        painter.line_segment(
            [vanishing_point, egui::pos2(bottom_x, rect.bottom())],
            egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(65, 125, 155, 38)),
        );
    }

    for depth in 1..=7 {
        let amount = depth as f32 / 7.0;
        let y = horizon + amount.powi(2) * terrain_height;
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(65, 125, 155, 42)),
        );
    }

    let glow_x = rect.left() + (time * 0.01).sin() * rect.width() * 0.03;
    painter.line_segment(
        [egui::pos2(glow_x, horizon), egui::pos2(glow_x + rect.width() * 0.18, horizon)],
        egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(120, 190, 210, 55)),
    );
}


fn draw_background(ui: &mut egui::Ui) {
    let painter = ui.painter();
    let rect = ui.max_rect();

    let time = ui.input(|i| i.time) as f32;
    let width = rect.width().max(1.0);
    let height = rect.height().max(1.0);

    // Background
    painter.rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgb(10, 15, 25),
    );

    draw_landscape(painter, rect, time);

    // Draw several aircraft
    let trajectories: [(f32, f32, f32, f32, f32); 4] = [
        (60.0, 0.10, 0.12, 0.25, 0.25),
        (35.0, 0.55, 0.10, 0.55, 0.20),
        (20.0, 0.25, 0.08, 0.80, 0.15),
        (15.0, -0.25, 0.8, 1.0, 0.10),
    ];

    for (speed, phase, arc_height, base_height, scale) in trajectories {
        let progress = (phase + time * speed / width).rem_euclid(1.0);
        let arc_angle = progress * std::f32::consts::PI;
        let x = rect.left() + progress * width;
        let y = rect.top() + height * (base_height - arc_height * arc_angle.sin());
        let velocity = egui::vec2(
            speed,
            -arc_height * height * std::f32::consts::PI
                * arc_angle.cos() * speed / width,
        );
        let heading = velocity.x.atan2(-velocity.y);
        let position = egui::pos2(
            x,
            y,
        );
        draw_plane(painter, position, scale, heading);
    }

    // Keep animating
    ui.ctx().request_repaint();
}


impl eframe::App for Settings {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_size().y;

            draw_background(ui);
        
            center_horizontal(ui, |ui| {
                column(ui, Align::Center, |ui| {
                    // Center header/image
                    ui.vertical_centered(|ui| {
                        ui.heading(egui::RichText::new("My VisionOTG")
                            .size(50.0)
                            .color(egui::Color32::WHITE));
                        
                        ui.add_space(10.0); 

                        ui.add(
                            egui::Image::new(
                                egui::include_image!("../assets/images/oswelm_pic.png")
                            )
                            .max_height(available_height * 0.25)
                        );
                    });

                    ui.add_space(20.0); 

                    row(ui, Align::Center, |ui| {
                        ui.add_sized(
                            [96.0, ui.spacing().interact_size.y],
                            egui::Label::new(egui::RichText::new("Camera: ").color(egui::Color32::WHITE))
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.camera)
                                .desired_width(200.0) // Set your desired width here
                        );
                    });

                    ui.add_space(10.0); 

                    row(ui, Align::Center, |ui| {
                        ui.label(egui::RichText::new("Detection Model: ").color(egui::Color32::WHITE));
                        egui::ComboBox::new("model_selection", "")
                            .width(200.0)
                            .selected_text(&self.model)
                            .show_ui(ui, |ui| {
                                for path in &self.onnx_files {
                                    let filename = Path::new(path)
                                        .file_name()
                                        .unwrap()
                                        .to_string_lossy();
                                    ui.selectable_value(
                                        &mut self.model, 
                                        filename.to_string(), filename);
                                }
                            });
                    });

                    ui.add_space(10.0); 

                    let btn_color = egui::Color32::from_rgba_unmultiplied(70, 160, 210, 200);
                    let start_btn = egui::Button::new(egui::RichText::new("START").color(egui::Color32::WHITE))
                        .fill(btn_color);

                    if ui.add_sized([95.0, 25.0], start_btn).clicked() {
                        // Here you can add the logic to start your application with the selected camera and model.
                        match start_application(&self.camera, &self.model,) {
                            Ok(()) => {
                                // Application started successfully
                            }
                            Err(e) => {
                                eprintln!("Error starting application: {}", e);
                            }
                        }
                    }
                    
                });
            });

        });
    }
}