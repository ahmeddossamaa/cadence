use crate::adapters::persistence::Db;
use std::sync::Arc;

#[tauri::command]
pub fn export_csv(
    db: tauri::State<'_, Arc<Db>>,
    path: Option<String>,
) -> Result<String, String> {
    let today_start = today_midnight();
    let blocks = db
        .get_blocks_since(today_start)
        .map_err(|e| e.to_string())?;

    let mut csv = String::from("id,state,started_at,ended_at,keys,clicks,moves,scroll,cpu,ema,app_switches,dominant_app,source\n");

    for b in &blocks {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{:.2},{:.4},{},{},{}\n",
            b.id,
            b.state,
            b.started_at,
            b.ended_at.map(|t| t.to_string()).unwrap_or_default(),
            b.keys,
            b.clicks,
            b.moves,
            b.scroll,
            b.cpu,
            b.ema,
            b.app_switches,
            b.dominant_app.as_deref().unwrap_or(""),
            b.source,
        ));
    }

    if let Some(path) = path {
        std::fs::write(&path, &csv).map_err(|e| e.to_string())?;
        Ok(path)
    } else {
        Ok(csv)
    }
}

fn today_midnight() -> i64 {
    let now = chrono::Utc::now();
    now.date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp()
}
