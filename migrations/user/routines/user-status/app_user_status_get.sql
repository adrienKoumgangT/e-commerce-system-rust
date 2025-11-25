create procedure app_user_status_get(IN __user_status_id bigint, IN __meta_user bigint)
begin

    select *
    from user_status
    where id = __user_status_id;

end;

