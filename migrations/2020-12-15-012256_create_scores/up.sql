create table scores
(
    id int primary key auto_increment,
    user_id int not null,
    sha256 char(64) not null,
    mode int not null,
    clear int not null,
    epg int not null,
    lpg int not null,
    egr int not null,
    lgr int not null,
    egd int not null,
    lgd int not null,
    ebd int not null,
    lbd int not null,
    epr int not null,
    lpr int not null,
    ems int not null,
    lms int not null,
    combo int not null,
    min_bp int not null,
    play_count int not null,
    clear_count int not null,
    date datetime not null,
    unique key score_unique_key (user_id, sha256, mode),
    foreign key users_scores (user_id) references users(id),
    foreign key scores_hash(sha256) references hashes(sha256)
)