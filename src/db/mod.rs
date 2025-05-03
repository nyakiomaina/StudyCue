use rusqlite::{Connection, Result, params};
use crate::models::topic::{Topic, Priority, ResourceType, ReadingGoal};
use chrono::{DateTime, Local};
use serde_json;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("studycue.db")?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS topics (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                reading_goal_json TEXT NOT NULL,
                resources_json TEXT NOT NULL,
                priority TEXT NOT NULL,
                estimated_time INTEGER NOT NULL,
                scheduled_date TEXT,
                completed INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_topic(&self, topic: &Topic) -> Result<i64> {
        let reading_goal_json = serde_json::to_string(&topic.reading_goal).unwrap();
        let resources_json = serde_json::to_string(&topic.resources).unwrap();

        self.conn.execute(
            "INSERT INTO topics (
                title, description, reading_goal_json, resources_json,
                priority, estimated_time, scheduled_date, completed,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                topic.title,
                topic.description,
                reading_goal_json,
                resources_json,
                format!("{:?}", topic.priority),
                topic.estimated_time,
                topic.scheduled_date.map(|dt| dt.to_rfc3339()),
                topic.completed,
                topic.created_at.to_rfc3339(),
                topic.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_topic(&self, id: i64) -> Result<Topic> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM topics WHERE id = ?1"
        )?;

        let topic = stmt.query_row([id], |row| {
            let reading_goal_json: String = row.get(3)?;
            let resources_json: String = row.get(4)?;

            Ok(Topic {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                reading_goal: serde_json::from_str(&reading_goal_json).unwrap(),
                resources: serde_json::from_str(&resources_json).unwrap(),
                priority: match row.get::<_, String>(5)?.as_str() {
                    "High" => Priority::High,
                    "Medium" => Priority::Medium,
                    "Low" => Priority::Low,
                    _ => Priority::Medium,
                },
                estimated_time: row.get(6)?,
                scheduled_date: row.get::<_, Option<String>>(7)?
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Local)),
                completed: row.get(8)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Local),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .unwrap()
                    .with_timezone(&Local),
            })
        })?;

        Ok(topic)
    }

    pub fn get_topics_for_date(&self, date: DateTime<Local>) -> Result<Vec<Topic>> {
        let mut stmt = self.conn.prepare(
            "SELECT id FROM topics WHERE scheduled_date = ?1"
        )?;

        let topic_ids: Vec<i64> = stmt.query_map([date.to_rfc3339()], |row| row.get(0))?.collect::<Result<Vec<_>>>()?;

        let mut topics = Vec::new();
        for id in topic_ids {
            topics.push(self.get_topic(id)?);
        }

        Ok(topics)
    }

    pub fn update_topic(&self, topic: &Topic) -> rusqlite::Result<()> {
        let reading_goal_json = serde_json::to_string(&topic.reading_goal).unwrap();
        let resources_json = serde_json::to_string(&topic.resources).unwrap();

        self.conn.execute(
            "UPDATE topics SET
                title = ?1,
                description = ?2,
                reading_goal_json = ?3,
                resources_json = ?4,
                priority = ?5,
                estimated_time = ?6,
                scheduled_date = ?7,
                completed = ?8,
                created_at = ?9,
                updated_at = ?10
            WHERE id = ?11",
            rusqlite::params![
                topic.title,
                topic.description,
                reading_goal_json,
                resources_json,
                format!("{:?}", topic.priority),
                topic.estimated_time,
                topic.scheduled_date.map(|dt| dt.to_rfc3339()),
                topic.completed,
                topic.created_at.to_rfc3339(),
                topic.updated_at.to_rfc3339(),
                topic.id,
            ],
        )?;
        Ok(())
    }
}