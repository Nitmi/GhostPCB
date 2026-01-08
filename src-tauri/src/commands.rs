use crate::gerber::{GerberProcessor, ProcessRequest, ProcessResult};

#[tauri::command]
pub async fn process_gerber(request: ProcessRequest) -> Result<ProcessResult, String> {
    GerberProcessor::process(&request).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_default_options() -> crate::gerber::ObfuscateOptions {
    crate::gerber::ObfuscateOptions::default()
}
