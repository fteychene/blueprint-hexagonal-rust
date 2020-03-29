# Blueprint hexagonal architecture in Rust

:wave: This project goal is to provide a blueprint of an implementation of hexagonal architecture in Rust.  
:warning: This project is in a work in progress mode and will be updated when I will find the time (and the courage) to work on it

This project is a tiny task scheduler, you can create some tasks that will be run and get their status.

### Execute

`cargo run`

## Hexagonal architecture

Domain & Infra code are split in two projects.  
I know it i not perfect and it could be improved (and it will be) but tere is all the basics of hexagonal architecture from my point of view :
 - Separation of domain and infrastructure code
 - Portable domain
 - Composition in infra code to execute as wanted

### TODO

 - [x] Improve genericity for the domain using `Into` and `From` (limitation on secondary ports see [notes](Notes.md#))
 - [ ] Add input validation
 - [ ] Add unit tests
 - [ ] Add real life adapter
 - [ ] Improve error management
 - [ ] Improve documentation
 - [ ] Split task execution
 
 #### [Notes](Notes.md#)
 
 See notes describing choices and limitations during the development process.  
 If something could be done, please raise an issue with comment to explain what I did get wrong.