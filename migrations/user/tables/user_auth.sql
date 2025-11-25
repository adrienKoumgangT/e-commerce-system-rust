-- auto-generated definition
create table if not exists user_auth
(
    id          bigint auto_increment
        primary key,
    name        varchar(200) not null,
    description longtext     null
);

create index user_auth_name_index
    on user_auth (name);

