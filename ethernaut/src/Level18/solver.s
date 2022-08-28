// Based on https://monokh.com/posts/ethereum-contract-creation-bytecode
// EXECUTION
60 2A // PUSH1 2A - Push 42 (decimal) on the stack
60 1F // PUSH1 1F - Push 1F on the stack (last byte of uint256)
53 // MSTORE - Store result to memory (32 byte word size)

// RETURN
60 20 // PUSH1 32 - Push 32 on the stack (length of data to return, 32 bytes)
60 00 // PUSH1 0 - Push 0 on the stack (memory location to return)
F3 // RETURN

// CONTRACT IS 10 BYTES LONG