mod db;
mod models;
mod services;

use db::Database;
use models::topic::{Topic, Priority, ReadingGoal};
use services::notification::NotificationService;
use chrono::{Local, Datelike};
use eframe::egui::{self, Color32};
use std::sync::Mutex;

struct StudyCue {
    db: Mutex<Database>,
    notification_service: NotificationService,
    topics: Vec<Topic>,
    new_topic: TopicForm,
    selected_date: chrono::NaiveDate,
    editing_topic_id: Option<i64>,
}

struct TopicForm {
    title: String,
    description: String,
    time_minutes: u32,
    pages: Option<u32>,
    priority: Priority,
    estimated_time: u32,
    scheduled_date: chrono::NaiveDate,
}

impl Default for TopicForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            time_minutes: 30,
            pages: None,
            priority: Priority::Medium,
            estimated_time: 30,
            scheduled_date: chrono::Local::now().date_naive(),
        }
    }
}

impl Default for StudyCue {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            db: Mutex::new(Database::new().expect("Failed to initialize database")),
            notification_service: NotificationService::new(),
            topics: Vec::new(),
            new_topic: TopicForm::default(),
            selected_date: today,
            editing_topic_id: None,
        }
    }
}

impl eframe::App for StudyCue {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().frame(
            egui::Frame::none().fill(Color32::from_rgb(255, 240, 250))
        ).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🌸 StudyCue 🌸").color(Color32::from_rgb(233, 30, 99)));
            });
            ui.label(egui::RichText::new("Your Weekly Reading Companion").color(Color32::from_rgb(233, 30, 99)));
            ui.separator();

            self.show_week_selector(ui);
            ui.separator();

            ui.heading(egui::RichText::new("Add a Topic").color(Color32::from_rgb(233, 30, 99)));
            self.show_topic_form(ui);
            ui.separator();

            ui.heading(egui::RichText::new(format!("Topics for {}", self.selected_date)).color(Color32::from_rgb(233, 30, 99)));
            self.show_topics(ui);
        });
        ctx.request_repaint();
    }
}

impl StudyCue {
    fn show_week_selector(&mut self, ui: &mut egui::Ui) {
        let today = chrono::Local::now().date_naive();
        let weekday = today.weekday().num_days_from_monday();
        let start_of_week = today - chrono::Duration::days(weekday as i64);
        ui.horizontal(|ui| {
            for i in 0..7 {
                let day = start_of_week + chrono::Duration::days(i);
                let label = format!("{}", day.format("%a %d"));
                if ui.selectable_label(self.selected_date == day, label).clicked() {
                    self.selected_date = day;
                }
            }
        });
    }

    fn show_topic_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.new_topic.title);
            });
            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_multiline(&mut self.new_topic.description);
            });
            ui.horizontal(|ui| {
                ui.label("Reading Time (minutes):");
                ui.add(egui::DragValue::new(&mut self.new_topic.time_minutes));
            });
            ui.horizontal(|ui| {
                ui.label("Pages (optional):");
                if let Some(pages) = &mut self.new_topic.pages {
                    ui.add(egui::DragValue::new(pages));
                } else {
                    if ui.button("Add Pages").clicked() {
                        self.new_topic.pages = Some(0);
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Priority:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.new_topic.priority))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_topic.priority, Priority::High, "High");
                        ui.selectable_value(&mut self.new_topic.priority, Priority::Medium, "Medium");
                        ui.selectable_value(&mut self.new_topic.priority, Priority::Low, "Low");
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Estimated Time (minutes):");
                ui.add(egui::DragValue::new(&mut self.new_topic.estimated_time));
            });
            ui.horizontal(|ui| {
                ui.label("Scheduled Date:");
                let mut ymd = (
                    self.new_topic.scheduled_date.year(),
                    self.new_topic.scheduled_date.month(),
                    self.new_topic.scheduled_date.day(),
                );
                ui.add(egui::DragValue::new(&mut ymd.0).prefix("Year: "));
                ui.add(egui::DragValue::new(&mut ymd.1).prefix("Month: "));
                ui.add(egui::DragValue::new(&mut ymd.2).prefix("Day: "));
                if chrono::NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2).is_some() {
                    self.new_topic.scheduled_date = chrono::NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2).unwrap();
                }
            });
            if let Some(edit_id) = self.editing_topic_id {
                if ui.button("💾 Save Changes").clicked() {
                    self.save_edited_topic(edit_id);
                }
                if ui.button("❌ Cancel").clicked() {
                    self.editing_topic_id = None;
                    self.new_topic = TopicForm::default();
                }
            } else {
                if ui.button("Add Topic").clicked() {
                    self.add_topic();
                }
            }
        });
    }

    fn show_topics(&mut self, ui: &mut egui::Ui) {
        if let Ok(db) = self.db.lock() {
            if let Ok(topics) = db.get_topics_for_date(self.selected_date.and_hms_opt(0,0,0).unwrap().and_local_timezone(chrono::Local).unwrap()) {
                self.topics = topics;
            }
        }
        if self.topics.is_empty() {
            ui.label(egui::RichText::new("No topics scheduled for today.").color(Color32::from_rgb(233, 30, 99)));
            return;
        }
        for topic in &self.topics {
            let (bg, flower) = match topic.priority {
                Priority::High => (Color32::from_rgb(255, 182, 193), "🌺"), // peony pink
                Priority::Medium => (Color32::from_rgb(255, 210, 220), "🌸"), // lighter peony
                Priority::Low => (Color32::from_rgb(255, 240, 250), "🌷"), // softest peony
            };
            egui::Frame::group(ui.style())
                .fill(bg)
                .stroke(egui::Stroke::new(2.0, Color32::from_rgb(233, 30, 99)))
                .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("{} {}", flower, topic.title));
                });
                if let Some(desc) = &topic.description {
                    ui.label(desc);
                }
                ui.horizontal(|ui| {
                    ui.label(format!("Time: {} min", topic.estimated_time));
                    ui.label(format!("Priority: {:?}", topic.priority));
                });
                ui.horizontal(|ui| {
                    if ui.button("✏️ Edit").clicked() {
                        self.editing_topic_id = Some(topic.id);
                        self.new_topic = TopicForm {
                            title: topic.title.clone(),
                            description: topic.description.clone().unwrap_or_default(),
                            time_minutes: topic.reading_goal.time_minutes,
                            pages: topic.reading_goal.pages,
                            priority: topic.priority,
                            estimated_time: topic.estimated_time,
                            scheduled_date: topic.scheduled_date.unwrap().date_naive(),
                        };
                    }
                    if ui.button("🔔 Notify").clicked() {
                        let _ = self.notification_service.send_topic_reminder(topic);
                    }
                });
            });
        }
    }

    fn add_topic(&mut self) {
        let reading_goal = ReadingGoal {
            time_minutes: self.new_topic.time_minutes,
            pages: self.new_topic.pages,
        };
        let scheduled_date = self.new_topic.scheduled_date.and_hms_opt(0,0,0).unwrap().and_local_timezone(chrono::Local).unwrap();
        let topic = Topic::new(
            self.new_topic.title.clone(),
            Some(self.new_topic.description.clone()),
            reading_goal,
            Vec::new(),
            self.new_topic.priority,
            self.new_topic.estimated_time,
        ).with_scheduled_date(scheduled_date);
        if let Ok(db) = self.db.lock() {
            if let Ok(_) = db.add_topic(&topic) {
                self.new_topic = TopicForm::default();
            }
        }
    }

    fn save_edited_topic(&mut self, topic_id: i64) {
        let reading_goal = ReadingGoal {
            time_minutes: self.new_topic.time_minutes,
            pages: self.new_topic.pages,
        };
        let scheduled_date = self.new_topic.scheduled_date.and_hms_opt(0,0,0).unwrap().and_local_timezone(chrono::Local).unwrap();
        let mut topic = Topic::new(
            self.new_topic.title.clone(),
            Some(self.new_topic.description.clone()),
            reading_goal,
            Vec::new(),
            self.new_topic.priority,
            self.new_topic.estimated_time,
        ).with_scheduled_date(scheduled_date);
        topic.id = topic_id;
        if let Ok(db) = self.db.lock() {
            let _ = db.update_topic(&topic);
        }
        self.editing_topic_id = None;
        self.new_topic = TopicForm::default();
    }
}

// Add a helper to Topic to set scheduled_date
impl Topic {
    pub fn with_scheduled_date(mut self, date: chrono::DateTime<chrono::Local>) -> Self {
        self.scheduled_date = Some(date);
        self
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "StudyCue",
        options,
        Box::new(|_cc| Box::new(StudyCue::default())),
    );
}