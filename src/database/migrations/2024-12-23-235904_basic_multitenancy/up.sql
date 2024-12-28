-- Your SQL goes here
CREATE TABLE `server_variables`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`server_id` BIGINT NOT NULL,
	FOREIGN KEY(`server_id`) REFERENCES `servers`(`server_id`)
);

CREATE TABLE `servers`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`server_id` BIGINT NOT NULL
);

