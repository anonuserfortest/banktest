# BankApp

## Assumptions

1. Amounts are somewhat realistic, I implemented my own type to handle the currency due to the constraints put on the precision which allows all "realistic" values to fit into a i64 even with 4 decimals of precision. Currently the limit is about 900 trillion for the max amount it can handle, but as that is about 30 times more than the worlds wealth I think it should be fine

2. ClientId's are valid u16, since they are valid `u16`'s then it's quite fast to store them all in a single vector instead of using a HashMap

3. Disputes are rare, from my quick reading it seems that disputes should happen less than 1% of the time in a healthy business, thus I chose to optimise based on this assumption


## Safety and Robustness

Errors are handled by wrapping them in a `Result` such that they can be handled, disregarding io/parsing errors, then the actual errors coming from the application such as attempting to withdraw to much money is not handled and are simply ignored as per the instructions in the assignment, one should probably log these errors atleast in a real world scenario

## Efficiency

Much effort was put into squeezing performance out, such as the assumptions that disputes are rare leading a more optimal solution for the common case, thus the payment engine is able to handle around 42000 transaction per ms on my machine(Ryzen 5900x). But the overall runtime is most constrained by io and parsing. Originally the `csv` parser from the `csv` crate was used, but I decided to try and make my own since about 80% of the total runtime was in the parsing. The new parser halfed the amount of time it took to parse the file, which was pretty significant since it was such a big chunk of the runtime.

One might be able to gain some extra speedup for paralizing the parser.