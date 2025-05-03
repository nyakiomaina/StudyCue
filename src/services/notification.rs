use notify_rust::Notification;
use crate::models::topic::Topic;
use chrono::Local;
use log::error;

pub struct NotificationService;

impl NotificationService {
    pub fn new() -> Self {
        Self
    }

    pub fn send_daily_reminder(&self, topics: &[Topic]) -> Result<(), Box<dyn std::error::Error>> {
        if topics.is_empty() {
            return Ok(());
        }

        let mut summary = format!("Today's Reading Topics ({})", Local::now().format("%Y-%m-%d"));
        let mut body = String::new();

        for (i, topic) in topics.iter().enumerate() {
            body.push_str(&format!(
                "{}. {} ({} min)\n",
                i + 1,
                topic.title,
                topic.estimated_time
            ));
        }

        match Notification::new()
            .summary(&summary)
            .body(&body)
            .icon("book")
            .show()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send notification: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn send_topic_reminder(&self, topic: &Topic) -> Result<(), Box<dyn std::error::Error>> {
        let summary = format!("Reading Reminder: {}", topic.title);
        let body = format!(
            "Estimated time: {} minutes\nPriority: {:?}",
            topic.estimated_time,
            topic.priority
        );

        match Notification::new()
            .summary(&summary)
            .body(&body)
            .icon("book")
            .show()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send notification: {}", e);
                Err(Box::new(e))
            }
        }
    }
}