// ignore_for_file: non_constant_identifier_names

class HistoryItem {
  final int id;
  final String job_name;
  final String printer_name;
  final String file_path;
  final String status;
  final int cups_job_id;
  final String error_reason;
  final String created_at;
  final String updated_at;
  final String finished_at;

  HistoryItem({
    required this.id,
    required this.job_name,
    required this.printer_name,
    required this.file_path,
    required this.status,
    required this.cups_job_id,
    required this.error_reason,
    required this.created_at,
    required this.updated_at,
    required this.finished_at,
  });
}

// -- print_jobs
// CREATE TABLE print_jobs(
//   id INTEGER PRIMARY KEY AUTOINCREMENT,
//   job_name TEXT NOT NULL, -- 允许 NULL：创建时还没上传
//   printer_name TEXT NOT NULL,
//   file_path TEXT NOT NULL,
//   status TEXT NOT NULL DEFAULT 'pending',
//   cups_job_id INTEGER,
//   error_reason TEXT,
//   created_at TEXT NOT NULL,
//   updated_at TEXT NOT NULL,
//   finished_at TEXT
// );

// -- share_links
// CREATE TABLE share_links(
//   id TEXT PRIMARY KEY,
//   file_path TEXT, -- 允许 NULL：创建时还没上传
//   original_filename TEXT, -- 允许 NULL：创建时还没上传
//   used INTEGER NOT NULL DEFAULT 0, -- 允许 NULL：创建时还没上传
//   expires_at TEXT NOT NULL, -- RFC3339 UTC
//   created_at TEXT NOT NULL
// );

class ShareLink {
  final String id;
  final String file_path;
  final String original_filename;
  final int used;
  final String expires_at;
  final String created_at;

  ShareLink({
    required this.id,
    required this.file_path,
    required this.original_filename,
    required this.used,
    required this.expires_at,
    required this.created_at,
  });
}
