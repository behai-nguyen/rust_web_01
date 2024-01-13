-- 
-- 30/11/2023.
--

-- 
-- Create a new temporary employees, tmp_emp, table with additional email 
-- and password columns at the desired positions.
-- 
CREATE TABLE IF NOT EXISTS employees.tmp_emp
(
    emp_no integer NOT NULL PRIMARY KEY,
    email character varying(255) COLLATE pg_catalog."default" NOT NULL UNIQUE,
    password character varying(100) COLLATE pg_catalog."default" NOT NULL,
    birth_date date NOT NULL,
    first_name character varying(14) COLLATE pg_catalog."default" NOT NULL,
    last_name character varying(16) COLLATE pg_catalog."default" NOT NULL,
    gender char(1) NOT NULL,
    hire_date date NOT NULL
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS employees.tmp_emp
    OWNER to postgres;
    
--
-- Populate temporary employees table "tmp_emp" with data from "employees" 
-- table, together with make up email and hard-coded password.
--
insert into employees.tmp_emp (
    emp_no, email, password, birth_date, first_name, last_name, gender, hire_date
)
select
    emp_no, 
    lower(first_name) || '.' || lower(last_name) || '.' || emp_no || '@gmail.com', 
    '$argon2id$v=19$m=16,t=2,p=1$cTJhazRqRWRHR3NYbEJ2Zg$z7pMnKzV0eU5eJkdq+hycQ', 
    birth_date,
    first_name, 
    last_name, 
    gender, 
    hire_date
from
    employees;

--
-- Drop "employees" table.
--
ALTER TABLE employees RENAME TO employees_old;

--
-- Rename temporary employees table "tmp_emp" to "employees" table, 
--
ALTER TABLE tmp_emp RENAME TO employees;
