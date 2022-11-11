## How to setup a development environment

### you need to have a couple of things installed on your machine
+ rust
+ PostgresQl
+ diesel-cli

### then you need to run the following commands to to setup the database
#### first run : 
         diesel setup --database-url='{database_url on your machine}'
#### then you need to run the migrations:
         diesel migration run --database-url='{database_url}'
#### unfortunately I wasn't able to read the database_url from .env file so I hard coded it (BAD IDEA, trust me I know), so you need to change that as well.