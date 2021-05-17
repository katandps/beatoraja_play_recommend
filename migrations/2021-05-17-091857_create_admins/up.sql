create table admins
(
    id int primary key auto_increment,
    user_id int not null,
    foreign key admins(user_id) references users(id)
)