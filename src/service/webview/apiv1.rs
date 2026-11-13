use axum::Json;

use crate::models::print_jobs::PrintJob;

pub async fn print_records() -> impl axum::response::IntoResponse {
    Json(vec![
        PrintJob {
            id: Some(1),
            job_name: "document1.pdf".to_string(),
            printer_name: "Printer A".to_string(),
            file_path: "/path/to/document1.pdf".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(101),
            error_reason: None,
            created_at: "2025-06-01 10:00:00".to_string(),
            updated_at: "2025-06-01 10:05:00".to_string(),
            finished_at: Some("2025-06-01 10:05:00".to_string()),
        },
        PrintJob {
            id: Some(2),
            job_name: "image1.png".to_string(),
            printer_name: "Printer B".to_string(),
            file_path: "/path/to/image1.png".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(102),
            error_reason: None,
            created_at: "2025-06-02 14:30:00".to_string(),
            updated_at: "2025-06-02 14:35:00".to_string(),
            finished_at: Some("2025-06-02 14:35:00".to_string()),
        },
        PrintJob {
            id: Some(3),
            job_name: "report.docx".to_string(),
            printer_name: "Printer C".to_string(),
            file_path: "/path/to/report.docx".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(103),
            error_reason: None,
            created_at: "2025-06-03 09:15:00".to_string(),
            updated_at: "2025-06-03 09:20:00".to_string(),
            finished_at: Some("2025-06-03 09:20:00".to_string()),
        },
        PrintJob {
            id: Some(4),
            job_name: "presentation.pptx".to_string(),
            printer_name: "Printer D".to_string(),
            file_path: "/path/to/presentation.pptx".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(104),
            error_reason: None,
            created_at: "2025-06-04 11:45:00".to_string(),
            updated_at: "2025-06-04 11:50:00".to_string(),
            finished_at: Some("2025-06-04 11:50:00".to_string()),
        },
    ])
}
