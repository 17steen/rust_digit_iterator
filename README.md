# digit-iterator

Simple library allowing you to quickly iterate over digits of unsigned integers.

calling `as_digits::<BASE>()` results in an Iterator, any function implementing Iterator will then be available


###### Getting the sum:

```rust
let n = 123;
let digit_sum = n.as_digits::<10>().sum(); 
assert_eq!(123, 6);
```


###### Placing them in a vector:
```rust
let n = 0xBABE;
let digits = n.as_digits::<16>().collect_vec();
//notice the inverted order
assert_eq!(digits, vec![0xE, 0xB, 0xA, 0xB]);
```

