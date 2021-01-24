create table user_statuses
(
    id int primary key auto_increment,
    user_id int not null,
    visible bool not null default true,
    score_updated_at datetime not null,
    unique key user_id_unique_key (user_id),
    foreign key user_statuses(user_id) references users(id)
)