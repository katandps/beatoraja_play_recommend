create table upload_log_stats
(
    id            int primary key auto_increment,
    upload_log_id int not null,
    user_id       int not null,
    playcount     int not null,
    clear         int not null,
    epg           int not null,
    lpg           int not null,
    egr           int not null,
    lgr           int not null,
    egd           int not null,
    lgd           int not null,
    ebd           int not null,
    lbd           int not null,
    epr           int not null,
    lpr           int not null,
    ems           int not null,
    lms           int not null,
    playtime      int not null,
    unique key upload_log_stats_unique_key (upload_log_id),
    foreign key upload_log_stats (user_id) references users (id),
    foreign key upload_log_stats (upload_log_id) references score_upload_logs (id)
)
