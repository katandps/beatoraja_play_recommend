alter table score_snaps add index idx_score_snaps_score_upload_log_id(score_upload_log_id);
alter table scores add index idx_score_snaps_score_upload_log_id(score_upload_log_id);
