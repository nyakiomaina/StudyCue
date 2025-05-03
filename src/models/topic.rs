use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Url(String),
    FilePath(String),
    Note(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingGoal {
    pub time_minutes: u32,
    pub pages: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub reading_goal: ReadingGoal,
    pub resources: Vec<ResourceType>,
    pub priority: Priority,
    pub estimated_time: u32, // in minutes
    pub scheduled_date: Option<DateTime<Local>>,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Topic {
    pub fn new(
        title: String,
        description: Option<String>,
        reading_goal: ReadingGoal,
        resources: Vec<ResourceType>,
        priority: Priority,
        estimated_time: u32,
    ) -> Self {
        let now = Local::now();
        Self {
            id: 0, // Will be set by the database
            title,
            description,
            reading_goal,
            resources,
            priority,
            estimated_time,
            scheduled_date: None,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }
}