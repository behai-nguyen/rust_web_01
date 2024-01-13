#
# 29/11/2023.
#

#
# Add columns `email` and `password` to table `employees`.
#
ALTER TABLE `employees` 
ADD COLUMN `email` VARCHAR(255) NULL AFTER `emp_no`,
ADD UNIQUE INDEX `email_unique` (`email` ASC);

ALTER TABLE `employees` 
ADD COLUMN `password` VARCHAR(100) NULL AFTER `email`;

#
# Set values for `email` and `password`.
# 
update `employees` set
  `email` = concat(lcase(first_name), '.', lcase(last_name), '.', convert(emp_no, char(6)), '@gmail.com'),
  `password` = '$argon2id$v=19$m=16,t=2,p=1$cTJhazRqRWRHR3NYbEJ2Zg$z7pMnKzV0eU5eJkdq+hycQ';

#
# Set columns `email` and `password` to not null.
# 
ALTER TABLE `employees` MODIFY `email` VARCHAR(255) not null;
ALTER TABLE `employees` MODIFY `password` VARCHAR(100) not null;
