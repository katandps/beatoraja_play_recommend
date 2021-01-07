create table score_snaps
(
    id int primary key auto_increment,
    user_id int not null,
    sha256 char(64) not null,
    mode int not null,
    date datetime not null,
    clear int not null,
    score int not null,
    combo int not null,
    min_bp int not null,
    unique key score_snap_unique_key (user_id, sha256, mode, date),
    index date_index(date),
    foreign key users_score_snaps (user_id) references users(id),
    foreign key score_snaps_hash (sha256) references hashes(sha256)
)