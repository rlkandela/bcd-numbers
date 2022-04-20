# BCD Numbers

Currently I have no time for writting a decent README so I just pasted some examples of how to use the crate. I will update the README as I improve it when I have more time.

```
// add a use directive for ToBCD and FromBCD in order
// to use the functions `to_bcd` and `from_bcd` with
// any unsigned integer
println!("1234 -> {:#04X?}", (1234 as u16).to_bcd()?);
println!("1528 -> {:#04X?}", (1528 as u16).to_bcd()?);
println!("9999 -> {:#04X?}", (9999 as u16).to_bcd()?);
println!("7612 -> {:#04X?}", (7612 as u16).to_bcd()?);
println!("0x1234 -> {}", (0x1234 as u16).from_bcd());
println!("0x1528 -> {}", (0x1528 as u16).from_bcd());
println!("0x9999 -> {}", (0x9999 as u16).from_bcd());
println!("0x7612 -> {}", (0x7612 as u16).from_bcd());
```
