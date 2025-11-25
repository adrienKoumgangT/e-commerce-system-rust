-- auto-generated definition
create table if not exists user_status
(
    id          bigint auto_increment
        primary key,
    name        varchar(200) not null,
    description longtext     null
);

create index user_status_name_index
    on user_status (name);

