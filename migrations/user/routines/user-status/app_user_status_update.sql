create procedure app_user_status_update(IN __user_status_id bigint,
                                        IN __user_status_name varchar(200),
                                        IN __user_status_description longtext,
                                        IN __meta_user bigint)
begin

    update user_status
    set name = __user_status_name,
        description = __user_status_description
    where id = __user_status_id;

    call app_user_status_get(__user_status_id, __meta_user);

end;

