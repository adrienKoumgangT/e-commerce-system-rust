create procedure app_user_status_insert(IN __user_status_name varchar(200),
                                        IN __user_status_description longtext,
                                        IN __meta_user bigint)
begin

    insert into user_status
    (
        name,
        description
    )
    VALUES
        (
            __user_status_name,
            __user_status_description
        )
    ;

    call app_user_status_get(last_insert_id(), __meta_user);

end;

