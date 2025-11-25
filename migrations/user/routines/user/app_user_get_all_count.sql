create procedure app_user_get_all_count(IN __meta_user bigint)
begin

    select count(*) as count
    from user;

end;

