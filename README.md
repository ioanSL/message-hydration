# message-hydration

## Test the contract

To test the contract, run:

`cargo test`

If you want to see more details about the results, run:

`cargo test -- --nocapture`

## Notes:

Here are some notes about the code. The result of hydrating the message is not the expected. Compared to the output provided in the doc, this is what went wrong.

After successfully populating the input string with the given variables, on contract.rs line 93 I'm converting the Cw20ExecuteMsg message to Binary. The code seems to work fine but when I print the CosmosWasm result, I can still see a string. What I'm expecting is a vector of binaries which later can be converted to base64 and compare the result with the expected value.

Seems like converting to binary twice does the trick but is completelly wrong and I don't even get the expected result.

What I believe I'm doing wrong is to convert the msg field (witch is type Binary) to String in line 71 and 80. This conversion is very convenient because I can easily replace the variable names with the correspondent value using the function replace_variables. But I think I shouldn't use this method and there might be a more efficient way to solve this.

I've also tried to replace String::from_utf8 conversion from lines 71 and 80 but I get a conversion error every time.