/**
 * The script engine uses Opcodes as the scripting language
 * The stack uses 4 data types: bytes, numbers, booleans and byte.
 * If a boolean is expected but a number provided, 0 and 1 will work. 2+ will fail.
 * If a boolean is expected but bytes are provided, the first byte will tried as 0x00 or 0x01.
 * If a number is expected but bytes are provided, the engine will try to read a dynint.
 */
#[repr(u8)]
pub enum Opcode {
    FALSE = 0, // Push 0x00 to the stack
    TRUE = 1,  // Push 0x01 to the stack

    PUSH1 = 2, // Push next byte to the stack
    PUSH2 = 3, // Push next 2 bytes to the stack
    PUSH4 = 4, // Push next 4 bytes to the stack

    PUSHL1 = 5, // Push n bytes to the stack, where n is u8 value directly following the operator
    PUSHL2 = 6, // Push n bytes to the stack, where n is u16 value directly following the operator
    PUSHL4 = 7, // Push n bytes to the stack, where n is u32 value directly following the operator
    PUSHINT = 8, // Push dynamic int to the stack

    // Numeric operations - all numbers are signed Plabble dynints
    ADD = 9,  // Pop two numbers from the stack and sum them
    SUB = 10, // Pop two numbers from the stack and substract them
    MUL = 11, // Pop two numbers from the stack and multiply them
    DIV = 12, // Pop two numbers from the stack and divide them
    MOD = 13, // Pop two numbers from the stack and modulo divide them
    NEG = 14, // Pop one number and negate it
    ABS = 15, // Pop one number and make it positive

    // Numberic comparation operations - all numbers are signed Plabble dynints. Returns FALSE or TRUE
    LT = 16,  // Pop two numbers and check if second number is smaller
    GT = 17,  // Pop two numbers and check if second number is greater
    LTE = 18, // Pop two numbers and check if second number is smaller or equal
    GTE = 19, // Pop two numbers and check if second number is greater or equal
    MIN = 20, // Pop two numbers and return smallest
    MAX = 21, // Pop two numbers and return largest

    // Binary numeric operations
    BAND = 22, // Pop two numbers and perform bitwise AND
    BOR = 23,  // Pop two numbers and perform bitwise OR
    BXOR = 24, // Pop two numbers and perform bitwise XOR
    BSHL = 25, // Pop two numbers, shift first left by second
    BSHR = 26, // Pop two numbers, shift first right by second
    BNOT = 27, // Pop one number and bitwise NOT

    // Boolean/logic operations
    NOT = 28, // Pop boolean and invert it
    AND = 29, // Pop two booleans and check if both true
    OR = 30,  // Pop two booleans and check if one is true
    XOR = 31, // Pop two booleans and check if exactly ONE is true

    EQ = 32,  // Pop two items and check if they are equal
    NEQ = 33, // Pop two items and check if they are different

    // Advanced numerics
    POW = 34,  // Pop two numbers, calculate exponent
    SQRT = 35, // Pop two numbers, calculate square root
    // 36 - 40

    // Control flow
    NOP = 41,    // Do nothing
    IF = 42,     // If statement (takes boolean)
    ELSE = 43,   // Else statement
    ENDIF = 44,  // End if
    BREAK = 45,  // Break if (skip to next ENDIF, or POOL)
    LOOP = 46,   // Start loop
    POOL = 47,   // End of loop
    JMP = 48,    // Jump to address (takes unsigned dynint as address)
    ASSERT = 49, // Crash if top is not true
    RETURN = 50, // Stop execution, return stack as result

    // Stack manipulation
    DUP = 51,      // Duplicate top item of stack
    DUP2 = 52,     // Duplicate top two items of stack
    DUP3 = 53,     // Duplicate top three items of stack
    DUP4 = 54,     // Duplicate top four items of stack
    DUPN = 55,     // Duplicate top item of stack N times (takes dynint for count)
    SWAP = 56,     // Swap top two items of stack
    ROT = 57,      // Rotate top three items of stack
    POP = 58,      // Take one item from stack
    COPY = 59,     // Take the item at index n (takes unsigned dynint as address) and copy it to top
    BUBBLE = 60,   // Take the item at index n (takes unsigned dynint as address) and move it to top
    SINK = 61,     // Take the top item and move it to index n (replace)
    TOALT = 62,    // Move top item to alt (other) stack
    FROMALT = 63,  // Move top item from alt (other) stack back
    SNAPSHOT = 64, // Store a snapshot of the current stack
    RESTORE = 65,  // Restore the snapshot (pushes items to current stack)
    CLEAR = 66,    // Clear current stack
    SWITCH = 67,   // Switches between current and alt stack
    CONCAT = 68,   // Merge top two items together as bytes
    COUNT = 69,    // Push number of items in stack to stack (excluding itself)
    // 70 - 80,

    // Bucket operations
    SERVER = 81, // Connect to other server. Takes address from stack
    SELECT = 82, // Select bucket by ID (takes 16 bytes for bucket ID)
    READ = 83, // Read numeric slot of bucket (takes 2 bytes for u16 bucket index) and push result to stack
    WRITE = 84, // Write numeric slot to bucket. Takes 2 bytes for slot, content from stack
    APPEND = 85, // Write to bucket, next free slot. Takes content from stack
    DELETE = 86, // Delete slot from bucket. Takes 2 bytes for slot number.
    // 87 - 90

    // Slice operations
    LEN = 91,     // Push length of top item to the stack
    REVERSE = 92, // Reverse bytes of top item

    // Crypto operations
    HASH = 101, // Take byte from stack for algorithm, hashes bytes and put them back on stack
    SIGN = 102, // Take byte from stack for algorithm, signature, data to sign and puts signature back on stack
    VERIFY = 103, // Takes byte from stack for algorithm, public key, signature, data and puts boolean back
    ENCRYPT = 104, // Takes byte from stack for algorithm, key, data and puts encrypted data back
    DECRYPT = 105, // Takes byte from stack for algorithm, key, ciphertext and puts plain data back

    EVAL = 255, // Evaluate stack bytes as if it is a script
}

/* Example script

[16]            PUSHINT 16
[16, 2]         PUSHINT 2
[32]            MUL
[32, 32]        PUSHINT 32
[true]          EQ
[]              ASSERT

*/
