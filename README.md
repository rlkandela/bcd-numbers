# BCD Numbers

Currently I have no time for writting a decent README so I just pasted some examples of how to use the crate. I will update the README as I improve the crate when I have more time.

```
// From decimal to BCD<N>
let bcd1234: BCD<2> = BCD::new(1234);
let bcd1528: BCD<2> = BCD::new(1528);
let bcd9999: BCD<2> = BCD::new(9999);
let bcd7612: BCD<2> = BCD::new(7612);
println!("1234 -> {:#04X?}", bcd1234);
println!("1528 -> {:#04X?}", bcd1528);
println!("9999 -> {:#04X?}", bcd9999);
println!("7612 -> {:#04X?}", bcd7612);

// From hex to BCD<N>
let bcd1234: BCD<2> = BCD::from([0x12, 0x34]);
let bcd1528: BCD<2> = BCD::from([0x15, 0x28]);
let bcd9999: BCD<2> = BCD::from([0x99, 0x99]);
let bcd7612: BCD<2> = BCD::from([0x76, 0x12]);
println!("0x1234 -> {:#04X?}", bcd1234);
println!("0x1528 -> {:#04X?}", bcd1528);
println!("0x9999 -> {:#04X?}", bcd9999);
println!("0x7612 -> {:#04X?}", bcd7612);

// Getting inner decimal number
let bcd1234: BCD<2> = BCD::from([0x12, 0x34]);
let number = bcd1234.get_number();
println!("{}", number);
```
