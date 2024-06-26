# DRY SQL
__***CLI tool to dry run your SQL scripts***__

## Background
This tool was created when I was setting up some database migrations stuffs in the CI pipeline that was supposed to be executed on another environment. The migration should be applied on main branch. It's mandatory that specific migrations in a the pull request get successfully applied before the app is deployed.

If the team practices DB migrations strictly, then the presence of migration issues would be minimalized or nihilized. However,developers are human after all. Some migrations scripts some times failed to execute due to some human errors. Hence, there is a need to validate the migration scripts which are SQL scripts without committing them.

While some SQL script linter have benefits to check for SQL statement for errors, they don't validate whether your scripts would be successfully executed. There are several reasons why a SQL script might fail to execute even if it passes the linter:

- ***Data-related Issues***: The script may encounter unexpected data conditions or constraints during execution that were not accounted for in the linting process.
- ***Database Schema Changes***: If the database schema changes after the linting process, the script may no longer be compatible with the updated schema.

## Use case example
The most common use case would be obviously to validate migration SQL scripts in the pipeline :). Of course, you can use it to mimick your SQL scripts anywhere else.

### How to Use

On the terminal:

if you want to dry-run a SQL file.

```bash
dry_sql --host <host> --port <port> --user <username> --password <password> --file <path/to/your/sql/script>
```

if you want to dry-run multiple a SQL inside a folder.

```bash
dry_sql --host <host> --port <port> --user <username> --password <password> --dir <path/to/your/sql/script/foder>
```

## Limitation
Currently, this tool only support Postgres SQL. The rest will be coming along.

## Stack
Written in Rust. 

