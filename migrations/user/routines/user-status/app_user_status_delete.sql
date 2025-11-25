create procedure app_user_status_delete(IN __user_status_id bigint, IN __meta_user bigint)
begin

    delete from user_status
    where id = __user_status_id;

end;

