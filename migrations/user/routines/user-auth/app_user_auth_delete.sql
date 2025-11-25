create procedure app_user_auth_delete(IN __user_auth_id bigint, IN __meta_user bigint)
begin

delete from user_auth
where id = __user_auth_id;

end;

