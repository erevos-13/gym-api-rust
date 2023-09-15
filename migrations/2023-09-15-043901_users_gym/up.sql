-- Your SQL goes here
create table users_gym (
    user_id varchar(255) references users(id) not null,
    gym_id varchar(255) references gym(id) not null,
    created_at  timestamp not null,
    updated_at  timestamp not null,
    primary key (user_id, gym_id)
);
