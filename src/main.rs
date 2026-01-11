use eframe::{egui};
use egui::Color32;
use rand::Rng;

const WINDOW_TITLE: &str = "Color Guesser";

fn main() -> eframe::Result<()> {
    eframe::run_native(
        WINDOW_TITLE,
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

struct App {
    target_color: Color32,
    string_guess: String,
    color_guess: Option<Color32>,
    show_color: bool,
    has_guess: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            target_color: random_color(None, None, None),
            string_guess: String::new(),
            color_guess: None,
            show_color: false,
            has_guess: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let guess_button = egui::Button::new("Guess");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Game", |ui| {
                    if ui.button("New Color").clicked() {
                        self.target_color = random_color(None, None, None);
                    }
                    if ui.button("Cheat (show color)").clicked() {
                        self.show_color = !self.show_color;
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Guess a color:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.string_guess);
                    if ui.add(guess_button).clicked() {
                        if get_color_from_hex_string(&self.string_guess).is_some() {
                            self.color_guess = get_color_from_hex_string(&self.string_guess);
                            self.has_guess = true;
                            self.show_color = !self.show_color;
                        } else {
                            dbg!("get_color_from_hex_string is not some!");
                        }
                    }
                });
                ui.add_space(20.0);
                if self.show_color { ui.label(get_text_from_color(&self.target_color)); }

                let rect = get_color_rect(self.target_color, ui.next_widget_position(), egui::vec2(200.0, 100.0));
                ui.painter().add(rect);

                if self.has_guess {
                    ui.add_space(50.0);
                    let guess_rect = get_color_rect(
                        self.color_guess.unwrap_or(Color32::BLACK), 
                        ui.next_widget_position(), 
                        egui::vec2(200.0, 100.0)
                    );
                    ui.painter().add(guess_rect);
                }
            });
        });
    }
}

fn random_color(r: Option<u8>, g: Option<u8>, b: Option<u8>) -> Color32 {
    Color32::from_rgb(
        r.unwrap_or(rand::rng().random()),
        g.unwrap_or(rand::rng().random()),
        b.unwrap_or(rand::rng().random())
    )
}

fn get_color_rect(color: Color32, pos: egui::Pos2, size: egui::Vec2) -> egui::Shape {
    egui::Shape::rect_filled(
        egui::Rect::from_min_size(pos, size), 
        2.0, 
        color
    )
}

fn get_text_from_color(color: &Color32) -> String {
    let [r, g, b, _a] = color.to_array();
    format!("Color: #{:02X}{:02X}{:02X}", r, g, b)
}

fn get_color_from_hex_string(input: &str) -> Option<Color32> {
    let s = input.strip_prefix("#").unwrap_or(input);
    if s.len() != 6 { return None; }

    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Color32::from_rgb(r, g, b))
}