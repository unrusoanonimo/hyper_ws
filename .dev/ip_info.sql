CREATE TABLE `IP_INFO` (
    `ip` VARCHAR(45) NOT NULL,
    `city` VARCHAR(64) NOT NULL,
    `region` VARCHAR(64) NOT NULL,
    `country` VARCHAR(64) NOT NULL,
    `loc` VARCHAR(64) NOT NULL,
    `org` VARCHAR(64),
    `postal` VARCHAR(64) NOT NULL,
    `timezone` VARCHAR(64) NOT NULL,
    PRIMARY KEY (`ip`)
);