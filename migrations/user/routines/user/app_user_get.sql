create procedure app_user_get(IN __user_id bigint, IN __meta_user bigint)
begin

    select *
    from user_view
    where id = __user_id;

end;

