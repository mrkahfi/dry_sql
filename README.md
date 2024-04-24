# Validate PostgreSQL Script
__***without having to apply it***__

Accurately validate your PostgreSQL script by simulating its real execution without having to applying it.

## Background
This tool when I was developing using Supabase and setup some migrations stuffs in the CI pipeline. The migration should be applied on main branch. It's mandatory that specific migrations in a pull request get successfully applied before the app is deployed.

If the team implementing DB migrations strictly, then the presence of migration issues would be minimalized or nihilized.

However, some migrations scripts some times failed to execute for whatever reason due to some human error.  Hence, there is a need to validate the migration scripts which are SQL scripts before executing it.

While some SQL script linter have benefits to check for SQL statement for errors, they don't validate whether your scripts would be successfully executed.

There are several reasons why a SQL script might fail to execute successfully even if it passes the linter:

- ***Data-related Issues***: The script may encounter unexpected data conditions or constraints during execution that were not accounted for in the linting process.
- ***Database Schema Changes***: If the database schema changes after the linting process, the script may no longer be compatible with the updated schema.
- ***Permissions and Security***: The user executing the script may not have the necessary permissions to perform certain operations specified in the script.
***Database Engine Differences***: The SQL script might be written in a way that is compatible with one database engine but not with another.
- ***Resource Constraints***: The script may encounter resource limitations such as running out of memory or exceeding execution time limits.


## Use case example
The most common use case would be to validate migration SQL scripts. Of course, you can use it to accurately validate your SQL scripts anywhere else.

## Stack
Written in Rust. 

## How it works
The way how it works is really simple. It does really execute your script and does rollback. By wrapping your script inside a prepared statement that starts with `BEGIN` and ends with `ROLLBACK`, your scripts would actually be executed and rolled back afterwards.

