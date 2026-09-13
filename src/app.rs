use eframe::egui;
use egui::Align;
use egui_alignments::{center_horizontal, column, row};
use ndarray::Array3;
use std::path::Path;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};

use crate::camera;
use crate::draw;
use crate::model;

pub enum AppScreen {
    Startup,
    Camera,
}

pub struct App {
    // Camera index or v4l2src camera to run
    pub camera: String,
    // The chosen model (e.g., yolov8n.onnx)
    pub model: String,
    // List of available ONNX model files in the assets/models directory
    pub onnx_files: Vec<std::path::PathBuf>,
    // Input normalization method
    pub norm: model::Normalization,
    // The current screen of the application
    pub screen: AppScreen,
    // Frame receiver contains a GStreamer frame sample
    pub display_rx: Option<mpsc::Receiver<camera::Frame>>,
    // GStreamer camera pipeline
    pub camera_pipeline: Option<gstreamer::Pipeline>,
    // egui camera texture
    pub camera_texture: Option<egui::TextureHandle>,
    // Camera thread for frame access
    pub camera_thread: Option<std::thread::JoinHandle<()>>,
    // Model thread for inference execution
    pub model_thread: Option<std::thread::JoinHandle<()>>,
    // Display thread for rendering frames to the GUI
    pub display_thread: Option<std::thread::JoinHandle<()>>,
    // Global model detections placeholder
    pub detections: model::SharedDetections,
    // Global app shutdown flag
    pub shutdown: Arc<AtomicBool>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            camera: "0".to_owned(),
            model: "yolov8n.onnx".to_owned(),
            onnx_files: Vec::new(),
            norm: model::Normalization::Unsigned,
            screen: AppScreen::Startup,
            display_rx: None,
            camera_pipeline: None,
            camera_texture: None,
            camera_thread: None,
            model_thread: None,
            display_thread: None,
            detections: Arc::new(Mutex::new(Array3::<f32>::zeros((1, 300, 6)))),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl App {
    fn draw_startup_screen(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_size().y;

            draw::draw_background(ui);

            center_horizontal(ui, |ui| {
                column(ui, Align::Center, |ui| {
                    // Center header/image
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("My VisionOTG")
                                .size(50.0)
                                .color(egui::Color32::WHITE),
                        );

                        ui.add_space(10.0);

                        ui.add(
                            egui::Image::new(egui::include_image!(
                                "../assets/images/oswelm_pic.png"
                            ))
                            .max_height(available_height * 0.25),
                        );
                    });

                    ui.add_space(20.0);

                    row(ui, Align::Center, |ui| {
                        ui.label(
                            egui::RichText::new("Camera:                   ")
                                .color(egui::Color32::WHITE),
                        );
                        let cam_response = ui.add(
                            egui::TextEdit::singleline(&mut self.camera).desired_width(200.0), // Set your desired width here
                        );

                        cam_response.on_hover_text(
                            "Specifies the camera device used for video capture.\n\n\
                            Linux: Use a device path, such as '/dev/video0'.\n\
                            Windows: Use the camera's numeric device index, such as '0'.",
                        );
                    });

                    ui.add_space(10.0);

                    row(ui, Align::Center, |ui| {
                        ui.label(
                            egui::RichText::new("Detection Model: ").color(egui::Color32::WHITE),
                        );
                        let model_response = egui::ComboBox::from_id_salt("model_selection")
                            .width(200.0)
                            .selected_text(&self.model)
                            .show_ui(ui, |ui| {
                                for path in &self.onnx_files {
                                    let filename =
                                        Path::new(path).file_name().unwrap().to_string_lossy();
                                    ui.selectable_value(
                                        &mut self.model,
                                        filename.to_string(),
                                        filename,
                                    );
                                }
                            });

                        model_response.response.on_hover_text(
                            "Specifies the object detection model used for inference.\n\n\
                            Models must be in ONNX format and exported from Ultralytics \
                            with embedded Non-Maximum Suppression (NMS).",
                        );
                    });

                    ui.add_space(10.0);

                    row(ui, Align::Center, |ui| {
                        ui.label(
                            egui::RichText::new("Normalization:       ")
                                .color(egui::Color32::WHITE),
                        );
                        let norm_response = egui::ComboBox::from_id_salt("input_normalization")
                            .width(200.0)
                            .selected_text(self.norm.as_str())
                            .show_ui(ui, |ui| {
                                for norm in model::Normalization::str_variants() {
                                    ui.selectable_value(
                                        &mut self.norm,
                                        model::Normalization::from_str(&norm),
                                        norm,
                                    );
                                }
                            });

                        norm_response
                            .response
                            .on_hover_text("The type of model input normalization.");
                    });

                    ui.add_space(10.0);

                    let btn_color = egui::Color32::from_rgba_unmultiplied(70, 160, 210, 200);
                    let start_btn =
                        egui::Button::new(egui::RichText::new("START").color(egui::Color32::WHITE))
                            .fill(btn_color);

                    if ui.add_sized([95.0, 25.0], start_btn).clicked() {
                        match camera::start_camera(&self.camera, self.shutdown.clone()) {
                            Ok((
                                camera_pipeline,
                                display_rx,
                                model_rx,
                                camera_thread,
                                display_thread,
                            )) => {
                                match model::start_model(
                                    &self.model,
                                    self.shutdown.clone(),
                                    model_rx,
                                    self.detections.clone(),
                                    self.norm.clone(),
                                ) {
                                    Ok(model_thread) => {
                                        self.camera_pipeline = Some(camera_pipeline);
                                        self.display_rx = Some(display_rx);
                                        self.camera_thread = Some(camera_thread);
                                        self.model_thread = Some(model_thread);
                                        self.display_thread = Some(display_thread);

                                        self.screen = AppScreen::Camera;
                                    }

                                    Err(e) => {
                                        eprintln!("Failed to start model: {}", e);

                                        // Important: shut the camera back down
                                        // if model startup fails.
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("Error starting camera: {}", e);
                            }
                        }
                    }
                });
            });
        });
    }

    fn draw_camera_feed(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Background
            let painter = ui.painter();
            let rect = ui.max_rect();
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(10, 15, 25));

            // Get the latest camera frame
            if let Some(rx) = &self.display_rx {
                if let Ok(frame) = rx.try_recv() {
                    let image =
                        egui::ColorImage::from_rgb([frame.width, frame.height], &frame.pixels);

                    if let Some(texture) = &mut self.camera_texture {
                        texture.set(image, egui::TextureOptions::LINEAR);
                    } else {
                        self.camera_texture = Some(ui.ctx().load_texture(
                            "camera",
                            image,
                            egui::TextureOptions::LINEAR,
                        ));
                    }
                }
            }

            // Display camera
            if let Some(texture) = &self.camera_texture {
                let available = ui.available_size();
                center_horizontal(ui, |ui| {
                    let response = ui.add(egui::Image::new(texture).max_size(available));
                    let image_rect = response.rect;
                    let painter = ui.painter_at(image_rect);

                    draw::draw_overlay(&painter, image_rect, &self.detections);
                });
            } else {
                ui.label("Waiting for camera frame...");
            }

            // Keep egui rendering
            ui.ctx().request_repaint();
        });
    }

    fn stop_app(&mut self) {
        println!("Stopping VisionOTG...");

        // Tell the camera thread to stop.
        let shutdown = &self.shutdown;
        shutdown.store(true, Ordering::Relaxed);

        // Stop the GStreamer pipeline.
        if let Some(camera_pipeline) = &self.camera_pipeline {
            camera::cleanup(camera_pipeline);
        }

        // Wait for the camera thread to exit.
        if let Some(camera_thread) = self.camera_thread.take() {
            println!("Joining camera thread");

            if let Err(e) = camera_thread.join() {
                eprintln!("Failed to join camera thread: {:?}", e);
            }
        }

        // Wait for the model thread to exit.
        if let Some(model_thread) = self.model_thread.take() {
            println!("Joining model thread");

            if let Err(e) = model_thread.join() {
                eprintln!("Failed to join model thread: {:?}", e);
            }
        }

        // Wait for the display thread to exit.
        if let Some(display_thread) = self.display_thread.take() {
            println!("Joining display thread");

            if let Err(e) = display_thread.join() {
                eprintln!("Failed to join display thread: {:?}", e);
            }
        }

        // Remove the remaining resources.
        self.camera_pipeline = None;
        self.display_rx = None;
        self.shutdown = Arc::new(AtomicBool::new(false));
        self.camera_texture = None;

        println!("VisionOTG shutdown complete");
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.screen {
            AppScreen::Startup => {
                self.draw_startup_screen(ui);
            }

            AppScreen::Camera => {
                self.draw_camera_feed(ui);
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_app();
    }
}
