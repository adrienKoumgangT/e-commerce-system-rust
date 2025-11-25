create procedure app_user_delete(IN __user_id bigint, IN __meta_user bigint)
begin

    delete from user
    where id = __user_id;

end;

