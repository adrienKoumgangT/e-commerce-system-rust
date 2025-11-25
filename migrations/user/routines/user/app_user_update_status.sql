create procedure app_user_update_status(IN __user_id bigint, IN __user_status bigint, IN __meta_user bigint)
begin

    update user
    set status = __user_status,
        updated_at = now()
    where id = __user_id;


    call app_user_get(__user_id, __meta_user);

end;

