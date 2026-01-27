-- DEV ONLY -- Brute force drop and recreate db for local unit testing and local development
select pg_terminate_backend(pid) from pg_stat_activity WHERE pid <> pg_backend_pid()
  AND (usename = 'app_user' OR datname = 'app_db');


DROP database if exists app_db;
DROP user if exists app_user;


-- DEV ONLY -- Dev only password for local unit testing and local development
CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db OWNER app_user ENCODING 'UTF8';