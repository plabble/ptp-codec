use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};

use crate::scripting::interpreter::ScriptError;

/**
 * The script engine uses Opcodes as the scripting language
 * The stack uses 4 data types: bytes, numbers, booleans and byte.
 * If a boolean is expected but a number provided, 0 and 1 will work. 2+ will fail.
 * If a boolean is expected but bytes are provided, the first byte will tried as 0x00 or 0x01.
 * If a number is expected but bytes are provided, the engine will try to read a dynint.
 */
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, ToBytes, FromBytes)]
pub enum Opcode {
    FALSE = 0, // Push 0x00 to the stack
    TRUE = 1,  // Push 0x01 to the stack

    PUSH1(u8) = 2,      // Push next byte to the stack
    PUSH2([u8; 2]) = 3, // Push next 2 bytes to the stack
    PUSH4([u8; 4]) = 4, // Push next 4 bytes to the stack

    // Push n bytes to the stack, where n is u8 value directly following the operator
    PUSHL1 {
        #[length_for = "l1"]
        len: u8,
        #[length_by("l1")]
        data: Vec<u8>,
    } = 5,

    // Push n bytes to the stack, where n is u16 value directly following the operator
    PUSHL2 {
        #[length_for = "l2"]
        len: u16,
        #[length_by("l2")]
        data: Vec<u8>,
    } = 6,

    // Push n bytes to the stack, where n is u32 value directly following the operator
    PUSHL4 {
        #[length_for = "l4"]
        len: u32,
        #[length_by("l4")]
        data: Vec<u8>,
    } = 7,

    // Push dynamic int to the stack
    PUSHINT(#[dyn_int] i128) = 8,

    // Push floating point number to the stack. Floats are 64-bit IEEE 754 binary64
    PUSHFLOAT(f64) = 9,

    /* END OF PUSH-ONLY OPCODES */
    // Numeric operations - all numbers are signed Plabble dynints
    ADD = 10,  // Pop two numbers from the stack and sum them
    SUB = 11, // Pop two numbers from the stack and substract them
    MUL = 12, // Pop two numbers from the stack and multiply them
    DIV = 13, // Pop two numbers from the stack and divide them
    MOD = 14, // Pop two numbers from the stack and modulo divide them
    
    NEG = 15, // Pop one number and negate it
    ABS = 16, // Pop one number and make it positive
 
    // Floating point numeric operations
    FADD = 17, // Pop two floats from the stack and sum them
    FSUB = 18, // Pop two floats from the stack and substract them
    FMUL = 19, // Pop two floats from the stack and multiply them
    FDIV = 20, // Pop two floats from the stack and divide them
    FMOD = 21, // Pop two floats from the stack and modulo divide them

    // Numberic comparation operations - all numbers are signed Plabble dynints. Returns FALSE of TRUE
    LT = 22,  // Pop two numbers and check if second number is smaller
    GT = 23,  // Pop two numbers and check if second number is greater
    LTE = 24, // Pop two numbers and check if second number is smaller or equal
    GTE = 25, // Pop two numbers and check if second number is greater or equal
    MIN = 26, // Pop two numbers and return smallest
    MAX = 27, // Pop two numbers and return largest

    // Floating point numeric comparation operations. Returns FALSE of TRUE
    FLT = 28,  // Pop two floats and check if second number is smaller
    FGT = 29,  // Pop two floats and check if second number is greater
    FLTE = 30, // Pop two floats and check if second number is smaller or equal
    FGTE = 31, // Pop two floats and check if second number is greater or equal
    FMIN = 32, // Pop two floats and return smallest
    FMAX = 33, // Pop two floats and return largest
    // 34, 35
    FLOOR = 36, // Pop one float and round it down to nearest integer 
    CEIL = 37, // Pop one float and round it up to nearest integer
    ROUND = 38, // Pop one float and round it to nearest integer
    ROUNDE = 39, // Pop one float and round it to nearest integer, rounding even

    // Binary numeric operations
    BAND = 40, // Pop two numbers and perform bitwise AND
    BOR = 41,  // Pop two numbers and perform bitwise OR
    BXOR = 42, // Pop two numbers and perform bitwise XOR
    BSHL = 43, // Pop two numbers, shift first left by second
    BSHR = 44, // Pop two numbers, shift first right by second
    BNOT = 45, // Pop one number and bitwise NOT

    // Boolean/logic operations
    NOT = 50, // Pop boolean and invert it
    AND = 51, // Pop two booleans and check if both true
    OR = 52,  // Pop two booleans and check if one is true
    XOR = 53, // Pop two booleans and check if exactly ONE is true

    EQ = 54,  // Pop two items and check if they are equal
    NEQ = 55, // Pop two items and check if they are different
    // 56 - 60

    // Advanced numerics
    POW = 60,  // Pop two numbers, calculate exponent
    SQRT = 61, // Pop one float, calculate square root
    // 62 - 69

    // Control flow
    NOP = 70,   // Do nothing
    IF = 71,    // If statement (takes boolean from stack)
    ELSE = 72,  // Else statement
    FI = 73,    // End if
    BREAK = 74, // Break loop (skip to next POOL)
    LOOP = 75,  // Start loop
    POOL = 76,  // End of loop
    JMP = 77,   // Jump to address (takes unsigned dynint as address from stack)

    ASSERT = 78, // Crash if top is not true
    RETURN = 79, // Stop execution, return stack as result
    // 80 - 89

    // Stack manipulation
    DUP = 90,  // Duplicate top item of stack
    DUP2 = 91, // Duplicate top two items of stack
    DUP3 = 92, // Duplicate top three items of stack
    DUP4 = 93, // Duplicate top four items of stack
    DUPN(u8) = 94, // Duplicate top item of stack N times (takes byte for count from script)

    SWAP = 95,   // Swap top two items of stack
    ROT = 96,    // Rotate top three items of stack
    POP = 97,    // Take one item from stack
    COPY = 98, // Take the item at index n (takes unsigned dynint as address from stack) and copy it to top
    BUBBLE = 99, // Take the item at index n (takes unsigned dynint as address from stack) and move it to top
    SINK = 100, // Take the item at index n (takes unsigned dynint as address from stack) and move it to bottom

    TOALT = 101,    // Move top item to alt (other) stack
    FROMALT = 102,  // Move top item from alt (other) stack back
    SNAPSHOT = 103, // Store a snapshot of the current stack
    RESTORE = 104,  // Restore the snapshot (replaces current stack)
    CLEAR = 105,    // Clear current stack
    SWITCH = 106,   // Switches between current and alt stack
    CONCAT = 107,   // Merge top two items together as bytes
    COUNT = 108,    // Push number of items in stack to stack (excluding itself)
    // 109 - 119

    // Casts
    NUMBER = 120, // Cast current value to number. This is only needed if you for instance want to compare a byte array as a number to a number
    FLOAT = 121,  // Cast current value to float. Floats are 64-bit IEEE 754 binary64
    // 122 - 129,

    // Bucket operations
    SERVER = 130, // Connect to other server. Takes address from stack
    SELECT = 131, // Select bucket by ID (takes 16 bytes for bucket ID)
    READ = 132, // Read numeric slot of bucket (takes 2 bytes for u16 bucket index) and push result to stack
    WRITE = 133, // Write numeric slot to bucket. Takes 2 bytes for slot, content from stack
    APPEND = 134, // Write to bucket, next free slot. Takes content from stack
    DELETE = 135, // Delete slot from bucket. Takes 2 bytes for slot number.
    // 136 - 139

    // Slice operations
    LEN = 140,     // Pops top item from stack and returns slice length
    REVERSE = 141, // Reverse bytes of top item
    SLICE = 142, // Slice bytes from existing byte array (copy). Takes 2 numbers, one for offset, one for length.
    SPLICE = 143, // Splice bytes from existing byte array (modifies). Takes 2 numbers, one for range offset, one for range length, then bytes to put, then bytes to splice into
    // 144 - 149

    // Crypto operations
    HASH = 150,    // Take byte for algorithm, hashes bytes and put them back on stack
    SIGN = 151, // Take byte for algorithm, signature, data to sign and puts signature back on stack
    VERIFY = 152, // Takes byte for algorithm, public key, signature, data and puts boolean back
    ENCRYPT = 153, // Takes byte for algorithm, key, data and puts encrypted data back
    DECRYPT = 154, // Takes byte for algorithm, key, ciphertext and puts plain data back
    // 155 - 159

    // TODO: missing (slice) operations for modifying strings/byte arrays 
    // (INDEXOF, SPLIT, CONTAINS, STARTSWITH?(can be done with slice), ENDSWITH, JOIN)

    // Special: 200+
    TIME = 200, // Push the current time as a Plabble numeric timestamp to the stack

    EVALSUB = 254, // Evaluate top stack item as if it is a script in a child process and push the result back
    EVAL = 255, // Evaluate stack bytes as if it is a script against the current stack (dangerous)
}

/* Example script

[16]            PUSHINT 16
[16, 2]         PUSHINT 2
[32]            MUL
[32, 32]        PUSHINT 32
[true]          EQ
[]              ASSERT

*/

#[derive(Debug, Clone)]
pub struct ScriptSettings {
    pub memory_limit: usize,
    pub executions_limit: usize,
    pub opcode_limit: usize,
    pub search_limit: usize,
    pub max_slice_size: usize,
    pub max_stack_items: usize,
    pub max_script_len: usize,
    pub max_nesting_depth: usize,

    pub allow_clear: bool,
    pub allow_control_flow: bool,
    pub allow_loop: bool,
    pub allow_jump: bool,
    pub allow_non_push: bool,
    pub allow_eval: bool,
    pub allow_sandboxed_eval: bool,
    pub allow_bucket_actions: bool,
}

impl Default for ScriptSettings {
    fn default() -> Self {
        Self {
            memory_limit: 10_000,
            executions_limit: 1000,
            search_limit: 1000,
            opcode_limit: 100,
            max_slice_size: 8000, // 8 kB
            max_stack_items: 100,
            max_script_len: 20_000,
            max_nesting_depth: 10,
            allow_clear: true,
            allow_loop: true,
            allow_jump: true,
            allow_control_flow: true,
            allow_non_push: true,
            allow_eval: true,
            allow_sandboxed_eval: true,
            allow_bucket_actions: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToBytes, FromBytes)]
pub struct OpcodeScript {
    pub instructions: Vec<Opcode>,
}

impl OpcodeScript {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        Self { instructions }
    }

    /// If we want a locking/unlocking script experience like Bitcoin,
    /// we don't want the unlocker to use any control statement or whatever.
    /// The unlocker script should only contain push statements and be put BEFORE the locking script
    pub fn is_push_only(&self) -> bool {
        self.instructions.iter().all(|i| i.get_discriminator() < 10)
    }

    /// Generate a jump target map for this script
    pub fn generate_jump_target_map(&self) -> Result<HashMap<usize, usize>, ScriptError> {
        let mut if_stack = Vec::new();
        let mut else_stack = Vec::new();
        let mut loop_stack = Vec::new();
        let mut break_map = HashMap::new();
        let mut targets = HashMap::new();

        for (address, opcode) in self.instructions.iter().enumerate() {
            match opcode {
                Opcode::IF => {
                    if_stack.push(address);
                }
                Opcode::ELSE => {
                    // IF jumps to ELSE
                    let if_pos = if_stack.pop().ok_or(ScriptError::ControlFlowMalformed)?;
                    targets.insert(if_pos, address);
                    else_stack.push(address);
                }
                Opcode::FI => {
                    if let Some(else_pos) = else_stack.pop() {
                        // ELSE jumps to FI
                        targets.insert(else_pos, address);
                    } else {
                        let if_pos = if_stack.pop().ok_or(ScriptError::ControlFlowMalformed)?;
                        // IF false jumps to FI
                        targets.insert(if_pos, address);
                    }
                }
                Opcode::LOOP => {
                    loop_stack.push(address);
                }
                Opcode::POOL => {
                    let loop_pos = loop_stack.pop().ok_or(ScriptError::ControlFlowMalformed)?;

                    // POOL jumps back to LOOP
                    targets.insert(address, loop_pos);

                    // If BREAK is present, it will jump to POOL
                    // Resolve all BREAKs for this loop
                    if let Some(breaks) = break_map.remove(&loop_pos) {
                        for break_pos in breaks {
                            targets.insert(break_pos, address);
                        }
                    }
                }
                Opcode::BREAK => {
                    let loop_pos = loop_stack.last().ok_or(ScriptError::ControlFlowMalformed)?;
                    break_map
                        .entry(*loop_pos)
                        .or_insert_with(Vec::new)
                        .push(address);
                }
                _ => {}
            }
        }

        if !if_stack.is_empty()
            || !else_stack.is_empty()
            || !loop_stack.is_empty()
            || !break_map.is_empty()
        {
            return Err(ScriptError::ControlFlowMalformed);
        }

        Ok(targets)
    }
}

#[cfg(test)]
pub mod tests {
    use binary_codec::{BinaryDeserializer, SerializerConfig};

    use crate::scripting::opcode_script::{Opcode, OpcodeScript};

    #[test]
    fn can_deserialize_simple_script() {
        let bytes = vec![0x01, 0x02, 1, 54];
        let config: Option<&mut SerializerConfig> = None;
        let script = OpcodeScript::from_bytes(&bytes, config).unwrap();

        assert_eq!(
            script.instructions,
            vec![Opcode::TRUE, Opcode::PUSH1(1), Opcode::EQ]
        );
    }

    #[test]
    fn can_determine_if_script_is_push_only() {
        let mut script = OpcodeScript::new(vec![
            Opcode::FALSE,
            Opcode::TRUE,
            Opcode::PUSHINT(5),
            Opcode::PUSHINT(7),
        ]);

        assert_eq!(script.is_push_only(), true);
        script.instructions.push(Opcode::ADD);
        assert_eq!(script.is_push_only(), false);
    }
}
