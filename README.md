# Blueprint hexagonal architecture in Rust

:wave: This project goal is to provide a blueprint of an implementation of hexagonal architecture in Rust.  
:warning: This project is in a work in progress mode and will be updated when I will find the time (and the courage) to work on it

This project is a tiny task scheduler, you can create some tasks that will be run and get their status.

## Setup

### Configuration

To initialize database please install [`cargo install diesel_cli`](https://github.com/diesel-rs/diesel/tree/master/diesel_cli#installation).

Run the migrations at root of the project: `diesel migration run`

### Build

Use cargo for build : `cargo build`

### Execute

```
tasc 0.1.0

USAGE:
    blueprint-hexagonal-infra [OPTIONS] [command]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>    Optional : Name of the task for later querying

ARGS:
    <command>...    Command to be executed by the task

```

### Database connection

`sqlite3 test.db`

## Hexagonal architecture

Domain & Infra code are split in two projects.  
I know it is not perfect and it could be improved (and it will be) but there is all the basics of hexagonal architecture from my point of view :
 - Separation of domain and infrastructure code
 - Portable domain
 - Composition in infra code to execute as wanted 
 
Storage can be in memory using `adapter::secondary::storage::TaskStorageAdapter` or with sqlitedb using `adapter::secondary::database::TaskDatabaseStorageAdapter`

### TODO

 - [x] Improve genericity for the domain using `Into` and `From` (limitation on secondary ports see [notes](Notes.md#))
 - [x] Add unit tests
 - [x] Add real life adapter
 - [x] CLI Adapter for input
 - [ ] Improve error management
 - [ ] Add input validation
 - [ ] Improve documentation
 - [ ] Split task execution
 - [ ] Run migration through code for database
 
 #### [Notes](Notes.md#)
 
 See notes describing choices and limitations during the development process.  
 If something could be done, please raise an issue with comment to explain what I did get wrong.