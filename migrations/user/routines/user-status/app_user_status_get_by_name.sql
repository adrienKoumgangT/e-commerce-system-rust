create procedure app_user_status_get_by_name(IN __user_status_name varchar(200), IN __meta_user bigint)
begin

    select *
    from user_status
    where name = __user_status_name;

end;

