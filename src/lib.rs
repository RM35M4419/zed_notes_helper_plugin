use chrono::Local;
use zed_extension_api as zed;
use serde_json::json;

struct NotesHelper;

impl zed::Extension for NotesHelper {
    fn new() -> Self {
        Self
    }
}

// Use the macro to register the extension
zed::register_extension!(NotesHelper);

// This will be injected into Zed through a different mechanism
#[no_mangle]
pub extern "C" fn activate_extension(extension_api: &mut dyn std::any::Any) {
    // Cast the extension_api to the appropriate type
    if let Ok(cx) = extension_api.downcast_mut::<zed::Context>() {
        // Register the commands
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
