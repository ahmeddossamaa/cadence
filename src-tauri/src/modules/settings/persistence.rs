use crate::db::Db;
use crate::types::settings::Settings;
use rusqlite::params;
use std::collections::HashMap;

impl Db {
    pub fn get_all_settings(&self) -> Result<Settings, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut values = HashMap::new();
        for row in rows {
            let (k, v) = row?;
            values.insert(k, v);
        }
        Ok(Settings { values })
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query_map(params![key], |row| row.get::<_, String>(0))?;
        rows.next().transpose()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn update_settings(&self, settings: &Settings) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        for (key, value) in &settings.values {
            conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
                params![key, value],
            )?;
        }
        Ok(())
    }
}
