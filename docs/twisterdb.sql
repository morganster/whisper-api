-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema twisterdb
-- -----------------------------------------------------
DROP SCHEMA IF EXISTS `twisterdb` ;

-- -----------------------------------------------------
-- Schema twisterdb
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `twisterdb` ;
USE `twisterdb` ;

-- -----------------------------------------------------
-- Table `twisterdb`.`users`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`users` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`users` (
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
-- Table `twisterdb`.`twisters`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`twisters` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`twisters` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `content` VARCHAR(280) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  `user_id` INT UNSIGNED NULL,
  `reply_to` BIGINT UNSIGNED NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_twist_user`
    FOREIGN KEY (`user_id`)
    REFERENCES `twisterdb`.`users` (`id`)
    ON DELETE SET NULL
    ON UPDATE CASCADE)
ENGINE = InnoDB;

CREATE INDEX `fk_twist_user_idx` ON `twisterdb`.`twisters` (`user_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `twisterdb`.`reaction_type`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`reaction_type` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`reaction_type` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `twisterdb`.`reactions`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`reactions` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`reactions` (
  `reaction_type_id` INT UNSIGNED NOT NULL,
  `twist_id` BIGINT UNSIGNED NOT NULL,
  `user_id` INT UNSIGNED NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_reaction_reaction_type1`
    FOREIGN KEY (`reaction_type_id`)
    REFERENCES `twisterdb`.`reaction_type` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_reaction_twist1`
    FOREIGN KEY (`twist_id`)
    REFERENCES `twisterdb`.`twisters` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_reaction_user1`
    FOREIGN KEY (`user_id`)
    REFERENCES `twisterdb`.`users` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_reaction_reaction_type1_idx` ON `twisterdb`.`reactions` (`reaction_type_id` ASC) VISIBLE;

CREATE INDEX `fk_reaction_twist1_idx` ON `twisterdb`.`reactions` (`twist_id` ASC) VISIBLE;

CREATE INDEX `fk_reaction_user1_idx` ON `twisterdb`.`reactions` (`user_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `twisterdb`.`tags`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`tags` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`tags` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(45) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `twisterdb`.`twisters_has_tags`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`twisters_has_tags` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`twisters_has_tags` (
  `twisters_id` BIGINT UNSIGNED NOT NULL,
  `tags_id` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`twisters_id`, `tags_id`),
  CONSTRAINT `fk_twisters_has_tags_twisters1`
    FOREIGN KEY (`twisters_id`)
    REFERENCES `twisterdb`.`twisters` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_twisters_has_tags_tags1`
    FOREIGN KEY (`tags_id`)
    REFERENCES `twisterdb`.`tags` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_twisters_has_tags_tags1_idx` ON `twisterdb`.`twisters_has_tags` (`tags_id` ASC) VISIBLE;

CREATE INDEX `fk_twisters_has_tags_twisters1_idx` ON `twisterdb`.`twisters_has_tags` (`twisters_id` ASC) VISIBLE;


-- -----------------------------------------------------
-- Table `twisterdb`.`retwisters`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `twisterdb`.`retwisters` ;

CREATE TABLE IF NOT EXISTS `twisterdb`.`retwisters` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `users_id` INT UNSIGNED NOT NULL,
  `twisters_id` BIGINT UNSIGNED NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` DATETIME NOT NULL DEFAULT NOW(),
  PRIMARY KEY (`id`),
  CONSTRAINT `fk_retwisters_users1`
    FOREIGN KEY (`users_id`)
    REFERENCES `twisterdb`.`users` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_retwisters_twisters1`
    FOREIGN KEY (`twisters_id`)
    REFERENCES `twisterdb`.`twisters` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE INDEX `fk_retwisters_users1_idx` ON `twisterdb`.`retwisters` (`users_id` ASC) VISIBLE;

CREATE INDEX `fk_retwisters_twisters1_idx` ON `twisterdb`.`retwisters` (`twisters_id` ASC) VISIBLE;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
