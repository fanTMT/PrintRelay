// ignore_for_file: non_constant_identifier_names

class HistoryItem {
  final int id;
  final String jobName;
  final String printerName;
  final String filePath;
  final String status;
  final int? cupsJobId;
  final String? errorReason;
  final String createdAt;
  final String updatedAt;
  final String? finishedAt;

  HistoryItem({
    required this.id,
    required this.jobName,
    required this.printerName,
    required this.filePath,
    required this.status,
    required this.cupsJobId,
    required this.errorReason,
    required this.createdAt,
    required this.updatedAt,
    required this.finishedAt,
  });

  factory HistoryItem.fromJson(Map<String, dynamic> json) {
    return HistoryItem(
      id: (json['id'] as num?)?.toInt() ?? 0,
      jobName: json['job_name'] as String? ?? '未命名文件',
      printerName: json['printer_name'] as String? ?? '未知打印机',
      filePath: json['file_path'] as String? ?? '',
      status: json['status'] as String? ?? 'unknown',
      cupsJobId: (json['cups_job_id'] as num?)?.toInt(),
      errorReason: json['error_reason'] as String?,
      createdAt: json['created_at'] as String? ?? '',
      updatedAt: json['updated_at'] as String? ?? '',
      finishedAt: json['finished_at'] as String?,
    );
  }
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
