-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema whisperdb
-- -----------------------------------------------------
DROP SCHEMA IF EXISTS `whisperdb` ;

-- -----------------------------------------------------
-- Schema whisperdb
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `whisperdb` ;
USE `whisperdb` ;

-- -----------------------------------------------------
-- Table `whisperdb`.`users`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`users` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`users` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` VARCHAR(45) NOT NULL,
  `password` VARCHAR(160) NOT NULL,
  `photo` VARCHAR(255) NULL,
  `email` VARCHAR(100) NOT NULL,
  `full_name` VARCHAR(255) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `whisperdb`.`posts`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`posts` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`posts` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `content` VARCHAR(280) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  `user_id` INT UNSIGNED NULL,
  `reply_to` BIGINT UNSIGNED NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_post_user`
    FOREIGN KEY (`user_id`)
    REFERENCES `whisperdb`.`users` (`id`)
    ON DELETE SET NULL
    ON UPDATE CASCADE)
ENGINE = InnoDB;

CREATE INDEX `fk_post_user_idx` ON `whisperdb`.`posts` (`user_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `whisperdb`.`reaction_type`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`reaction_type` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`reaction_type` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `whisperdb`.`reactions`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`reactions` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`reactions` (
  `reaction_type_id` INT UNSIGNED NOT NULL,
  `post_id` BIGINT UNSIGNED NOT NULL,
  `user_id` INT UNSIGNED NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_reaction_reaction_type1`
    FOREIGN KEY (`reaction_type_id`)
    REFERENCES `whisperdb`.`reaction_type` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_reaction_post1`
    FOREIGN KEY (`post_id`)
    REFERENCES `whisperdb`.`posts` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_reaction_user1`
    FOREIGN KEY (`user_id`)
    REFERENCES `whisperdb`.`users` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_reaction_reaction_type1_idx` ON `whisperdb`.`reactions` (`reaction_type_id` ASC) VISIBLE;

CREATE INDEX `fk_reaction_post1_idx` ON `whisperdb`.`reactions` (`post_id` ASC) VISIBLE;

CREATE INDEX `fk_reaction_user1_idx` ON `whisperdb`.`reactions` (`user_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `whisperdb`.`tags`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`tags` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`tags` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `whisperdb`.`posts_has_tags`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`posts_has_tags` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`posts_has_tags` (
  `posts_id` BIGINT UNSIGNED NOT NULL,
  `tags_id` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`posts_id`, `tags_id`),
  CONSTRAINT `fk_posts_has_tags_posts1`
    FOREIGN KEY (`posts_id`)
    REFERENCES `whisperdb`.`posts` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_posts_has_tags_tags1`
    FOREIGN KEY (`tags_id`)
    REFERENCES `whisperdb`.`tags` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_posts_has_tags_tags1_idx` ON `whisperdb`.`posts_has_tags` (`tags_id` ASC) VISIBLE;

CREATE INDEX `fk_posts_has_tags_posts1_idx` ON `whisperdb`.`posts_has_tags` (`posts_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `whisperdb`.`reposts`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `whisperdb`.`reposts` ;

CREATE TABLE IF NOT EXISTS `whisperdb`.`reposts` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `users_id` INT UNSIGNED NOT NULL,
  `posts_id` BIGINT UNSIGNED NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_reposts_users1`
    FOREIGN KEY (`users_id`)
    REFERENCES `whisperdb`.`users` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_reposts_posts1`
    FOREIGN KEY (`posts_id`)
    REFERENCES `whisperdb`.`posts` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_reposts_users1_idx` ON `whisperdb`.`reposts` (`users_id` ASC) VISIBLE;

CREATE INDEX `fk_reposts_posts1_idx` ON `whisperdb`.`reposts` (`posts_id` ASC) VISIBLE;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
