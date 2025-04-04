use chrono::Local;
use zed_extension_api as zed;
use serde_json::json;

struct NotesHelper;

impl zed::Extension for NotesHelper {
    fn new() -> Self {
        Self
    }

    fn activate(&mut self, cx: &mut zed::Context) {
        cx.register_command("insert_current_date", |_params, cx| {
            let date = Local::now().format("%Y-%m-%d").to_string();
            cx.workspace.insert_text(&date)?;
            Ok(json!(null))
        });

        cx.register_command("insert_current_time", |_params, cx| {
            let time = Local::now().format("%H:%M:%S").to_string();
            cx.workspace.insert_text(&time)?;
            Ok(json!(null))
        });

        cx.register_command("insert_current_datetime", |_params, cx| {
            let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            cx.workspace.insert_text(&datetime)?;
            Ok(json!(null))
        });

        cx.register_command("insert_current_datetime_iso", |_params, cx| {
            let datetime = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string();
            cx.workspace.insert_text(&datetime)?;
            Ok(json!(null))
        });

        cx.register_command("insert_day_of_week", |_params, cx| {
            let day = Local::now().format("%A").to_string();
            cx.workspace.insert_text(&day)?;
            Ok(json!(null))
        });

        cx.register_command("insert_file_path", |_params, cx| {
            if let Some(active_editor) = cx.workspace.active_editor() {
                if let Some(buffer) = active_editor.active_buffer() {
                    if let Some(file) = buffer.file() {
                        let path = file.path().to_string();
                        cx.workspace.insert_text(&path)?;
                        return Ok(json!(null));
                    }
                }
            }
            cx.workspace.insert_text("[No file path available]")?;
            Ok(json!(null))
        });
    }
}

zed::register_extension!(NotesHelper);
