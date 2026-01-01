use eframe::{egui, App};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

/// ================= CONFIG =================

#[derive(Deserialize, Serialize)]
struct Config {
    auto_paste: Option<bool>,
    remember_last_category: Option<bool>,
    last_category: Option<usize>,
    kaomoji: Option<KaomojiSection>,
}

#[derive(Deserialize, Serialize)]
struct KaomojiSection {
    items: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_paste: Some(true),
            remember_last_category: Some(true),
            last_category: Some(0),
            kaomoji: None,
        }
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("emojid/config.toml");
    path
}

fn load_config() -> Config {
    fs::read_to_string(config_path())
    .ok()
    .and_then(|s| toml::from_str(&s).ok())
    .unwrap_or_default()
}

fn save_config(cfg: &Config) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(toml) = toml::to_string_pretty(cfg) {
        let _ = fs::write(path, toml);
    }
}

/// ================= APP =================

struct Category {
    name: &'static str,
    emojis: Vec<String>,
}

struct EmojiApp {
    input: String,
    category: usize,
    selected: usize,
    categories: Vec<Category>,
    auto_paste: bool,
    remember_last_category: bool,
}

impl EmojiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Force sane visuals once (important on KDE Wayland)
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        
        let cfg = load_config();
        
        let mut categories = vec![
        Category {
            name: "Smileys",
            emojis: vec!["😀","😁","😂","🤣","😊","😍","😎","😭"]
            .into_iter().map(String::from).collect(),
        },
        Category {
            name: "Gestures",
            emojis: vec!["👍","👎","🙏","👏","🤝","✌️"]
            .into_iter().map(String::from).collect(),
        },
        Category {
            name: "Symbols",
            emojis: vec!["❤️","💔","✨","🔥","💀","⭐"]
            .into_iter().map(String::from).collect(),
        },
        Category {
            name: "Objects",
            emojis: vec!["🎉","🎮","🎧","📦","💡","🖥️"]
            .into_iter().map(String::from).collect(),
        },
        ];
        
        if let Some(k) = &cfg.kaomoji {
            categories.push(Category {
                name: "Kaomoji",
                emojis: k.items.clone(),
            });
        }
        
        let start_category = if cfg.remember_last_category.unwrap_or(true) {
            cfg.last_category.unwrap_or(0)
        } else {
            0
        }
        .min(categories.len().saturating_sub(1));
        
        Self {
            input: String::new(),
            category: start_category,
            selected: 0,
            categories,
            auto_paste: cfg.auto_paste.unwrap_or(true),
            remember_last_category: cfg.remember_last_category.unwrap_or(true),
        }
    }
    
    fn persist_category(&self) {
        if !self.remember_last_category {
            return;
        }
        
        let mut cfg = load_config();
        cfg.last_category = Some(self.category);
        save_config(&cfg);
    }
}

impl App for EmojiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ESC closes
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        
        // Keyboard navigation
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowDown) {
                self.selected += 1;
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                self.selected = self.selected.saturating_sub(1);
            }
            if i.key_pressed(egui::Key::Tab) {
                if i.modifiers.shift {
                    self.category = self.category.saturating_sub(1);
                } else {
                    self.category = (self.category + 1) % self.categories.len();
                }
                self.selected = 0;
                self.persist_category();
            }
        });
        
        let category_index = self.category;
        
        egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
            .fill(ctx.style().visuals.window_fill())
            .inner_margin(egui::Margin::same(12.0)),
        )
        .show(ctx, |ui| {
            // Categories (scrollable)
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, c) in self.categories.iter().enumerate() {
                        if ui.selectable_label(i == self.category, c.name).clicked() {
                            self.category = i;
                            self.selected = 0;
                            self.persist_category();
                        }
                    }
                });
            });
            
            ui.add_space(8.0);
            ui.text_edit_singleline(&mut self.input);
            ui.separator();
            
            let cat = &self.categories[category_index];
            
            for (idx, e) in cat.emojis.iter().enumerate() {
                if !e.contains(&self.input) {
                    continue;
                }
                
                let is_selected = idx == self.selected;
                let resp = ui.selectable_label(is_selected, e);
                
                if resp.clicked()
                || (is_selected && ctx.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    let mut cb = arboard::Clipboard::new().unwrap();
                    cb.set_text(e).unwrap();
                    
                    if self.auto_paste {
                        let _ = Command::new("sh")
                        .arg("-c")
                        .arg("sleep 0.15 && ydotool key 29:1 47:1 47:0 29:0")
                        .spawn();
                    }
                    
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
            }
        });
        
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([360.0, 460.0])
        .with_decorations(false)
        .with_always_on_top(),
        ..Default::default()
    };
    
    eframe::run_native(
        "emojid",
        options,
        Box::new(|cc| Box::new(EmojiApp::new(cc))),
    )
}
