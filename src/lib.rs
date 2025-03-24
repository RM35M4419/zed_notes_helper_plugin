use chrono::Local;
use zed_extension_api as zed;

struct NotesHelper;

impl zed::Extension for NotesHelper {
    fn new() -> Self {
        Self
    }

    fn commands(&self) -> Vec<zed::Command> {
        vec![
            zed::Command::new("insert_current_date", "Insert current date")
                .with_handler(|_ctx, _params| {
                    let date = Local::now().format("%Y-%m-%d").to_string();
                    zed::workspace::insert_text(&date)?;
                    Ok(serde_json::Value::Null)
                }),
            zed::Command::new("insert_current_time", "Insert current time")
                .with_handler(|_ctx, _params| {
                    let time = Local::now().format("%H:%M:%S").to_string();
                    zed::workspace::insert_text(&time)?;
                    Ok(serde_json::Value::Null)
                }),
        ]
    }
}

zed::register_extension!(NotesHelper);
