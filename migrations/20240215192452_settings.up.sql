create table if not exists settings
(
  id  text default 'DEFAULT_SETTINGS' not null primary key,
  encrypted_global_api_key  text
);

insert into settings (encrypted_global_api_key)
values ('0c37568a3290f44ecb7f89bde13753066f735bd62cec7a2e9827b0a881acd526');
