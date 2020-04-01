# Blueprint hexagonal architecture in Rust

:wave: This project goal is to provide a blueprint of an implementation of hexagonal architecture in Rust.  
:warning: This project is in a work in progress mode and will be updated when I will find the time (and the courage) to work on it

This project is a tiny task scheduler, you can create some tasks that will be run and get their status.

## Hexagonal architecture

For hexagonal architecture presentation please refer to [Alistair Cockburn presentation](https://alistair.cockburn.us/hexagonal-architecture/)

_This project:_  
Domain & Infra code are split in two projects.  
I know it is not perfect and it could be improved (and it will be) but there is all the basics of hexagonal architecture from my point of view :
 - Separation of domain logic and infrastructure (side-effects) code
 - Portable domain
 - Testable domain
 - Composition in infra code to execute as wanted
 - Each adapter have a proper model for its purpose

### Domain
![domain schema](doc/domain_schema.png) 

_Ports_ :  
- __TaskSchedulePort__ : Contract to schedule some Task and get their status
- __TaskStoragePort__ : Contract to store tasks and their executions
- __TaskExecutionPort__ : Contract for task execution
- __IdGeneratorPort__ : Contract to generate ids for tasks


### Infra
![infra schema](doc/infra_schema.png) 

_Adapters_ :  
- __CLI Input (_TaskCliInput_)__ : Input of the application via command line
- __UUID IdGenerator (_IdGeneratorAdapter_)__ : Ig generator based on UUID
- __Local ExecutionAdapter (_TaskExecutionAdapter_)__ : Task execution adapter on local machine
- __Database StorageAdapter (_TaskDatabaseStorageAdapter_)__ : Database storage
- __InMemory StorageAdapter (_TaskStorageAdapter_)__ : InMemory storage

### Composability

Storage can be in memory using `adapter::secondary::storage::TaskStorageAdapter` or with sqlitedb using `adapter::secondary::storage::database::TaskDatabaseStorageAdapter`

## Setup

### Configuration

Application need env var `DATABASE_URL` with database usage (example `export DATABASE_URL=test.db`). This configruation is needed for migration, and run of the project.

To initialize database please install [`cargo install diesel_cli`](https://github.com/diesel-rs/diesel/tree/master/diesel_cli#installation).

Run the migrations at root of the project: `diesel migration run`

### Build

Use cargo for build : `cargo build`

### Execute

_Run from `cargo` :_  
```
 cargo run -- --name test ls /home
 # compilation logs ...
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/blueprint-hexagonal-infra --name test ls /home`
Task status is Success("fteychene\nlinuxbrew\n")
```

_Command line execution :_
```
 ./target/debug/blueprint-hexagonal-infra --help
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

```
 sqlite3 test.db
SQLite version 3.31.1 2020-01-27 19:55:54
Enter ".help" for usage hints.
sqlite> select * from tasks;
1477bb1b-73b2-468e-a4e2-fc571423da25||ls /home||SUCCESS|fteychene
linuxbrew

```

## TODO

 - [x] Improve genericity for the domain using `Into` and `From` (limitation on secondary ports see [#limitations](#into-for-secondary-ports))
 - [x] Add unit tests
 - [x] Add real life adapter
 - [x] CLI Adapter for input
 - [ ] Improve error management
 - [ ] Add input validation
 - [ ] Improve documentation
 - [ ] Split task execution
 - [ ] Run migration through code for database
 
## Limitations

### Into for secondary ports

__Goal__ :
Main goal was to provide a way for adapters to not adapt their internal domains to application domain as function result.  
To provide this feature we would like to provide functions of secondary ports to return types with only `Into<DomainStruct>` constraint.

__Issue__ :
```
error[E0038]: the trait `std::convert::Into` cannot be made into an object
  --> infra/src/main.rs:37:1
   |
37 | fn test() -> Box<dyn Into<u8>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::Into` cannot be made into an object
   |
   = note: the trait cannot require that `Self : Sized`

error: aborting due to previous error
```

The trait `Into` cannot be used as part of a dynamic type due to [error E0038](https://doc.rust-lang.org/error-index.html#E0038)