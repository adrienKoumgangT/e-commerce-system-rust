create procedure app_user_status_get_by_id(IN __user_status_id bigint, IN __meta_user bigint)
begin

    select *
    from user_status
    where id = __user_status_id;

end;

