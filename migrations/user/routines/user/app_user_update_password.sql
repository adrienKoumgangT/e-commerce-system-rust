create procedure app_user_update_password(IN __user_id bigint,
                                          IN __user_password varchar(500),
                                          IN __meta_user bigint)
begin

    update user
    set password = __user_password,
        updated_at = now()
    where id = __user_id;


    call app_user_get(__user_id, __meta_user);

end;

