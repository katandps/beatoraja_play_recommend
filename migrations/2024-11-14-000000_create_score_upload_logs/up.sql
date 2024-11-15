create table score_upload_logs
(
    id int primary key auto_increment,
    user_id int not null,
    date datetime not null,
    index date_index(date),
    foreign key users_score_upload_logs (user_id) references users(id)
)