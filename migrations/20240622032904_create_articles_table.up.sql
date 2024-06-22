-- Add up migration script here
CREATE TABLE IF NOT EXISTS `articles` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(100) NOT NULL,
    `description` VARCHAR(255) DEFAULT NULL,
    `body` TEXT DEFAULT NULL,
    `created_at` TIMESTAMP DEFAULT NULL,
    `updated_at` TIMESTAMP DEFAULT NULL,
    `deleted_at` TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (`id`)
);