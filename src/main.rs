#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, Ui, Vec2},
    epaint::ColorImage,
    run_native, App, IconData, NativeOptions,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use winsafe;

struct SFE {
    current_path: PathBuf,
    last_path: PathBuf,
    folder_image: ColorImage,
    file_image: ColorImage,
}

impl SFE {
    fn new() -> Self {
        let folder_buffer = image::open(
            "C:\\Users\\Ario\\Desktop\\Dario Zeug\\rust\\file-explorer\\assets\\folder.png",
        )
        .expect("Couldnt load image")
        .resize(20, 24, image::imageops::FilterType::CatmullRom)
        .into_rgba8();
        let folder_image = egui::ColorImage::from_rgba_unmultiplied(
            [
                folder_buffer.width().try_into().unwrap(),
                folder_buffer.height().try_into().unwrap(),
            ],
            &folder_buffer,
        );

        let file_buffer = image::open(
            "C:\\Users\\Ario\\Desktop\\Dario Zeug\\rust\\file-explorer\\assets\\file.png",
        )
        .expect("Couldnt load image")
        .resize(15, 19, image::imageops::FilterType::CatmullRom)
        .into_rgba8();
        let file_image = egui::ColorImage::from_rgba_unmultiplied(
            [
                file_buffer.width().try_into().unwrap(),
                file_buffer.height().try_into().unwrap(),
            ],
            &file_buffer,
        );
        Self {
            current_path: PathBuf::from("Drives"),
            last_path: PathBuf::new(),
            folder_image: folder_image,
            file_image: file_image,
        }
    }

    fn change_display(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if self.current_path.display().to_string() == "Drives" {
            for item in winsafe::GetLogicalDriveStrings().unwrap() {
                if ui.button(&item).double_clicked() {
                    self.current_path.push(&item);
                    self.last_path = PathBuf::from(item);
                }
            }
        } else {
            let folder_handle =
                ctx.load_texture("folder", self.folder_image.clone(), Default::default());
            let file_handle = ctx.load_texture("file", self.file_image.clone(), Default::default());

            let content = fs::read_dir(&self.current_path).unwrap();
            for path in content {
                let md = fs::metadata(path.as_ref().unwrap().path()).unwrap();
                let mut path_as_string = path.as_ref().unwrap().path().display().to_string();
                ui.horizontal(|ui| {
                    // use extension for different icons????
                    // let path_as_raw_path = path.unwrap().path();
                    // let ext = path_as_raw_path.extension();

                    if md.is_dir() {
                        ui.image(&folder_handle, folder_handle.size_vec2());
                    } else if md.is_file() {
                        ui.image(&file_handle, file_handle.size_vec2());
                    }
                    if ui
                        .button(
                            &path_as_string.replace(&self.current_path.display().to_string(), ""),
                        )
                        .double_clicked()
                    {
                        if md.is_dir() {
                            path_as_string.push_str("\\");
                            self.current_path.push(&path_as_string);
                            self.last_path.push(&path_as_string);
                        }

                        if md.is_file() {
                            open::commands(&path_as_string)[0].status();
                        }
                    }
                });
            }
        }
    }

    fn render_header(&mut self, ui: &mut Ui) {
        egui::Grid::new("id_source").show(ui, |ui| {
            if ui.button("back").clicked() {
                // fix back by pushiing / to the end of current_path
                let mut parent = self.current_path.parent();
                if parent == None || parent == Some(Path::new("")) {
                    parent = Some(Path::new("Drives"));
                }
                self.last_path = self.current_path.clone();
                self.current_path = parent.unwrap().to_path_buf();
            }
            if ui.button("prev").clicked() {
                if self.last_path == Path::new("") {
                    self.last_path = PathBuf::from("Drives");
                }
                self.current_path = self.last_path.clone();
            }
            ui.label(self.current_path.display().to_string());
            // implement text field
            // ui.text_edit_singleline(&mut self.current_path.display().to_string());
            ui.end_row();
        });
    }
}

impl App for SFE {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            egui::ScrollArea::new([false, true])
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.change_display(ui, ctx);
                });
        });
    }
}

fn main() {
    // let icon = image::open("123.jpg").expect("Failed to open icon path").to_rgba8();
    // let (icon_width, icon_height) = icon.dimensions();

    let app = SFE::new();
    let mut win_option = NativeOptions::default();

    win_option.initial_window_size = Some(Vec2::new(960.0, 540.0));
    // win_option.icon_data = Some(IconData {
    //     rgba: icon.into_raw(),
    //     width: icon_width,
    //     height: icon_height,
    // });

    let _ = run_native(
        "Dario’s spritziger Dateinsucher",
        win_option,
        Box::new(|_cc| Box::new(app)),
    );
}
