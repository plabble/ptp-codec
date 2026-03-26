use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::{TryFromInto, serde_as};

/**
 * The script engine uses Opcodes as the scripting language
 * The stack uses 4 data types: bytes, numbers, booleans and byte.
 * If a boolean is expected but a number provided, 0 and 1 will work. 2+ will fail.
 * If a boolean is expected but bytes are provided, the first byte will tried as 0x00 or 0x01.
 * If a number is expected but bytes are provided, the engine will try to read a dynint.
 */
#[repr(u8)]
#[serde_as]
#[derive(Debug, Clone, PartialEq, ToBytes, FromBytes, Serialize, Deserialize)]
pub enum Opcode {
    FALSE = 0, // Push 0x00 to the stack
    TRUE = 1,  // Push 0x01 to the stack

    PUSH1(u8) = 2, // Push next byte to the stack
    PUSH2(#[serde_as(as = "Hex<Lowercase>")] [u8; 2]) = 3, // Push next 2 bytes to the stack
    PUSH4(#[serde_as(as = "Hex<Lowercase>")] [u8; 4]) = 4, // Push next 4 bytes to the stack

    // Push n bytes to the stack, where n is u8 value directly following the operator
    PUSHL1 {
        #[length_for = "l1"]
        len: u8,
        #[length_by("l1")]
        #[serde_as(as = "Hex<Lowercase>")]
        data: Vec<u8>,
    } = 5,

    // Push n bytes to the stack, where n is u16 value directly following the operator
    PUSHL2 {
        #[length_for = "l2"]
        len: u16,
        #[length_by("l2")]
        #[serde_as(as = "Hex<Lowercase>")]
        data: Vec<u8>,
    } = 6,

    // Push n bytes to the stack, where n is u32 value directly following the operator
    PUSHL4 {
        #[length_for = "l4"]
        len: u32,
        #[length_by("l4")]
        #[serde_as(as = "Hex<Lowercase>")]
        data: Vec<u8>,
    } = 7,

    // Push dynamic int to the stack
    PUSHINT(
        #[dyn_int]
        #[serde_as(as = "TryFromInto<i64>")]
        i128,
    ) = 8,

    // Push floating point number to the stack. Floats are 64-bit IEEE 754 binary64
    PUSHFLOAT(f64) = 9,

    /* END OF PUSH-ONLY OPCODES */
    // Numeric operations - all numbers are signed Plabble dynints
    ADD = 10, // Pop two numbers from the stack and sum them
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
    FLOOR = 36,  // Pop one float and round it down to nearest integer
    CEIL = 37,   // Pop one float and round it up to nearest integer
    ROUND = 38,  // Pop one float and round it to nearest integer
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

    FUN(u8, u8) = 80, // Declare start of function X with Y parameters
    NUF = 81, // End function declaration. Pulls function body from script and stores it in function map with ID from FUN opcode.
    CALL(u8) = 82, // Call function with ID X (takes Y parameters from stack, where Y is determined by function declaration)
    // 83 - 89

    // Stack manipulation
    DUP = 90,      // Duplicate top item of stack
    DUP2 = 91,     // Duplicate top two items of stack
    DUP3 = 92,     // Duplicate top three items of stack
    DUP4 = 93,     // Duplicate top four items of stack
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

    // Variables (memory or persistent storage, based on context of script)
    STOREVAR(u8) = 109, // Store variable with ID (takes byte for variable ID from script, content from stack)
    LOADVAR(u8) = 110, // Load variable with ID (takes byte for variable ID from script, pushes content to stack)
    DELVAR(u8) = 111,  // Delete variable with ID (takes byte for variable ID from script)

    // 112 - 119

    // Casts
    NUMBER = 120, // Cast current value to number. This is only needed if you for instance want to compare a byte array as a number to a number
    FLOAT = 121,  // Cast current value to float. Floats are 64-bit IEEE 754 binary64
    // 122 - 129,

    // Bucket operations
    SERVER = 130, // Connect to other server. Takes address from stack
    SELECT = 131, // Select bucket by ID (takes 16 bytes for bucket ID)
    READ = 132, // Read numeric slot of bucket (takes dynamic int for u32 bucket index) and push result to stack
    WRITE = 133, // Write numeric slot to bucket. Takes dynamic int for u32 bucket index, content from stack
    APPEND = 134, // Write to bucket, next free slot. Takes content from stack
    DELETE = 135, // Delete slot from bucket. Takes dynamic int for u32 slot number.
    // 136 - 139

    // Slice operations
    LEN = 140,     // Pops top item from stack and returns slice length
    REVERSE = 141, // Reverse bytes of top item
    SLICE = 142, // Slice bytes from existing byte array (copy). Takes 2 numbers, one for offset, one for length.
    SPLICE = 143, // Splice bytes from existing byte array (modifies). Takes 2 numbers, one for range offset, one for range length, then bytes to put, then bytes to splice into
    INDEXOF = 144, // Takes a value to search for and a byte array to search in, returns index of first occurrence or -1 if not found
    SPLIT = 145, // Take a value to search for and a byte array to split, splits byte array into N at every occurrence of value and pushes the results back.
    SCOUNT = 146, // Take a value to search for and a byte array to count, counts occurrences of value in byte array and pushes count back
    // 147 - 149

    // Crypto operations
    CRYPTO(OpAlgorithm) = 150, // Perform a crypto operation based on the provided algorithm. Takes necessary parameters for the algorithm from the stack and pushes result back. See OpAlgorithm enum for details.
    // 151 - 159

    // Special: 200+
    TIME = 200, // Push the current time as a Plabble numeric timestamp to the stack

    // Blockchain: 220-230
    // TODO: see how this should work out. At least, see providers.rs and integrate them in the interpreter.rs
    CHECKLOCK = 220, // Takes a number from the stack and fails if it is bigger than the current transaction block height or time, depending on the transaction. (fails if not in the context of a transaction)
    TXID = 221, // Push the current transaction ID to the stack (fails if not in the context of a transaction)
    SELBLOCK = 222, // Take block ID from the stack and select block as context for other block-related operations (fails if not in the context of a blockchain)
    SELTX = 223, // Take transaction ID from the stack and select transaction as context for other transaction-related operations (fails if not in the context of a blockchain)
    GETENTRY = 224, // Take entry / TX ID from the stack and push raw entry data back from blockchain (fails if not in the context of a blockchain)
    CALLEXT(u8, u8) = 225, // Call external function (in smart contract) with ID X and Y parameters (pops external script ID from stack)

    EVALSUB = 254, // Evaluate top stack item as if it is a script in a child process and push the result back
    EVAL = 255, // Evaluate stack bytes as if it is a script against the current stack (dangerous)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromBytes, ToBytes, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
pub enum ScriptError {
    /// When the stack is empty while n items are required
    StackUnderflow(u32),
    /// When an operation expected a number but got something else
    NotANumber,
    /// When an operation expected a float but got something else
    NotAFloat,
    /// When an operation expected a boolean but got something else
    NotABoolean,
    /// When an operation expected UTF-8 bytes but got something else
    NotAString,
    /// When a mathematical operation fails (e.g., division by zero)
    MathError,
    /// When the script is not valid
    InvalidScript,
    /// When a not-existing address or index is provided
    OutOfBounds,
    /// When an assertion fails
    AssertionFailed,
    /// Failed to cast a value to the expected type
    InvalidType,
    /// Expected byte array with different size than actual
    InvalidSize,

    ControlFlowMalformed,
    ClearNotAllowed,
    ControlFlowNotAllowed,
    FunctionDeclarationNotAllowed,
    FunctionCallNotAllowed,
    FunctionNotFound,
    VariableNotFound,
    JumpNotAllowed,
    LoopNotAllowed,
    NonPushNotAllowed,
    EvalNotAllowed,
    BucketActionsNotAllowed,
    BucketProviderNotAvailable,
    BucketConnectionFailed,
    BucketReadFailed,
    BucketWriteFailed,
    BucketDeleteFailed,
    BlockchainProviderNotAvailable,
    BlockchainConnectionFailed,
    MaxDepthExceeded,
    SearchLimitExceeded,
    ExecutionLimitExceeded,
    OpcodeLimitExceeded,
    MemoryLimitExceeded,
    SliceLimitExceeded,
    StackHeightLimitExceeded,

    /// Any precondition failed
    PreconditionFailed,

    AlgorithmNotSupported,
    CryptoOperationFailed,
}

// TODO: all algorithms in one, we can omit codes like HASH/SIGN etc and just have a few opcodes

/// Opcode Algorithm (for CRYPTO operations)
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, ToBytes, FromBytes, Serialize, Deserialize)]
pub enum OpAlgorithm {
    // Hashing / MAC
    /// Hash using Blake2b with 128-bit output
    Blake2_128 = 0,
    /// Hash using Blake3 with 128-bit output
    Blake3_128 = 1,
    /// Hash using Blake2b with 192-bit output
    Blake2_192 = 2,
    /// Hash using Blake3 with 192-bit output
    Blake3_192 = 3,
    /// Hash using Blake2b with 256-bit output
    Blake2_256 = 4,
    /// Hash using Blake3 with 256-bit output
    Blake3_256 = 5,
    /// Hash using Blake2b with 512-bit output
    Blake2_512 = 6,
    /// Hash using Blake3 with 512-bit output
    Blake3_512 = 7,

    /// Calculate MAC using Blake2b with 128-bit output (takes key, data from stack)
    Blake2Mac = 8,
    /// Verify and assert using Blake2b with 128-bit output (takes key, data, and expected MAC from stack, calculates MAC and asserts it is correct)
    Blake2MacAssert = 9,

    /// Calculate MAC using Blake3 with 128-bit output (takes key, data from stack)
    Blake3Mac = 10,
    /// Verify and assert using Blake3 with 128-bit output (takes key, data, and expected MAC from stack, calculates MAC and asserts it is correct)
    Blake3MacAssert = 11,

    /// Calculate MAC using Poly1305 (takes key, data from stack)
    Poly1305 = 12,
    /// Verify and assert using Poly1305 (takes key, data, and expected MAC from stack, calculates MAC and asserts it is correct)
    Poly1305Assert = 13,

    // 11-19: more space for hashing/MAC algorithms

    // Signing / Verifying
    /// Sign using Ed25519 (takes secret key and message from stack, puts signature back)
    SignEd25519 = 50,
    /// Verify using Ed25519 (takes public key, message, and signature from stack, puts boolean back)
    VerifyEd25519 = 51,
    /// Verify using Ed25519 (see above) and asserts true
    VerifyAssertEd25519 = 52,

    SignEd448 = 53,
    VerifyEd448 = 54,
    VerifyAssertEd448 = 55,

    /// Sign using ML-DSA-44 (takes secret key and message from stack, puts signature back)
    SignDsa44 = 56,
    /// Verify using ML-DSA-44 (takes public key, message, and signature from stack, puts boolean back)
    VerifyDsa44 = 57,
    /// Verify using ML-DSA-44 (see above) and asserts true
    VerifyAssertDsa44 = 58,

    /// Sign using ML-DSA-65 (takes secret key and message from stack, puts signature back)
    SignDsa65 = 59,
    /// Verify using ML-DSA-65 (takes public key, message, and signature from stack, puts boolean back)
    VerifyDsa65 = 60,
    /// Verify using ML-DSA-65 (see above) and asserts true
    VerifyAssertDsa65 = 61,

    /// Sign using Falcon (takes secret key and message from stack, puts signature back)
    SignFalcon = 62,
    /// Verify using Falcon (takes public key, message, and signature from stack, puts boolean back)
    VerifyFalcon = 63,
    /// Verify using Falcon (see above) and asserts true
    VerifyAssertFalcon = 64,

    /// Sign using SLH-DSA-SHA128s (takes secret key and message from stack, puts signature back)
    SignSlhDsaSha128s = 65,
    /// Verify using SLH-DSA-SHA128s (takes public key, message, and signature from stack, puts boolean back)
    VerifySlhDsaSha128s = 66,
    /// Verify using SLH-DSA-SHA128s (see above) and asserts true
    VerifyAssertSlhDsaSha128s = 67,

    // 68-79: more space for signing/verifying algorithms

    // Encryption / Decryption
    /// Encrypt/decrypt using XChaCha20 stream cipher (takes key and data from stack, puts result back)
    KeyStreamXChaCha20 = 80,
    /// Encrypt/decrypt using AES-256 in CTR mode (takes key and data from stack, puts result back)
    KeyStreamAes256 = 81,
    // 82-89: more space for encryption/decryption algorithms
}

/* Example script

[16]            PUSHINT 16
[16, 2]         PUSHINT 2
[32]            MUL
[32, 32]        PUSHINT 32
[true]          EQ
[]              ASSERT

*/

/// Settings for the script engine, such as limits and allowed operations.
///
/// # Properties
/// - `memory_limit`: The maximum memory usage of the script, in memory points
/// - `executions_limit`: The maximum number of opcode executions allowed for the script
/// - `opcode_limit`: The maximum number of opcodes allowed in the script
/// - `search_limit`: The maximum number of cursor movements allowed in the script (e.g., for jumps and loops)
/// - `max_slice_size`: The maximum size of byte slices that can be created or manipulated by the script
/// - `max_stack_items`: The maximum number of items allowed on the stack
/// - `max_script_len`: The maximum length of the script in bytes
/// - `max_nesting_depth`: The maximum depth of nested control flow structures (e.g., IFs and loops)
/// - `allow_clear`: Whether the CLEAR opcode is allowed in the script
/// - `allow_control_flow`: Whether control flow opcodes (IF, ELSE, FI, LOOP, POOL, JMP) are allowed in the script
/// - `allow_loop`: Whether loop opcodes (LOOP and POOL) are allowed in the script
/// - `allow_jump`: Whether the JMP opcode is allowed in the script
/// - `allow_non_push`: Whether non-push opcodes are allowed in the script (if false, only push opcodes will be allowed)
/// - `allow_eval`: Whether the EVAL opcode is allowed in the script
/// - `allow_sandboxed_eval`: Whether the EVALSUB opcode is allowed in the script
/// - `allow_bucket_actions`: Whether bucket operation opcodes (SERVER, SELECT, READ, WRITE, APPEND, DELETE) are allowed in the script
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

    pub alllow_function_declaration: bool,
    pub allow_function_calls: bool,
    pub allow_external_function_calls: bool,
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
            allow_eval: false,
            allow_sandboxed_eval: false,
            allow_bucket_actions: false,
            alllow_function_declaration: false,
            allow_function_calls: false,
            allow_external_function_calls: false,
        }
    }
}

/// OPCODE script consisting of a list of instructions (opcodes)
#[derive(Debug, Clone, PartialEq, ToBytes, FromBytes, Serialize, Deserialize)]
pub struct OpcodeScript {
    /// Script instructions
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
    // TODO: use this to speed up the interpreter when executing
    // TODO: expand with function calls / subtract addresses from function declarations etc
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
