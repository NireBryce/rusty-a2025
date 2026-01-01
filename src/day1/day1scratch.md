simulating a safe dial

0 to 100 but it loops around

starts by pointing at 50

`let mut dial_position: i8 = 50;`
- needs to be mutable because we're keeping track of something with it

need to accumulate the number of times it lands on 0 after an instruction
`let mut password: u8 = 0;`

for each instruction, if position % 100 = 0, password++

println!("Password: {}", password);


