create procedure app_user_get_all(IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then
        select *
        from user_view
        limit __offset, __limit;
    else
        select *
        from user_view;
    end if;

end;