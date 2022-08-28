60 0A // PUSH1 10 - Push 10 (decimal) on the stack (length of runtime code)
60 0C // PUSH1 0C - Push 12 (decimal) on the stack (position of code in calldata)
60 00 // PUSH1 00 - Push 0 on the stack (memory location to store code)
39 // CODECOPY

// RETURN
60 0A // PUSH1 10 - Push 10 on the stack (length of runtime code)
60 00 // PUSH 00 - Push 0 on the stack (memory location of stored code)
F3 // Return