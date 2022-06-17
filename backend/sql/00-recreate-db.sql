-- DEV ONLY -- Comment out for keeping DB between restart
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY -- For a quick iteration
CREATE USER app_user PASSWORD 'app_pwd_to_change';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';