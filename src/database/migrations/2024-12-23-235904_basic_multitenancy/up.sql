-- Your SQL goes here
CREATE TABLE `server_variables`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`server_id` BIGINT NOT NULL
);

CREATE TABLE `servers`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`server_id` BIGINT NOT NULL
);

