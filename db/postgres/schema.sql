DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS item;
DROP TABLE IF EXISTS shelf;
DROP TABLE IF EXISTS location;
DROP TABLE IF EXISTS product;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS db_users;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
	user_id serial NOT NULL PRIMARY KEY,
	username varchar NOT NULL,
	email varchar NOT NULL,
	name varchar NOT NULL,
	surname varchar NOT NULL,
	auth_method varchar NOT NULL,
	api_key varchar,
	last_login timestamp with time zone,
	last_logout timestamp with time zone
);

CREATE TABLE db_users (
	user_id integer REFERENCES users(user_id) NOT NULL PRIMARY KEY,
	password varchar NOT NULL
);

CREATE TABLE sessions (
	user_id integer REFERENCES users(user_id) NOT NULL PRIMARY KEY,
	start timestamp with time zone NOT NULL DEFAULT(now()),
	address inet NOT NULL,
	session_id UUID NOT NULL
);

CREATE TABLE product (
	product_id serial NOT NULL PRIMARY KEY,
	name varchar NOT NULL,
	gtin bigint
);

CREATE TABLE location (
	location_id serial NOT NULL PRIMARY KEY,
	name varchar NOT NULL UNIQUE
);

CREATE TABLE shelf (
	location_id integer REFERENCES location(location_id) NOT NULL,
	shelf_id serial NOT NULL PRIMARY KEY,
	name varchar NOT NULL
);

CREATE UNIQUE INDEX shelf_loc_id_idx ON shelf (location_id, shelf_id);

CREATE TABLE item (
	item_id serial NOT NULL PRIMARY KEY,
	product_id integer REFERENCES product(product_id) NOT NULL,
	location_id integer REFERENCES location(location_id) NOT NULL,
	shelf_id integer REFERENCES shelf(shelf_id) NOT NULL,
	first_added timestamp with time zone NOT NULL DEFAULT(now()),
	last_moved timestamp with time zone NOT NULL DEFAULT(now()),
	FOREIGN KEY (location_id, shelf_id) REFERENCES shelf (location_id, shelf_id)
);

