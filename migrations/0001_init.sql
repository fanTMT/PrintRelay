-- share_links
CREATE TABLE share_links (
    id                TEXT PRIMARY KEY, 
    file_path         TEXT,                 -- 允许 NULL：创建时还没上传
    original_filename TEXT,                 -- 允许 NULL：创建时还没上传
    used              INTEGER NOT NULL DEFAULT 0, -- 允许 NULL：创建时还没上传
    expires_at        TEXT NOT NULL,        -- RFC3339 UTC
    created_at        TEXT NOT NULL
);

-- print_jobs
CREATE TABLE print_jobs (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    job_name     TEXT NOT NULL, -- 允许 NULL：创建时还没上传
    printer_name TEXT NOT NULL, 
    file_path    TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'pending',
    cups_job_id  INTEGER,
    error_reason TEXT,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL,
    finished_at  TEXT
);
CREATE INDEX idx_print_jobs_status  ON print_jobs(status);
CREATE INDEX idx_print_jobs_printer ON print_jobs(printer_name);
CREATE INDEX idx_print_jobs_created ON print_jobs(created_at);
