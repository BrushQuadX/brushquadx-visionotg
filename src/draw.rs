use ndarray::Axis;

use eframe::egui::{self, Align2};

use crate::model::SharedDetections;

struct Object {
    class: &'static str,
    color: (u8, u8, u8),
}

const OBJECTS: [Object; 80] = [
    Object {
        class: "person",
        color: (0, 255, 0),
    },
    Object {
        class: "bicycle",
        color: (255, 0, 0),
    },
    Object {
        class: "car",
        color: (0, 0, 255),
    },
    Object {
        class: "motorcycle",
        color: (255, 255, 0),
    },
    Object {
        class: "airplane",
        color: (255, 0, 255),
    },
    Object {
        class: "bus",
        color: (0, 255, 255),
    },
    Object {
        class: "train",
        color: (255, 128, 0),
    },
    Object {
        class: "truck",
        color: (128, 0, 255),
    },
    Object {
        class: "boat",
        color: (0, 128, 255),
    },
    Object {
        class: "traffic light",
        color: (128, 255, 0),
    },
    Object {
        class: "fire hydrant",
        color: (255, 77, 77),
    },
    Object {
        class: "stop sign",
        color: (77, 255, 77),
    },
    Object {
        class: "parking meter",
        color: (77, 77, 255),
    },
    Object {
        class: "bench",
        color: (255, 179, 77),
    },
    Object {
        class: "bird",
        color: (179, 77, 255),
    },
    Object {
        class: "cat",
        color: (77, 255, 179),
    },
    Object {
        class: "dog",
        color: (255, 77, 179),
    },
    Object {
        class: "horse",
        color: (179, 255, 77),
    },
    Object {
        class: "sheep",
        color: (77, 179, 255),
    },
    Object {
        class: "cow",
        color: (255, 128, 128),
    },
    Object {
        class: "elephant",
        color: (128, 255, 128),
    },
    Object {
        class: "bear",
        color: (128, 128, 255),
    },
    Object {
        class: "zebra",
        color: (255, 204, 128),
    },
    Object {
        class: "giraffe",
        color: (204, 128, 255),
    },
    Object {
        class: "backpack",
        color: (128, 255, 204),
    },
    Object {
        class: "umbrella",
        color: (255, 128, 204),
    },
    Object {
        class: "handbag",
        color: (204, 255, 128),
    },
    Object {
        class: "tie",
        color: (128, 204, 255),
    },
    Object {
        class: "suitcase",
        color: (255, 153, 153),
    },
    Object {
        class: "frisbee",
        color: (153, 255, 153),
    },
    Object {
        class: "skis",
        color: (153, 153, 255),
    },
    Object {
        class: "snowboard",
        color: (255, 230, 153),
    },
    Object {
        class: "sports ball",
        color: (230, 153, 255),
    },
    Object {
        class: "kite",
        color: (153, 255, 230),
    },
    Object {
        class: "baseball bat",
        color: (255, 153, 230),
    },
    Object {
        class: "baseball glove",
        color: (230, 255, 153),
    },
    Object {
        class: "skateboard",
        color: (153, 230, 255),
    },
    Object {
        class: "surfboard",
        color: (255, 179, 179),
    },
    Object {
        class: "tennis racket",
        color: (179, 255, 179),
    },
    Object {
        class: "bottle",
        color: (179, 179, 255),
    },
    Object {
        class: "wine glass",
        color: (255, 204, 179),
    },
    Object {
        class: "cup",
        color: (204, 255, 179),
    },
    Object {
        class: "fork",
        color: (179, 204, 255),
    },
    Object {
        class: "knife",
        color: (255, 179, 204),
    },
    Object {
        class: "spoon",
        color: (204, 255, 204),
    },
    Object {
        class: "bowl",
        color: (204, 204, 255),
    },
    Object {
        class: "banana",
        color: (255, 217, 153),
    },
    Object {
        class: "apple",
        color: (217, 255, 153),
    },
    Object {
        class: "sandwich",
        color: (153, 217, 255),
    },
    Object {
        class: "orange",
        color: (255, 153, 217),
    },
    Object {
        class: "broccoli",
        color: (217, 255, 153),
    },
    Object {
        class: "carrot",
        color: (153, 255, 217),
    },
    Object {
        class: "hot dog",
        color: (255, 153, 217),
    },
    Object {
        class: "pizza",
        color: (217, 153, 255),
    },
    Object {
        class: "donut",
        color: (153, 217, 255),
    },
    Object {
        class: "cake",
        color: (255, 217, 153),
    },
    Object {
        class: "chair",
        color: (102, 102, 102),
    },
    Object {
        class: "couch",
        color: (179, 102, 51),
    },
    Object {
        class: "potted plant",
        color: (51, 179, 102),
    },
    Object {
        class: "bed",
        color: (102, 51, 179),
    },
    Object {
        class: "dining table",
        color: (179, 179, 51),
    },
    Object {
        class: "toilet",
        color: (51, 179, 179),
    },
    Object {
        class: "tv",
        color: (179, 51, 179),
    },
    Object {
        class: "laptop",
        color: (230, 102, 51),
    },
    Object {
        class: "mouse",
        color: (51, 230, 102),
    },
    Object {
        class: "remote",
        color: (102, 51, 230),
    },
    Object {
        class: "keyboard",
        color: (230, 230, 51),
    },
    Object {
        class: "cell phone",
        color: (51, 230, 230),
    },
    Object {
        class: "microwave",
        color: (230, 51, 230),
    },
    Object {
        class: "oven",
        color: (128, 128, 128),
    },
    Object {
        class: "toaster",
        color: (77, 77, 77),
    },
    Object {
        class: "sink",
        color: (153, 77, 26),
    },
    Object {
        class: "refrigerator",
        color: (26, 153, 77),
    },
    Object {
        class: "book",
        color: (77, 26, 153),
    },
    Object {
        class: "clock",
        color: (153, 153, 26),
    },
    Object {
        class: "vase",
        color: (26, 153, 153),
    },
    Object {
        class: "scissors",
        color: (153, 26, 153),
    },
    Object {
        class: "teddy bear",
        color: (204, 204, 204),
    },
    Object {
        class: "hair drier",
        color: (128, 51, 204),
    },
    Object {
        class: "toothbrush",
        color: (204, 128, 51),
    },
];

fn draw_plane(painter: &egui::Painter, position: egui::Pos2, scale: f32, angle: f32) {
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

    let plane_color = egui::Color32::from_rgba_unmultiplied(70, 160, 210, 200);

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
        transform(egui::pos2(-8.0, -20.0)),  // wing root, front
        transform(egui::pos2(-17.0, -16.0)), // start of sweep
        transform(egui::pos2(-58.0, 20.0)),  // wing tip
        transform(egui::pos2(-53.0, 27.0)),  // wing tip rear
        transform(egui::pos2(-13.0, 8.0)),   // trailing edge
        transform(egui::pos2(-8.0, 15.0)),   // root rear
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
        painter.add(egui::Shape::line(translated, egui::Stroke::new(1.0, color)));
    };

    ridge(
        &[
            (-0.10, 0.01),
            (0.05, 0.14),
            (0.14, 0.06),
            (0.25, 0.25),
            (0.34, 0.09),
            (0.46, 0.19),
            (0.56, 0.05),
            (0.68, 0.22),
            (0.78, 0.08),
            (0.91, 0.18),
            (1.10, 0.02),
        ],
        distant_color,
    );
    ridge(
        &[
            (-0.08, 0.0),
            (0.08, 0.08),
            (0.18, 0.03),
            (0.30, 0.18),
            (0.38, 0.06),
            (0.51, 0.29),
            (0.60, 0.10),
            (0.73, 0.20),
            (0.84, 0.05),
            (0.96, 0.14),
            (1.08, 0.0),
        ],
        line_color,
    );

    let vanishing_point = egui::pos2(rect.center().x, horizon);

    let foreground_ridge = |points: &[(f32, f32)]| {
        let translated: Vec<egui::Pos2> = points
            .iter()
            .map(|(x, y)| {
                egui::pos2(
                    rect.left() + x * rect.width(),
                    rect.top() + y * rect.height(),
                )
            })
            .collect();
        painter.add(egui::Shape::line(
            translated,
            egui::Stroke::new(1.2, line_color),
        ));
    };

    // Foreground mountain planes rise out of the lower-left corner.
    let mountain_profile = [
        (-0.06, 1.02),
        (0.02, 0.98),
        (0.11, 0.96),
        (0.20, 0.93),
        (0.29, 0.89),
        (0.37, 0.91),
        (0.46, 0.88),
        (0.56, 0.90),
        (0.68, 0.90),
        (0.82, 0.91),
        (1.06, 0.92),
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
        [
            egui::pos2(glow_x, horizon),
            egui::pos2(glow_x + rect.width() * 0.18, horizon),
        ],
        egui::Stroke::new(
            2.0,
            egui::Color32::from_rgba_unmultiplied(120, 190, 210, 55),
        ),
    );
}

pub fn draw_background(ui: &mut egui::Ui) {
    let painter = ui.painter();
    let rect = ui.max_rect();

    let time = ui.input(|i| i.time) as f32;
    let width = rect.width().max(1.0);
    let height = rect.height().max(1.0);

    // Background
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(10, 15, 25));

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
            -arc_height * height * std::f32::consts::PI * arc_angle.cos() * speed / width,
        );
        let heading = velocity.x.atan2(-velocity.y);
        let position = egui::pos2(x, y);
        draw_plane(painter, position, scale, heading);
    }

    // Keep animating
    ui.ctx().request_repaint();
}

pub fn draw_overlay(
    painter: &egui::Painter,
    image_rect: egui::Rect,
    detections: &SharedDetections,
) {
    let dets = detections
        .lock()
        .expect("Failed to lock global detections placeholder");
    let dets_ = dets.index_axis(Axis(0), 0);

    let origin = image_rect.min;
    let width = image_rect.width();
    let height = image_rect.height();

    // Draw bounding boxes
    for d in dets_.outer_iter() {
        let xmin = d[0];
        let ymin = d[1];
        let xmax = d[2];
        let ymax = d[3];
        let score = d[4];
        let class = d[5];

        if score < 0.25 {
            continue;
        }

        let obj = OBJECTS.get(class as usize).unwrap_or(&Object {
            class: "unknown",
            color: (255, 255, 255),
        });

        if (xmax - xmin).min(ymax - ymin) < 0.20 {
            let cx = origin.x + (xmin + xmax) * 0.50 * width;
            let cy = origin.y + (ymin + ymax) * 0.50 * height;
            let radius = ((xmax - xmin) * width).max((ymax - ymin) * height) / 2.0;

            draw_reticle(painter, cx, cy, radius, obj.class, score, obj.color);
        } else {
            let obj_width = (xmax - xmin) * width;
            let obj_height = (ymax - ymin) * height;

            draw_box_panel(
                painter,
                origin.x + xmin * width,
                origin.y + ymin * height,
                obj_width,
                obj_height,
                obj.class,
                score,
                obj.color,
            );
        }
    }
}

fn draw_arc(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    stroke: egui::Stroke,
) {
    let segments = 32;

    let points: Vec<egui::Pos2> = (0..=segments)
        .map(|i| {
            let t = i as f32 / segments as f32;
            let angle = start_angle + (end_angle - start_angle) * t;

            egui::pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect();

    painter.add(egui::Shape::line(points, stroke));
}

fn draw_reticle(
    painter: &egui::Painter,
    cx: f32,
    cy: f32,
    radius: f32,
    label: &str,
    confidence: f32,
    color: (u8, u8, u8),
) {
    // ---------------------------
    // Outer glow
    // ---------------------------
    painter.add(egui::Shape::circle_stroke(
        egui::pos2(cx, cy),
        radius + 4.0,
        egui::Stroke::new(8.0, egui::Color32::from_rgba_unmultiplied(0, 255, 255, 20)),
    ));

    // ---------------------------
    // Main ring
    // ---------------------------
    painter.add(egui::Shape::circle_stroke(
        egui::pos2(cx, cy),
        radius,
        egui::Stroke::new(2.5, egui::Color32::from_rgb(0, 255, 255)),
    ));

    // ---------------------------
    // Rotating segmented arc
    // ---------------------------
    let angle = 0.0;

    draw_arc(
        painter,
        egui::pos2(cx, cy),
        radius,
        angle,
        angle + std::f32::consts::PI / 2.0,
        egui::Stroke::new(5.0, egui::Color32::from_rgb(0, 255, 255)),
    );

    // ---------------------------
    // Inner ring
    // ---------------------------
    painter.add(egui::Shape::circle_stroke(
        egui::pos2(cx, cy),
        radius - 10.0,
        egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 255, 255)),
    ));

    // ==========================
    // Label folder tab
    // ==========================

    let font_id = egui::FontId::proportional(13.0);
    let text = format!("{} {:.0}%", label, confidence * 100.0);
    let extents = painter.layout_no_wrap(
        text.clone(),
        font_id.clone(),
        egui::Color32::from_rgb(color.0, color.1, color.2),
    );

    let padding_x = 30.0;
    let padding_y = 8.0;

    let tab_w = extents.size().x + padding_x * 2.0;
    let tab_h = extents.size().y + padding_y * 2.0;

    let tab_x = cx - radius / 2.0 + 10.0;
    let tab_y = cy - radius - tab_h;

    let points = vec![
        egui::pos2(tab_x, tab_y + tab_h),
        egui::pos2(tab_x, tab_y),
        egui::pos2(tab_x + 24.0, tab_y),
        egui::pos2(tab_x + tab_w, tab_y),
        egui::pos2(tab_x + tab_w, tab_y + tab_h),
    ];

    painter.add(egui::Shape::convex_polygon(
        points,
        egui::Color32::from_rgba_unmultiplied(color.0, color.1, color.2, 51),
        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 255)),
    ));

    painter.text(
        egui::pos2(tab_x + tab_w / 2.0, tab_y + tab_h / 2.0),
        Align2::CENTER_CENTER,
        text,
        font_id,
        egui::Color32::from_rgb(color.0, color.1, color.2),
    );

    // ---------------------------
    // Tick marks
    // ---------------------------
    for i in 0..32 {
        let a = i as f32 * std::f32::consts::PI * 2.0 / 32.0;

        let r1 = radius + 4.0;
        let r2 = radius + if i % 4 == 0 { 12.0 } else { 8.0 };

        let p1 = egui::pos2(cx + r1 * a.cos(), cy + r1 * a.sin());

        let p2 = egui::pos2(cx + r2 * a.cos(), cy + r2 * a.sin());

        painter.line_segment(
            [p1, p2],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255)),
        );
    }
}

pub fn draw_box_panel(
    painter: &egui::Painter,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    confidence: f32,
    color: (u8, u8, u8),
) {
    let corner = 12.0;

    let panel_points = vec![
        egui::pos2(x + corner, y),
        egui::pos2(x + w - corner, y),
        egui::pos2(x + w, y + corner),
        egui::pos2(x + w, y + h - corner),
        egui::pos2(x + w - corner, y + h),
        egui::pos2(x + corner, y + h),
        egui::pos2(x, y + h - corner),
        egui::pos2(x, y + corner),
    ];

    let cyan = egui::Color32::from_rgb(0, 255, 255);
    painter.add(egui::Shape::convex_polygon(
        panel_points.clone(),
        egui::Color32::from_rgba_unmultiplied(0, 38, 46, 26),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::closed_line(
        panel_points.clone(),
        egui::Stroke::new(5.0, egui::Color32::from_rgba_unmultiplied(0, 255, 255, 64)),
    ));
    painter.add(egui::Shape::closed_line(
        panel_points,
        egui::Stroke::new(2.0, cyan),
    ));

    let text = format!("{} {:.0}%", label, confidence * 100.0);
    let font_id = egui::FontId::monospace(13.0);
    let text_color = egui::Color32::from_rgb(color.0, color.1, color.2);
    let extents = painter.layout_no_wrap(text.clone(), font_id.clone(), text_color);
    let padding_x = 15.0;
    let padding_y = 8.0;

    let tab_w = extents.size().x + padding_x * 2.0;
    let tab_h = extents.size().y + padding_y * 2.0;

    let tab_x = x + 10.0;
    let tab_y = y - tab_h;

    let tab_points = vec![
        egui::pos2(tab_x, tab_y + tab_h),
        egui::pos2(tab_x, tab_y),
        egui::pos2(tab_x + 24.0, tab_y),
        egui::pos2(tab_x + tab_w, tab_y),
        egui::pos2(tab_x + tab_w, tab_y + tab_h),
    ];
    painter.add(egui::Shape::convex_polygon(
        tab_points.clone(),
        egui::Color32::from_rgba_unmultiplied(color.0, color.1, color.2, 51),
        egui::Stroke::NONE,
    ));
    painter.add(egui::Shape::line(tab_points, egui::Stroke::new(2.0, cyan)));
    painter.text(
        egui::pos2(tab_x + padding_x, tab_y + padding_y),
        Align2::LEFT_TOP,
        text,
        font_id,
        text_color,
    );

    let bottom = y + h;
    let mut ticks = vec![
        [egui::pos2(x + 15.0, bottom), egui::pos2(x + 40.0, bottom)],
        [
            egui::pos2(x + w / 2.0 - 20.0, bottom),
            egui::pos2(x + w / 2.0 + 20.0, bottom),
        ],
        [
            egui::pos2(x + w - 40.0, bottom),
            egui::pos2(x + w - 15.0, bottom),
        ],
    ];

    for i in 0..5 {
        let xx = x + w / 2.0 - 12.0 + i as f32 * 6.0;
        ticks.push([egui::pos2(xx, bottom - 7.0), egui::pos2(xx + 3.0, bottom)]);
    }

    for segment in ticks {
        painter.line_segment(segment, egui::Stroke::new(2.0, cyan));
    }
}
