create procedure app_user_update_profile_pic_url(IN __user_id bigint,
                                                 IN __user_profile_pic_url longtext,
                                                 IN __meta_user bigint)
begin

    update user
    set profile_pic_url = __user_profile_pic_url,
        updated_at = now()
    where id = __user_id;


    call app_user_get(__user_id, __meta_user);

end;

