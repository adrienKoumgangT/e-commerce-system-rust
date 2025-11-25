create procedure app_user_status_get_all(IN __meta_user bigint)
begin

    select *
    from user_status;

end;

