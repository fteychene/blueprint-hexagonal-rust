## Notes

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