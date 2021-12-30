create table player_stats
(
    id int primary key auto_increment,
    user_id int not null,
    date datetime not null,
    playcount int not null,
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
    playtime int not null,
    unique key player_stat_unique_key (user_id, date),
    index date_index(date),
    foreign key player_stats(user_id) references users(id)
)
