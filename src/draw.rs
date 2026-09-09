use cairo::Context;
use ndarray::Axis;

use crate::model::SharedDetections;

struct Object {
    class: &'static str,
    color: (f64, f64, f64),
}

const OBJECTS: [Object; 80] = [
    Object {
        class: "person",
        color: (0.0, 1.0, 0.0),
    },
    Object {
        class: "bicycle",
        color: (1.0, 0.0, 0.0),
    },
    Object {
        class: "car",
        color: (0.0, 0.0, 1.0),
    },
    Object {
        class: "motorcycle",
        color: (1.0, 1.0, 0.0),
    },
    Object {
        class: "airplane",
        color: (1.0, 0.0, 1.0),
    },
    Object {
        class: "bus",
        color: (0.0, 1.0, 1.0),
    },
    Object {
        class: "train",
        color: (1.0, 0.5, 0.0),
    },
    Object {
        class: "truck",
        color: (0.5, 0.0, 1.0),
    },
    Object {
        class: "boat",
        color: (0.0, 0.5, 1.0),
    },
    Object {
        class: "traffic light",
        color: (0.5, 1.0, 0.0),
    },
    Object {
        class: "fire hydrant",
        color: (1.0, 0.3, 0.3),
    },
    Object {
        class: "stop sign",
        color: (0.3, 1.0, 0.3),
    },
    Object {
        class: "parking meter",
        color: (0.3, 0.3, 1.0),
    },
    Object {
        class: "bench",
        color: (1.0, 0.7, 0.3),
    },
    Object {
        class: "bird",
        color: (0.7, 0.3, 1.0),
    },
    Object {
        class: "cat",
        color: (0.3, 1.0, 0.7),
    },
    Object {
        class: "dog",
        color: (1.0, 0.3, 0.7),
    },
    Object {
        class: "horse",
        color: (0.7, 1.0, 0.3),
    },
    Object {
        class: "sheep",
        color: (0.3, 0.7, 1.0),
    },
    Object {
        class: "cow",
        color: (1.0, 0.5, 0.5),
    },
    Object {
        class: "elephant",
        color: (0.5, 1.0, 0.5),
    },
    Object {
        class: "bear",
        color: (0.5, 0.5, 1.0),
    },
    Object {
        class: "zebra",
        color: (1.0, 0.8, 0.5),
    },
    Object {
        class: "giraffe",
        color: (0.8, 0.5, 1.0),
    },
    Object {
        class: "backpack",
        color: (0.5, 1.0, 0.8),
    },
    Object {
        class: "umbrella",
        color: (1.0, 0.5, 0.8),
    },
    Object {
        class: "handbag",
        color: (0.8, 1.0, 0.5),
    },
    Object {
        class: "tie",
        color: (0.5, 0.8, 1.0),
    },
    Object {
        class: "suitcase",
        color: (1.0, 0.6, 0.6),
    },
    Object {
        class: "frisbee",
        color: (0.6, 1.0, 0.6),
    },
    Object {
        class: "skis",
        color: (0.6, 0.6, 1.0),
    },
    Object {
        class: "snowboard",
        color: (1.0, 0.9, 0.6),
    },
    Object {
        class: "sports ball",
        color: (0.9, 0.6, 1.0),
    },
    Object {
        class: "kite",
        color: (0.6, 1.0, 0.9),
    },
    Object {
        class: "baseball bat",
        color: (1.0, 0.6, 0.9),
    },
    Object {
        class: "baseball glove",
        color: (0.9, 1.0, 0.6),
    },
    Object {
        class: "skateboard",
        color: (0.6, 0.9, 1.0),
    },
    Object {
        class: "surfboard",
        color: (1.0, 0.7, 0.7),
    },
    Object {
        class: "tennis racket",
        color: (0.7, 1.0, 0.7),
    },
    Object {
        class: "bottle",
        color: (0.7, 0.7, 1.0),
    },
    Object {
        class: "wine glass",
        color: (1.0, 0.8, 0.7),
    },
    Object {
        class: "cup",
        color: (0.8, 1.0, 0.7),
    },
    Object {
        class: "fork",
        color: (0.7, 0.8, 1.0),
    },
    Object {
        class: "knife",
        color: (1.0, 0.7, 0.8),
    },
    Object {
        class: "spoon",
        color: (0.8, 1.0, 0.8),
    },
    Object {
        class: "bowl",
        color: (0.8, 0.8, 1.0),
    },
    Object {
        class: "banana",
        color: (1.0, 0.85, 0.6),
    },
    Object {
        class: "apple",
        color: (0.85, 1.0, 0.6),
    },
    Object {
        class: "sandwich",
        color: (0.6, 0.85, 1.0),
    },
    Object {
        class: "orange",
        color: (1.0, 0.6, 0.85),
    },
    Object {
        class: "broccoli",
        color: (0.85, 1.0, 0.6),
    },
    Object {
        class: "carrot",
        color: (0.6, 1.0, 0.85),
    },
    Object {
        class: "hot dog",
        color: (1.0, 0.6, 0.85),
    },
    Object {
        class: "pizza",
        color: (0.85, 0.6, 1.0),
    },
    Object {
        class: "donut",
        color: (0.6, 0.85, 1.0),
    },
    Object {
        class: "cake",
        color: (1.0, 0.85, 0.6),
    },
    Object {
        class: "chair",
        color: (0.4, 0.4, 0.4),
    },
    Object {
        class: "couch",
        color: (0.7, 0.4, 0.2),
    },
    Object {
        class: "potted plant",
        color: (0.2, 0.7, 0.4),
    },
    Object {
        class: "bed",
        color: (0.4, 0.2, 0.7),
    },
    Object {
        class: "dining table",
        color: (0.7, 0.7, 0.2),
    },
    Object {
        class: "toilet",
        color: (0.2, 0.7, 0.7),
    },
    Object {
        class: "tv",
        color: (0.7, 0.2, 0.7),
    },
    Object {
        class: "laptop",
        color: (0.9, 0.4, 0.2),
    },
    Object {
        class: "mouse",
        color: (0.2, 0.9, 0.4),
    },
    Object {
        class: "remote",
        color: (0.4, 0.2, 0.9),
    },
    Object {
        class: "keyboard",
        color: (0.9, 0.9, 0.2),
    },
    Object {
        class: "cell phone",
        color: (0.2, 0.9, 0.9),
    },
    Object {
        class: "microwave",
        color: (0.9, 0.2, 0.9),
    },
    Object {
        class: "oven",
        color: (0.5, 0.5, 0.5),
    },
    Object {
        class: "toaster",
        color: (0.3, 0.3, 0.3),
    },
    Object {
        class: "sink",
        color: (0.6, 0.3, 0.1),
    },
    Object {
        class: "refrigerator",
        color: (0.1, 0.6, 0.3),
    },
    Object {
        class: "book",
        color: (0.3, 0.1, 0.6),
    },
    Object {
        class: "clock",
        color: (0.6, 0.6, 0.1),
    },
    Object {
        class: "vase",
        color: (0.1, 0.6, 0.6),
    },
    Object {
        class: "scissors",
        color: (0.6, 0.1, 0.6),
    },
    Object {
        class: "teddy bear",
        color: (0.8, 0.8, 0.8),
    },
    Object {
        class: "hair drier",
        color: (0.5, 0.2, 0.8),
    },
    Object {
        class: "toothbrush",
        color: (0.8, 0.5, 0.2),
    },
];

// -----------------------------------------------------
// DRAW CALLBACK
// -----------------------------------------------------
pub fn draw_overlay(cr: &Context, detections: &SharedDetections) {
    let dets = detections
        .lock()
        .expect("Failed to lock global detections placeholder");
    let dets_ = dets.index_axis(Axis(0), 0);

    match cr.clip_extents() {
        Ok((x1, y1, x2, y2)) => {
            let width = x2 - x1;
            let height = y2 - y1;

            cr.set_line_width(3.0);

            // Draw bounding boxes
            for d in dets_.outer_iter() {
                let xmin = d[0] as f64;
                let ymin = d[1] as f64;
                let xmax = d[2] as f64;
                let ymax = d[3] as f64;
                let score = d[4];
                let class = d[5];

                if score < 0.25 {
                    continue;
                }

                let obj = OBJECTS.get(class as usize).unwrap_or(&Object {
                    class: "unknown",
                    color: (1.0, 1.0, 1.0),
                });

                if (xmax - xmin).min(ymax - ymin) < 0.20 {
                    let cx = (xmin + xmax) * 0.50 * width;
                    let cy = (ymin + ymax) * 0.50 * height;
                    let radius = ((xmax - xmin) * width).max((ymax - ymin) * height) / 2.0;

                    draw_reticle(cr, cx, cy, radius, obj.class, score as f64, obj.color);
                } else {
                    let obj_width = (xmax - xmin) * width;
                    let obj_height = (ymax - ymin) * height;

                    draw_box_panel(
                        cr,
                        xmin * width,
                        ymin * height,
                        obj_width,
                        obj_height,
                        obj.class,
                        score as f64,
                        obj.color,
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get clip extents: {}", e);
        }
    };
}

fn draw_reticle(
    cr: &Context,
    cx: f64,
    cy: f64,
    radius: f64,
    label: &str,
    confidence: f64,
    color: (f64, f64, f64),
) {
    // ---------------------------
    // Outer glow
    // ---------------------------
    cr.set_source_rgba(0.0, 1.0, 1.0, 0.20);
    cr.set_line_width(8.0);
    cr.arc(cx, cy, radius + 4.0, 0.0, 2.0 * std::f64::consts::PI);
    cr.stroke().expect("Failed to draw reticle element");

    // ---------------------------
    // Main ring
    // ---------------------------
    cr.set_source_rgb(0.0, 1.0, 1.0);
    cr.set_line_width(2.5);
    cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.stroke().expect("Failed to draw reticle element");

    // ---------------------------
    // Rotating segmented arc
    // ---------------------------
    let angle = 0.0;

    cr.set_line_width(5.0);
    cr.arc(cx, cy, radius, angle, angle + std::f64::consts::PI / 2.0);
    cr.stroke().expect("Failed to draw reticle element");

    // ---------------------------
    // Inner ring
    // ---------------------------
    cr.set_line_width(1.5);
    cr.arc(cx, cy, radius - 10.0, 0.0, 2.0 * std::f64::consts::PI);
    cr.stroke().expect("Failed to draw reticle element");

    // ==========================
    // Label folder tab
    // ==========================
    cr.select_font_face(
        "Consolas",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Bold,
    );
    cr.set_font_size(13.0);

    let text = format!("{} {:.0}%", label, confidence * 100.0);
    let extents = cr
        .text_extents(&text)
        .expect("Failed to draw box panel element");

    let padding_x = 15.0;
    let padding_y = 8.0;

    let tab_w = extents.width() + padding_x * 2.0;
    let tab_h = extents.height() + padding_y * 2.0;

    let tab_x = cx - radius / 2.0 + 10.0;
    let tab_y = cy - radius - tab_h;

    cr.new_path();
    cr.move_to(tab_x, tab_y + tab_h);
    cr.line_to(tab_x, tab_y);
    cr.line_to(tab_x + 24.0, tab_y);
    cr.line_to(tab_x + tab_w, tab_y);
    cr.line_to(tab_x + tab_w, tab_y + tab_h);
    cr.line_to(tab_x, tab_y + tab_h);

    // Fill label background
    // cr.set_source_rgba(0.0, 0.15, 0.18, 0.8);
    cr.set_source_rgba(color.0, color.1, color.2, 0.2);
    cr.fill_preserve()
        .expect("Failed to fill box panel element");

    // Tab outline
    cr.set_source_rgb(0.0, 1.0, 1.0);
    cr.set_line_width(2.0);

    cr.stroke().expect("Failed to draw box panel element");

    // ==========================
    // Label text
    // ==========================
    cr.move_to(tab_x + padding_x, tab_y + padding_y + extents.height());
    cr.show_text(&text)
        .expect("Failed to draw text for box panel element");

    // ---------------------------
    // Tick marks
    // ---------------------------
    for i in 0..32 {
        let a = i as f64 * std::f64::consts::PI * 2.0 / 32.0;

        let r1 = radius + 4.0;
        let r2 = radius + if i % 4 == 0 { 12.0 } else { 8.0 };

        cr.move_to(cx + r1 * a.cos(), cy + r1 * a.sin());

        cr.line_to(cx + r2 * a.cos(), cy + r2 * a.sin());
    }

    cr.set_line_width(1.0);
    cr.stroke().expect("Failed to draw reticle element");

    // Ok(())
}

pub fn draw_box_panel(
    cr: &cairo::Context,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    label: &str,
    confidence: f64,
    color: (f64, f64, f64),
) {
    let corner = 12.0;

    // ==========================
    // Main panel path
    // ==========================
    let draw_panel = |cr: &cairo::Context| {
        cr.new_path();

        cr.move_to(x + corner, y);
        cr.line_to(x + w - corner, y);
        cr.line_to(x + w, y + corner);
        cr.line_to(x + w, y + h - corner);
        cr.line_to(x + w - corner, y + h);
        cr.line_to(x + corner, y + h);
        cr.line_to(x, y + h - corner);
        cr.line_to(x, y + corner);
        cr.close_path();
    };

    // ==========================
    // Transparent HUD background
    // ==========================
    draw_panel(cr);

    // cr.set_source_rgba(0.0, 0.15, 0.18, 0.10);
    // cr.fill().expect("Failed to fill box panel element");

    // ==========================
    // Glow outline
    // ==========================
    draw_panel(cr);

    cr.set_source_rgba(0.0, 1.0, 1.0, 0.25);
    cr.set_line_width(5.0);

    cr.stroke().expect("Failed to draw box panel element");

    // ==========================
    // Main outline
    // ==========================
    draw_panel(cr);

    cr.set_source_rgb(0.0, 1.0, 1.0);
    cr.set_line_width(2.0);
    cr.stroke().expect("Failed to draw box panel element");

    // ==========================
    // Label folder tab
    // ==========================
    cr.select_font_face(
        "Consolas",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Bold,
    );
    cr.set_font_size(13.0);

    let text = format!("{} {:.0}%", label, confidence * 100.0);
    let extents = cr
        .text_extents(&text)
        .expect("Failed to draw box panel element");

    let padding_x = 15.0;
    let padding_y = 8.0;

    let tab_w = extents.width() + padding_x * 2.0;
    let tab_h = extents.height() + padding_y * 2.0;

    let tab_x = x + 10.0;
    let tab_y = y - tab_h;

    cr.new_path();
    cr.move_to(tab_x, tab_y + tab_h);
    cr.line_to(tab_x, tab_y);
    cr.line_to(tab_x + 24.0, tab_y);
    cr.line_to(tab_x + tab_w, tab_y);
    cr.line_to(tab_x + tab_w, tab_y + tab_h);

    // Fill label background
    // cr.set_source_rgba(0.0, 0.15, 0.18, 0.8);
    cr.set_source_rgba(color.0, color.1, color.2, 0.2);
    cr.fill_preserve()
        .expect("Failed to fill box panel element");

    // Tab outline
    cr.set_source_rgb(0.0, 1.0, 1.0);
    cr.set_line_width(2.0);

    cr.stroke().expect("Failed to draw box panel element");

    // ==========================
    // Label text
    // ==========================
    cr.move_to(tab_x + padding_x, tab_y + padding_y + extents.height());
    cr.show_text(&text)
        .expect("Failed to draw text for box panel element");

    // ==========================
    // Bottom HUD ticks
    // ==========================
    let bottom = y + h;
    cr.new_path();
    cr.move_to(x + 15.0, bottom);
    cr.line_to(x + 40.0, bottom);

    cr.move_to(x + w / 2.0 - 20.0, bottom);
    cr.line_to(x + w / 2.0 + 20.0, bottom);

    cr.move_to(x + w - 40.0, bottom);
    cr.line_to(x + w - 15.0, bottom);

    for i in 0..5 {
        let xx = x + w / 2.0 - 12.0 + i as f64 * 6.0;

        cr.move_to(xx, bottom - 7.0);
        cr.line_to(xx + 3.0, bottom);
    }

    cr.stroke().expect("Failed to draw box panel element");
}
