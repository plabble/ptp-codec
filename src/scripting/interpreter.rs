use std::{cmp, ops::Neg};

use binary_codec::BinaryDeserializer;
use chrono::Utc;

use crate::{
    core::PlabbleDateTime,
    scripting::opcode_script::{Opcode, OpcodeScript, ScriptSettings},
};
use log::{debug, trace};

use super::stack::StackData;

#[derive(Debug, Clone)]
pub struct ScriptInterpreter {
    main_stack: Vec<StackData>,
    alt_stack: Vec<StackData>,
    snapshot: Vec<StackData>,
    snapshot_memory: usize,
    settings: ScriptSettings,

    script: OpcodeScript,
    cursor: usize,
    use_alt_stack: bool,
    executions: usize,
    searches: usize,
    memory: usize,
    memory_peak: usize,
}

#[derive(Debug, PartialEq)]
pub enum ScriptError {
    /// When the stack is empty while n items are required
    StackUnderflow(usize),

    /// When an operation expected a number but got something else
    NotANumber,

    /// When an operation expected a float but got something else
    NotAFloat,

    /// When an operation expected a boolean but got something else
    NotABoolean,

    /// When a mathematical operation fails (e.g., division by zero)
    MathError,

    /// When the script is not valid
    InvalidScript,

    /// When a not-existing address or index is provided
    OutOfBounds,

    /// When an assertion fails
    AssertionFailed,

    ControlFlowMalformed,
    ClearNotAllowed,
    ControlFlowNotAllowed,
    JumpNotAllowed,
    LoopNotAllowed,
    NonPushNotAllowed,
    EvalNotAllowed,
    BucketActionsNotAllowed,
    MaxDepthExceeded,
    SearchLimitExceeded,
    ExecutionLimitExceeded,
    OpcodeLimitExceeded,
    MemoryLimitExceeded,
    SliceLimitExceeded,
    StackHeightLimitExceeded,
}

impl ScriptInterpreter {
    pub fn new(script: OpcodeScript, settings: Option<ScriptSettings>) -> Self {
        ScriptInterpreter {
            main_stack: Vec::new(),
            alt_stack: Vec::new(),
            snapshot: Vec::new(),
            snapshot_memory: 0,

            cursor: 0,
            script,
            use_alt_stack: false,
            executions: 0,
            searches: 0,
            memory: 0,
            memory_peak: 0,
            settings: settings.unwrap_or_default(),
        }
    }

    /// Create a new instance with a clean stack that uses the same memory, searches & executions count
    pub fn fork(&self, subscript: OpcodeScript, settings: Option<ScriptSettings>) -> Self {
        let mut instance = self.clone();
        if let Some(settings) = settings {
            instance.settings = settings;
        }
        instance.script = subscript;

        // Reset stacks, cursor, snapshots
        instance.main_stack.clear();
        instance.alt_stack.clear();
        instance.cursor = 0;
        instance.snapshot.clear();
        instance.snapshot_memory = 0;
        instance.use_alt_stack = false;
        instance
    }

    /// Validate a script against the current settings, without executing it. This is useful to check if a script is valid before forking or executing it.
    pub fn validate_script(&self, script: &OpcodeScript) -> Result<(), ScriptError> {
        if !self.settings.allow_non_push && !script.is_push_only() {
            return Err(ScriptError::NonPushNotAllowed);
        }

        if script.instructions.len() > self.settings.opcode_limit {
            return Err(ScriptError::OpcodeLimitExceeded);
        }

        for instruction in script.instructions.iter() {
            match instruction {
                Opcode::IF | Opcode::ELSE | Opcode::FI | Opcode::BREAK => {
                    if !self.settings.allow_control_flow {
                        return Err(ScriptError::ControlFlowNotAllowed);
                    }
                }
                Opcode::LOOP | Opcode::POOL => {
                    if !self.settings.allow_control_flow {
                        return Err(ScriptError::ControlFlowNotAllowed);
                    }

                    if !self.settings.allow_loop {
                        return Err(ScriptError::LoopNotAllowed);
                    }
                }
                Opcode::JMP => {
                    if !self.settings.allow_control_flow {
                        return Err(ScriptError::ControlFlowNotAllowed);
                    }

                    if !self.settings.allow_loop || !self.settings.allow_jump {
                        return Err(ScriptError::JumpNotAllowed);
                    }
                }
                Opcode::CLEAR => {
                    if !self.settings.allow_clear {
                        return Err(ScriptError::ClearNotAllowed);
                    }
                }
                Opcode::SERVER
                | Opcode::SELECT
                | Opcode::READ
                | Opcode::WRITE
                | Opcode::APPEND
                | Opcode::DELETE => {
                    if !self.settings.allow_bucket_actions {
                        return Err(ScriptError::BucketActionsNotAllowed);
                    }
                }
                Opcode::EVALSUB => {
                    if !self.settings.allow_sandboxed_eval {
                        return Err(ScriptError::EvalNotAllowed);
                    }
                }
                Opcode::EVAL => {
                    if !self.settings.allow_eval {
                        return Err(ScriptError::EvalNotAllowed);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Calculate the total memory used by the current stack, without counting the snapshot (which is not active memory)
    fn calculate_memory(&mut self) -> usize {
        self.stack().iter().map(|i| i.memory()).sum()
    }

    /// Get a mutable reference to the currently active stack (main or alt, depending on the current mode)
    fn stack(&mut self) -> &mut Vec<StackData> {
        if self.use_alt_stack {
            &mut self.alt_stack
        } else {
            &mut self.main_stack
        }
    }

    /// Get a mutable reference to the currently active alt stack (main or alt, depending on the current mode).
    /// This is used for TOALT and FROMALT operations, which move items between stacks, so we need to be able to access the alt stack even when it's not active
    fn alt_stack(&mut self) -> &mut Vec<StackData> {
        if self.use_alt_stack {
            &mut self.main_stack
        } else {
            &mut self.alt_stack
        }
    }

    /// Ensure the stack has at least `size` items, otherwise return a StackUnderflow error with the required size
    fn ensure_stack_size(&mut self, size: usize) -> Result<(), ScriptError> {
        if self.stack().len() < size {
            return Err(ScriptError::StackUnderflow(size - self.stack().len()));
        }
        Ok(())
    }

    /// Pop an item from the currently active stack, and decrease the memory count by the memory used by the popped item. Return None if the stack is empty.
    fn pop(&mut self) -> Option<StackData> {
        let item = self.stack().pop()?;
        self.memory -= item.memory();

        Some(item)
    }

    /// Push an item to the currently active stack, and increase the memory count by the memory used by the pushed item. Return an error if the new memory usage exceeds the limit, or if the new stack height exceeds the limit.
    fn push(&mut self, item: StackData) -> Result<(), ScriptError> {
        let item_memory = item.memory();

        if item_memory > self.settings.max_slice_size {
            return Err(ScriptError::SliceLimitExceeded);
        }

        self.memory += item_memory;
        self.memory_peak = cmp::max(self.memory, self.memory_peak);

        if self.memory > self.settings.memory_limit {
            return Err(ScriptError::MemoryLimitExceeded);
        }

        if self.stack().len() + 1 > self.settings.max_stack_items {
            return Err(ScriptError::StackHeightLimitExceeded);
        }

        self.stack().push(item);

        Ok(())
    }

    /// Pop an item from the stack and try to convert it to a number. Return an error if the stack is empty or if the item cannot be converted to a number.
    fn pop_number(&mut self) -> Result<i128, ScriptError> {
        self.pop()
            .and_then(|n| n.as_number())
            .ok_or(ScriptError::NotANumber)
    }

    /// Pop an item from the stack and try to convert it to a float. Return an error if the stack is empty or if the item cannot be converted to a float.
    fn pop_float(&mut self) -> Result<f64, ScriptError> {
        self.pop()
            .and_then(|n| n.as_float())
            .ok_or(ScriptError::NotAFloat)
    }

    /// Pop an item from the stack and try to convert it to a boolean. Return an error if the stack is empty or if the item cannot be converted to a boolean.
    fn pop_boolean(&mut self) -> Result<bool, ScriptError> {
        self.pop()
            .and_then(|b| b.as_boolean())
            .ok_or(ScriptError::NotABoolean)
    }

    /// Pop two items from the stack and check if they are equal, using the equality rules defined in the function.
    /// Return an error if the stack has less than 2 items, or if the items cannot be compared (e.g., different types that cannot be converted to a common type).
    fn check_equality(&mut self) -> Result<bool, ScriptError> {
        self.ensure_stack_size(2)?;
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();

        match (a, b) {
            (StackData::Boolean(a), StackData::Boolean(b)) => Ok(a == b),
            (StackData::Number(a), StackData::Number(b)) => Ok(a == b),
            (StackData::Float(a), StackData::Float(b)) => Ok(a == b),
            (StackData::Byte(a), StackData::Byte(b)) => Ok(a == b),
            (StackData::Number(a), StackData::Byte(b)) => Ok(a == b as i128),
            (StackData::Byte(a), StackData::Number(b)) => Ok(a as i128 == b),
            (StackData::Float(a), StackData::Number(b)) => Ok(a.fract() == 0.0 && a as i128 == b),
            (StackData::Number(a), StackData::Float(b)) => Ok(b.fract() == 0.0 && a == b as i128),
            (StackData::Float(a), StackData::Byte(b)) => Ok(a == b as f64),
            (StackData::Byte(a), StackData::Float(b)) => Ok(a as f64 == b),
            (StackData::Boolean(a), StackData::Number(b)) => Ok((if a { 1 } else { 0 }) == b),
            (StackData::Number(a), StackData::Boolean(b)) => Ok(a == (if b { 1 } else { 0 })),
            (StackData::Boolean(a), StackData::Float(b)) => Ok((if a { 1f64 } else { 0f64 }) == b),
            (StackData::Float(a), StackData::Boolean(b)) => Ok(a == (if b { 1f64 } else { 0f64 })),
            (StackData::Boolean(a), StackData::Byte(b)) => Ok((if a { 1 } else { 0 }) == b),
            (StackData::Byte(a), StackData::Boolean(b)) => Ok(a == (if b { 1 } else { 0 })),
            (a, b) => {
                let a = a.as_buffer().expect("Failed to convert to buffer");
                let b = b.as_buffer().expect("Failed to convert to buffer");

                Ok(a == b)
            }
        }
    }

    /// Execute the script until completion, and return the final stack as a single buffer (by concatenating all items' buffer representations)
    /// if a RETURN opcode is executed, or None if the script finishes without a RETURN.
    /// Return an error if any opcode execution fails, or if the script is invalid.
    pub fn exec(&mut self) -> Result<Option<Vec<u8>>, ScriptError> {
        self.validate_script(&self.script)?;

        while self.cursor < self.script.instructions.len() {
            let res = self.exec_next()?;
            if res.is_some() {
                return Ok(res);
            }
        }

        Ok(None)
    }

    /// Execute the next opcode in the script, and return the final stack as a single buffer if a RETURN opcode is executed, or None otherwise.
    pub fn exec_next(&mut self) -> Result<Option<Vec<u8>>, ScriptError> {
        if self.cursor >= self.script.instructions.len() {
            return Ok(None);
        }

        let opcode = self.script.instructions[self.cursor].clone();
        self.executions += 1; // Every opcode execution costs 1 CPU cycle
        if self.executions > self.settings.executions_limit {
            return Err(ScriptError::ExecutionLimitExceeded);
        }

        debug!("Executing opcode: {:?}", opcode);

        match opcode {
            Opcode::FALSE => self.push(StackData::Boolean(false))?,
            Opcode::TRUE => self.push(StackData::Boolean(true))?,
            Opcode::PUSH1(data) => self.push(StackData::Byte(data))?,
            Opcode::PUSH2(data) => self.push(StackData::Buffer(data.to_vec()))?,
            Opcode::PUSH4(data) => self.push(StackData::Buffer(data.to_vec()))?,
            Opcode::PUSHL1 { len: _, data } => self.push(StackData::Buffer(data))?,
            Opcode::PUSHL2 { len: _, data } => self.push(StackData::Buffer(data))?,
            Opcode::PUSHL4 { len: _, data } => self.push(StackData::Buffer(data))?,
            Opcode::PUSHINT(val) => self.push(StackData::Number(val))?,
            Opcode::PUSHFLOAT(val) => self.push(StackData::Float(val))?,
            Opcode::ADD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_add(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::SUB => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_sub(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::MUL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_mul(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::DIV => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_div(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::MOD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_rem(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::NEG => {
                self.ensure_stack_size(1)?;
                let val = self.pop();

                match val {
                    Some(StackData::Number(a)) => {
                        let c = a.checked_neg().ok_or(ScriptError::MathError)?;
                        self.push(StackData::Number(c))?;
                    }
                    Some(StackData::Float(a)) => {
                        self.push(StackData::Float(a.neg()))?;
                    }
                    _ => return Err(ScriptError::NotANumber),
                }
            }
            Opcode::ABS => {
                self.ensure_stack_size(1)?;
                let val = self.pop();

                match val {
                    Some(StackData::Number(a)) => {
                        let c = a.checked_abs().ok_or(ScriptError::MathError)?;
                        self.push(StackData::Number(c))?;
                    }
                    Some(StackData::Float(a)) => {
                        self.push(StackData::Float(a.abs()))?;
                    }
                    _ => return Err(ScriptError::NotANumber),
                }
            }
            Opcode::FADD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_float()?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a + b))?;
            }
            Opcode::FSUB => {
                self.ensure_stack_size(2)?;
                let b = self.pop_float()?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a - b))?;
            }
            Opcode::FMUL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_float()?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a * b))?;
            }
            Opcode::FDIV => {
                self.ensure_stack_size(2)?;
                let b = self.pop_float()?;
                let a = self.pop_float()?;
                if b == 0.0 {
                    return Err(ScriptError::MathError);
                }
                self.push(StackData::Float(a / b))?;
            }
            Opcode::FMOD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_float()?;
                let a = self.pop_float()?;
                if b == 0.0 {
                    return Err(ScriptError::MathError);
                }
                self.push(StackData::Float(a % b))?;
            }
            Opcode::LT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b < a))?;
            }
            Opcode::GT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b > a))?;
            }
            Opcode::LTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b <= a))?;
            }
            Opcode::GTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b >= a))?;
            }
            Opcode::MIN => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Number(cmp::min(a, b)))?;
            }
            Opcode::MAX => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Number(cmp::max(a, b)))?;
            }
            Opcode::FLT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Boolean(b < a))?;
            }
            Opcode::FGT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Boolean(b > a))?;
            }
            Opcode::FLTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Boolean(b <= a))?;
            }
            Opcode::FGTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Boolean(b >= a))?;
            }
            Opcode::FMIN => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Float(a.min(b)))?;
            }
            Opcode::FMAX => {
                self.ensure_stack_size(2)?;
                let a = self.pop_float()?;
                let b = self.pop_float()?;
                self.push(StackData::Float(a.max(b)))?;
            }
            Opcode::FLOOR => {
                self.ensure_stack_size(1)?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a.floor()))?;
            }
            Opcode::CEIL => {
                self.ensure_stack_size(1)?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a.ceil()))?;
            }
            Opcode::ROUND => {
                self.ensure_stack_size(1)?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a.round()))?;
            }
            Opcode::ROUNDE => {
                self.ensure_stack_size(1)?;
                let a = self.pop_float()?;
                self.push(StackData::Float(a.round_ties_even()))?;
            }
            Opcode::BAND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a & b;
                self.push(StackData::Number(c))?;
            }
            Opcode::BOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a | b;
                self.push(StackData::Number(c))?;
            }
            Opcode::BXOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a ^ b;
                self.push(StackData::Number(c))?;
            }
            Opcode::BSHL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shl(b as u32).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::BSHR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shr(b as u32).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::BNOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = !a;
                self.push(StackData::Number(c))?;
            }
            Opcode::NOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(!a))?;
            }
            Opcode::AND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a && b))?;
            }
            Opcode::OR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a || b))?;
            }
            Opcode::XOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a ^ b))?;
            }
            Opcode::EQ => {
                let eq = self.check_equality()?;
                self.push(StackData::Boolean(eq))?;
            }
            Opcode::NEQ => {
                let eq = self.check_equality()?;
                self.push(StackData::Boolean(!eq))?;
            }
            Opcode::POW => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b: u32 = self
                    .pop_number()?
                    .try_into()
                    .map_err(|_| ScriptError::MathError)?;
                let c = a.checked_pow(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c))?;
            }
            Opcode::SQRT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_float()?;
                if a < 0.0 {
                    return Err(ScriptError::MathError);
                }
                let c = a.sqrt();
                self.push(StackData::Float(c))?;
            }
            Opcode::NOP => { /* NOP = do nothing */ }
            Opcode::IF => {
                self.ensure_stack_size(1)?;

                let condition = self.pop_boolean()?;
                if !condition {
                    // Search a ELSE or FI to skip to
                    let pos =
                        self.search(Opcode::IF, Opcode::FI, Some(Opcode::ELSE), None, false)?;
                    self.cursor = pos;
                }
            }
            Opcode::ELSE => {
                // a. Make sure a matching IF exists
                self.search(Opcode::IF, Opcode::FI, None, None, true)?;

                // Skip to FI
                let pos = self.search(Opcode::IF, Opcode::FI, None, None, false)?;
                self.cursor = pos;
            }
            Opcode::FI => {
                // Validate there is a matching IF earlier
                self.search(Opcode::IF, Opcode::FI, None, None, true)?;
            }
            Opcode::BREAK => {
                // Ensure there's an enclosing LOOP (search backwards)
                self.search(Opcode::LOOP, Opcode::POOL, None, None, true)?;

                // Skip to next POOL (forward), taking nesting into account
                let pos = self.search(Opcode::LOOP, Opcode::POOL, None, None, false)?;
                self.cursor = pos;
            }
            Opcode::LOOP => {
                // Just continue execution
            }
            Opcode::POOL => {
                // Jump back to the corresponding LOOP
                let pos = self.search(Opcode::LOOP, Opcode::POOL, None, None, true)?;
                self.cursor = pos;
            }
            Opcode::JMP => {
                self.ensure_stack_size(1)?;
                let address = self.pop_number()?;
                if address < 0 || (address as usize) >= self.script.instructions.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                self.searches += address.abs_diff(self.cursor as i128) as usize;
                if self.searches > self.settings.search_limit {
                    return Err(ScriptError::SearchLimitExceeded);
                }

                self.cursor = address as usize;
                return Ok(None); // Skip the cursor increment at the end
            }
            Opcode::ASSERT => {
                self.ensure_stack_size(1)?;
                let condition = self.pop_boolean()?;
                if !condition {
                    return Err(ScriptError::AssertionFailed);
                }
            }
            Opcode::RETURN => {
                let mut stack_data = Vec::new();
                for item in self.stack().drain(..) {
                    let buffer = item.as_buffer().expect("Failed to convert to buffer");
                    stack_data.extend_from_slice(&buffer);
                }
                return Ok(Some(stack_data));
            }
            Opcode::DUP => {
                self.ensure_stack_size(1)?;
                let top = self.stack().last().unwrap().clone();
                self.push(top)?;
            }
            Opcode::DUP2 => {
                self.ensure_stack_size(2)?;
                let len = self.stack().len();
                let first = self.stack()[len - 2].clone();
                let second = self.stack()[len - 1].clone();
                self.push(first)?;
                self.push(second)?;
            }
            Opcode::DUP3 => {
                self.ensure_stack_size(3)?;
                let len = self.stack().len();
                let first = self.stack()[len - 3].clone();
                let second = self.stack()[len - 2].clone();
                let third = self.stack()[len - 1].clone();
                self.push(first)?;
                self.push(second)?;
                self.push(third)?;
            }
            Opcode::DUP4 => {
                self.ensure_stack_size(4)?;
                let len = self.stack().len();
                let first = self.stack()[len - 4].clone();
                let second = self.stack()[len - 3].clone();
                let third = self.stack()[len - 2].clone();
                let fourth = self.stack()[len - 1].clone();
                self.push(first)?;
                self.push(second)?;
                self.push(third)?;
                self.push(fourth)?;
            }
            Opcode::DUPN(n) => {
                self.ensure_stack_size(1)?;
                let top = self.stack().last().unwrap().clone();
                for _ in 0..n {
                    self.push(top.clone())?;
                }
            }
            Opcode::SWAP => {
                self.ensure_stack_size(2)?;
                let len = self.stack().len();
                self.stack().swap(len - 1, len - 2);
            }
            Opcode::ROT => {
                self.ensure_stack_size(3)?;
                let len = self.stack().len();
                self.stack()[len - 3..].rotate_left(1);
            }
            Opcode::POP => {
                self.ensure_stack_size(1)?;
                self.pop();
            }
            Opcode::COPY => {
                self.ensure_stack_size(1)?;
                let n = self.pop_number()?;
                if n < 0 || n as usize >= self.stack().len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let item = self.stack()[n as usize].clone();
                self.push(item)?;
            }
            Opcode::BUBBLE => {
                self.ensure_stack_size(1)?;
                let n = self.pop_number()?;
                if n < 0 || n as usize >= self.stack().len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let item = self.stack().remove(n as usize);
                // Bubble moves, so we don't use self.push because we don't want to increment memory
                self.stack().push(item);
            }
            Opcode::SINK => {
                self.ensure_stack_size(1)?;
                let n = self.pop_number()?;
                if n < 0 || n as usize >= self.stack().len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let item = self.stack().remove(n as usize);
                self.stack().insert(0, item);
            }
            Opcode::TOALT => {
                if self.stack().is_empty() {
                    return Err(ScriptError::StackUnderflow(1));
                }

                // To alt stack is a move operation, so we don't use self.pop() because we don't want to increment/decrement memory
                let item = self.stack().pop().unwrap();
                self.alt_stack().push(item);
            }
            Opcode::FROMALT => {
                if self.alt_stack().is_empty() {
                    return Err(ScriptError::StackUnderflow(1));
                }

                // From alt stack is a move operation, so we don't use self.pop() because we don't want to increment/decrement memory
                let item = self.alt_stack().pop().unwrap();
                self.stack().push(item);
            }
            Opcode::SNAPSHOT => {
                self.snapshot_memory = self.calculate_memory();
                self.snapshot = self.stack().clone();
            }
            Opcode::RESTORE => {
                let snapshot = self.snapshot.clone();
                self.memory -= self.calculate_memory();
                self.memory += self.snapshot_memory;
                self.snapshot_memory = 0;
                self.snapshot.clear();

                // Restore into the currently active stack (allow restoring alt snapshot into main and vice-versa)
                self.stack().clear();
                self.stack().extend_from_slice(&snapshot);
            }
            Opcode::CLEAR => {
                self.memory -= self.calculate_memory();
                self.stack().clear();
            }
            Opcode::SWITCH => {
                self.use_alt_stack = !self.use_alt_stack;
            }
            Opcode::CONCAT => {
                self.ensure_stack_size(2)?;
                let b = self.pop().unwrap();
                let a = self.pop().unwrap();

                let a_bytes = a.as_buffer().expect("Failed to convert to buffer");
                let b_bytes = b.as_buffer().expect("Failed to convert to buffer");

                let mut combined = a_bytes;
                combined.extend_from_slice(&b_bytes);

                self.push(StackData::Buffer(combined))?;
            }
            Opcode::COUNT => {
                let length = self.stack().len() as i128;
                self.push(StackData::Number(length))?;
            }
            Opcode::NUMBER => {
                self.ensure_stack_size(1)?;
                let num = self.pop_number()?;
                self.push(StackData::Number(num))?;
            }
            Opcode::FLOAT => {
                self.ensure_stack_size(1)?;
                let num = self.pop_float()?;
                self.push(StackData::Float(num))?;
            }
            Opcode::SERVER => todo!(),
            Opcode::SELECT => todo!(),
            Opcode::READ => todo!(),
            Opcode::WRITE => todo!(),
            Opcode::APPEND => todo!(),
            Opcode::DELETE => todo!(),
            Opcode::LEN => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");
                let length = bytes.len() as i128;
                self.push(StackData::Number(length))?;
            }
            Opcode::REVERSE => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let mut bytes = item.as_buffer().expect("Failed to convert to buffer");
                bytes.reverse();
                self.push(StackData::Buffer(bytes))?;
            }
            Opcode::SLICE => {
                self.ensure_stack_size(3)?;
                let length = self.pop_number()?;
                let offset = self.pop_number()?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                if offset < 0 || length < 0 || (offset as usize) + (length as usize) > bytes.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let slice = bytes
                    .get(offset as usize..(offset as usize) + (length as usize))
                    .unwrap()
                    .to_vec();

                self.push(StackData::Buffer(slice))?;
            }
            Opcode::SPLICE => {
                self.ensure_stack_size(3)?;
                let offset = self.pop_number()?;
                let length = self.pop_number()?;
                let item = self.pop().unwrap();
                let splice_data = item.as_buffer().expect("Failed to convert to buffer");
                let item = self.pop().unwrap();
                let mut bytes = item.as_buffer().expect("Failed to convert to buffer");

                if offset < 0 || length < 0 || (offset as usize) + (length as usize) > bytes.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                bytes.splice((offset as usize)..((offset + length) as usize), splice_data);
                self.push(StackData::Buffer(bytes))?;
            }
            Opcode::HASH => todo!(),
            Opcode::SIGN => todo!(),
            Opcode::VERIFY => todo!(),
            Opcode::ENCRYPT => todo!(),
            Opcode::DECRYPT => todo!(),
            Opcode::TIME => {
                let now = PlabbleDateTime(Utc::now());
                self.push(StackData::Number(now.timestamp() as i128))?;
            }
            Opcode::EVALSUB => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                let config: Option<&mut binary_codec::SerializerConfig> = None;
                let script = OpcodeScript::from_bytes(&bytes, config)
                    .map_err(|_| ScriptError::InvalidScript)?;

                self.validate_script(&script)?;

                if script.instructions.len() + self.script.instructions.len()
                    > self.settings.opcode_limit
                {
                    return Err(ScriptError::OpcodeLimitExceeded);
                }

                let sub_settings = ScriptSettings {
                    allow_eval: false,
                    allow_sandboxed_eval: false,
                    ..self.settings
                };

                // We want a child process, but have the same memory/search/execution limits
                let mut sub_interpreter = self.fork(script, Some(sub_settings));
                let result = sub_interpreter.exec()?;

                if let Some(result_bytes) = result {
                    self.push(StackData::Buffer(result_bytes))?;
                }

                self.executions = sub_interpreter.executions;
                self.searches = sub_interpreter.searches;
                self.memory = sub_interpreter.memory;
                self.memory_peak = sub_interpreter.memory_peak;
            }
            Opcode::EVAL => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                let config: Option<&mut binary_codec::SerializerConfig> = None;
                let script = OpcodeScript::from_bytes(&bytes, config)
                    .map_err(|_| ScriptError::InvalidScript)?;

                self.validate_script(&script)?;

                if script.instructions.len() + self.script.instructions.len()
                    > self.settings.opcode_limit
                {
                    return Err(ScriptError::OpcodeLimitExceeded);
                }

                // Insert new script instructions at current position
                self.script
                    .instructions
                    .splice(self.cursor + 1..self.cursor + 1, script.instructions);
            }
        };

        self.cursor += 1;

        Ok(None)
    }

    /// Search for a matching opCode and return the cursor difference (negative or positive value)
    fn search(
        &mut self,
        open: Opcode,
        close: Opcode,
        or: Option<Opcode>,
        stop: Option<Opcode>,
        backwards: bool,
    ) -> Result<usize, ScriptError> {
        debug!(
            "Search for {:?}--{:?} (or: {:?}, stop: {:?}). Reverse: {}",
            open, close, or, stop, backwards
        );

        let step: isize = if backwards { -1 } else { 1 };

        trace!(
            "Start search {} at {:?}",
            self.cursor, self.script.instructions[self.cursor]
        );

        let mut cursor = self.cursor as isize + step;
        self.searches += 1;

        if self.searches > self.settings.search_limit {
            return Err(ScriptError::SearchLimitExceeded);
        }

        let mut depth = 0isize;

        let len = self.script.instructions.len() as isize;
        while cursor >= 0 && cursor < len {
            let code = &self.script.instructions[cursor as usize];
            trace!("Search cursor {} at {:?}", cursor, code);

            if *code == open {
                if backwards {
                    if depth == 0 {
                        return Ok(cursor as usize);
                    } else {
                        depth -= 1;
                    }
                } else {
                    depth += 1;
                }
            } else if *code == close {
                if backwards {
                    depth += 1;
                } else if depth == 0 {
                    return Ok(cursor as usize);
                } else {
                    depth -= 1;
                }
            } else if let Some(ref or) = or
                && or == code
            {
                if depth == 0 {
                    return Ok(cursor as usize);
                }
            } else if let Some(ref stopcode) = stop
                && stopcode == code
            {
                return Ok(cursor as usize);
            }

            cursor += step;
            self.searches += 1;

            if depth.unsigned_abs() > self.settings.max_nesting_depth {
                return Err(ScriptError::MaxDepthExceeded);
            }
        }

        Err(ScriptError::ControlFlowMalformed)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use binary_codec::{BinarySerializer, SerializerConfig};

    use crate::scripting::{
        interpreter::ScriptInterpreter,
        opcode_script::{Opcode, OpcodeScript, ScriptSettings},
        stack::StackData,
    };

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            env_logger::Builder::new()
                .is_test(true) // important for tests
                .filter_level(log::LevelFilter::Trace)
                .init();
        });
    }

    #[test]
    fn can_create_and_break_a_loop() {
        // Generate [1,2,3]
        let script = OpcodeScript::new(vec![
            //                      CPU         | Cursor CPU     | Memory
            Opcode::LOOP,       // 1               12    24 29
            Opcode::COUNT,      // 2 10 18         11    23 28     +2
            Opcode::PUSHINT(2), // 3 11 19         10    22 27     +2
            Opcode::EQ,         // 4 12 20         9     21 26     -4, +1
            Opcode::IF,         // 5 13 21       * 8  *  20 25     -1
            Opcode::BREAK,      //      22       1 7  13 19 *  34
            Opcode::FI,         //               2 6  14 18    33
            Opcode::COUNT,      // 6 14            5     17    32  +2
            Opcode::PUSHINT(1), // 7 15            4     16    31  +2
            Opcode::ADD,        // 8 16            3     15    30  -4, +2
            Opcode::POOL,       // 9 17          *       *     *
            Opcode::PUSHINT(3), //      23                         +2
        ]);

        let mut interpreter = ScriptInterpreter::new(script, None);
        interpreter.exec().unwrap();

        assert_eq!(23, interpreter.executions);
        assert_eq!(34, interpreter.searches);
        assert_eq!(6, interpreter.memory);
        assert_eq!(8, interpreter.memory_peak);
    }

    #[test]
    fn can_do_simple_math() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(16),
            Opcode::PUSHINT(2),
            Opcode::MUL,
            Opcode::PUSHINT(32),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(10),
            Opcode::PUSHINT(5),
            Opcode::SUB,
            Opcode::PUSHINT(5),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut interpreter = ScriptInterpreter::new(script, None);
        let result = interpreter.exec();

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn can_do_eval() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(10),
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0x02, 5, 0x02, 2, 0xC],
            },
            Opcode::EVAL,
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut settings = ScriptSettings::default();
        settings.allow_eval = true;

        let mut interpreter = ScriptInterpreter::new(script, Some(settings));
        let result = interpreter.exec();

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn can_do_slice_operations() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 7, 8, 9],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![4, 5, 6],
            },
            Opcode::PUSHINT(0),
            Opcode::PUSHINT(3),
            Opcode::SPLICE,
        ]);

        let mut interpreter = ScriptInterpreter::new(script, None);
        let result = interpreter.exec();
        assert_eq!(result, Ok(None));

        assert_eq!(
            &interpreter.main_stack[..],
            &[StackData::Buffer(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])][..]
        );
    }

    #[test]
    fn arithmetic_and_bitwise_ops() {
        let script = OpcodeScript::new(vec![
            // ((20 / 5) * 3) + 8 == 20
            Opcode::PUSHINT(20),
            Opcode::PUSHINT(5),
            Opcode::DIV,
            Opcode::PUSHINT(3),
            Opcode::MUL,
            Opcode::PUSHINT(8),
            Opcode::ADD,
            Opcode::PUSHINT(20),
            Opcode::EQ,
            Opcode::ASSERT,
            // 7 % 3 == 1
            Opcode::PUSHINT(7),
            Opcode::PUSHINT(3),
            Opcode::MOD,
            Opcode::PUSHINT(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // neg / abs
            Opcode::PUSHINT(-5),
            Opcode::NEG,
            Opcode::PUSHINT(5),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(-3),
            Opcode::ABS,
            Opcode::PUSHINT(3),
            Opcode::EQ,
            Opcode::ASSERT,
            // bitwise: 6 & 3 == 2, 6 | 3 == 7, 6 ^ 3 == 5
            Opcode::PUSHINT(6),
            Opcode::PUSHINT(3),
            Opcode::BAND,
            Opcode::PUSHINT(2),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(6),
            Opcode::PUSHINT(3),
            Opcode::BOR,
            Opcode::PUSHINT(7),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(6),
            Opcode::PUSHINT(3),
            Opcode::BXOR,
            Opcode::PUSHINT(5),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(None));
    }

    #[test]
    fn boolean_and_equality_ops() {
        let script = OpcodeScript::new(vec![
            // boolean ops
            Opcode::TRUE,
            Opcode::FALSE,
            Opcode::XOR,
            Opcode::ASSERT,
            // number vs byte equality
            Opcode::PUSHINT(1),
            Opcode::PUSH1(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // buffer equality
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(None));
    }

    #[test]
    fn stack_dup_and_basic_manipulation() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::DUP,  // -> 1,2,3,3
            Opcode::DUP2, // -> 1,2,3,3,3,3? (duplicates the two topmost: 3,3)
            Opcode::POP,
            Opcode::POP,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(None));

        // final stack should be [1,2,3,3]
        assert_eq!(
            &i.main_stack[..],
            &[
                StackData::Number(1),
                StackData::Number(2),
                StackData::Number(3),
                StackData::Number(3)
            ][..]
        );
    }

    #[test]
    fn jmp_and_return() {
        // push an address (2), JMP to index 2 which will push a buffer and RETURN
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(3),
            Opcode::JMP,
            Opcode::NOP,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![9],
            },
            Opcode::RETURN,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![9])));
        // Make sure NOP is skipped
        assert_eq!(i.executions, 4);
    }

    #[test]
    fn buffer_length_reverse_slice_splice_concat() {
        let script = OpcodeScript::new(vec![
            // LEN
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::LEN,
            Opcode::PUSHINT(3),
            Opcode::EQ,
            Opcode::ASSERT,
            // REVERSE: verify reversing [4,5,6] -> [6,5,4]
            Opcode::PUSHL1 {
                len: 0,
                data: vec![4, 5, 6],
            },
            Opcode::DUP,
            Opcode::REVERSE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![6, 5, 4],
            },
            Opcode::EQ,
            Opcode::ASSERT,
            // SLICE: take [1,2,3,7,8,9] slice offset 3 length 3 -> [7,8,9]
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 7, 8, 9],
            },
            Opcode::PUSHINT(3), // offset
            Opcode::PUSHINT(3), // length
            Opcode::SLICE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![7, 8, 9],
            },
            Opcode::EQ,
            Opcode::ASSERT,
            // SPLICE: replace bytes at offset 3 length 3 in [1,2,3,7,8,9] with [4,5,6]
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 7, 8, 9],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![4, 5, 6],
            },
            Opcode::PUSHINT(3), // length
            Opcode::PUSHINT(3), // offset
            Opcode::SPLICE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 4, 5, 6],
            },
            Opcode::EQ,
            Opcode::ASSERT,
            // CONCAT: combine [1,2,3] and [4,5,6] -> [1,2,3,4,5,6]
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![4, 5, 6],
            },
            Opcode::CONCAT,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 4, 5, 6],
            },
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i2 = ScriptInterpreter::new(script, None);
        let r2 = i2.exec();
        assert_eq!(r2, Ok(None));
        // final stack only contains [4,5,6] because of the DUP before REVERSE
        assert_eq!(StackData::Buffer(vec![4, 5, 6]), i2.main_stack[0]);
    }

    #[test]
    fn comparisons_and_minmax() {
        let script = OpcodeScript::new(vec![
            // LT: 2 < 3
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::LT,
            Opcode::ASSERT,
            // GT: 3 > 2
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(2),
            Opcode::GT,
            Opcode::ASSERT,
            // LTE / GTE equal case
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(3),
            Opcode::LTE,
            Opcode::ASSERT,
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(3),
            Opcode::GTE,
            Opcode::ASSERT,
            // MIN / MAX
            Opcode::PUSHINT(5),
            Opcode::PUSHINT(7),
            Opcode::MIN,
            Opcode::PUSHINT(5),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(5),
            Opcode::PUSHINT(7),
            Opcode::MAX,
            Opcode::PUSHINT(7),
            Opcode::EQ,
            Opcode::ASSERT,
            // NEQ
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::NEQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn shifts_pow_sqrt_and_not() {
        let script = OpcodeScript::new(vec![
            // BSHL: 1 << 3 == 8
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(3),
            Opcode::BSHL,
            Opcode::PUSHINT(8),
            Opcode::EQ,
            Opcode::ASSERT,
            // BSHR: 8 >> 3 == 1
            Opcode::PUSHINT(8),
            Opcode::PUSHINT(3),
            Opcode::BSHR,
            Opcode::PUSHINT(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // BNOT twice returns original
            Opcode::PUSHINT(1),
            Opcode::BNOT,
            Opcode::BNOT,
            Opcode::PUSHINT(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // POW: 2^3 = 8 (push exponent then base per implementation)
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(2),
            Opcode::POW,
            Opcode::PUSHINT(8),
            Opcode::EQ,
            Opcode::ASSERT,
            // SQRT: sqrt(9) == 3
            Opcode::PUSHINT(9),
            Opcode::SQRT,
            Opcode::PUSHINT(3),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn logic_and_stack_manipulation_ops() {
        let script = OpcodeScript::new(vec![
            // AND / OR / NOT
            Opcode::TRUE,
            Opcode::TRUE,
            Opcode::AND,
            Opcode::ASSERT,
            Opcode::TRUE,
            Opcode::FALSE,
            Opcode::OR,
            Opcode::ASSERT,
            Opcode::FALSE,
            Opcode::NOT,
            Opcode::ASSERT,
            // DUPN: duplicate top value 2 times -> top becomes three copies
            Opcode::PUSHINT(7),
            Opcode::DUPN(2),
            // now stack top three are 7's; pop two and check remaining top
            Opcode::POP,
            Opcode::POP,
            Opcode::PUSHINT(7),
            Opcode::EQ,
            Opcode::ASSERT,
            // SWAP
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::SWAP,
            Opcode::PUSHINT(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // ROT: rotate [a,b,c] -> [b,c,a]
            Opcode::PUSHINT(10),
            Opcode::PUSHINT(11),
            Opcode::PUSHINT(12),
            Opcode::ROT,
            // top should now be 10
            Opcode::PUSHINT(10),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn copy_bubble_sink_and_alt_snapshot() {
        let script = OpcodeScript::new(vec![
            // COPY: [1,2,3] copy index 0 -> push 1
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(0),
            Opcode::COPY,
            Opcode::PUSHINT(1),
            Opcode::EQ,
            Opcode::ASSERT,
            // BUBBLE: clear then [1,2,3,4], bubble index 1 -> moves 2 to top
            Opcode::CLEAR,
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(4),
            Opcode::PUSHINT(1),
            Opcode::BUBBLE,
            // top should be 2
            Opcode::PUSHINT(2),
            Opcode::EQ,
            Opcode::ASSERT,
            // SINK: clear then [1,2,3,4], sink index 1 -> moves 2 to bottom
            Opcode::CLEAR,
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(4),
            Opcode::PUSHINT(1),
            Opcode::SINK,
            // COUNT should be 4
            Opcode::COUNT,
            Opcode::PUSHINT(4),
            Opcode::EQ,
            Opcode::ASSERT,
            // TOALT/FROMALT
            Opcode::PUSHINT(42),
            Opcode::TOALT,
            Opcode::PUSHINT(1),
            Opcode::FROMALT,
            Opcode::ADD,
            Opcode::PUSHINT(43),
            Opcode::EQ,
            Opcode::ASSERT,
            // SNAPSHOT / RESTORE (operate on a fresh stack)
            Opcode::CLEAR,
            Opcode::PUSHINT(5),
            Opcode::PUSHINT(6),
            Opcode::SNAPSHOT,
            Opcode::PUSHINT(7),
            Opcode::RESTORE,
            // after restore (now replaces stack), snapshot was [5,6]
            // expected COUNT == 2
            Opcode::COUNT,
            Opcode::PUSHINT(2),
            Opcode::EQ,
            Opcode::ASSERT,
            // CLEAR then COUNT == 0
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::CLEAR,
            Opcode::COUNT,
            Opcode::PUSHINT(0),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn push_variants_and_dup3_dup4() {
        let script = OpcodeScript::new(vec![
            // PUSH2 and PUSH4 & PUSHL2/PUSHL4
            Opcode::PUSH2([1, 2]),
            Opcode::LEN,
            Opcode::PUSHINT(2),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSH4([1, 2, 3, 4]),
            Opcode::LEN,
            Opcode::PUSHINT(4),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHL2 {
                len: 0,
                data: vec![9, 8, 7],
            },
            Opcode::LEN,
            Opcode::PUSHINT(3),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHL4 {
                len: 0,
                data: vec![6, 5, 4, 3, 2],
            },
            Opcode::LEN,
            Opcode::PUSHINT(5),
            Opcode::EQ,
            Opcode::ASSERT,
            // DUP3 / DUP4
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::DUP3,
            Opcode::COUNT,
            Opcode::PUSHINT(6),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHINT(4),
            Opcode::DUP4,
            Opcode::COUNT,
            Opcode::PUSHINT(11),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn switch_and_alt_stack_behavior() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // main stack
            Opcode::SWITCH,
            Opcode::PUSHINT(2), // alt stack now
            Opcode::SWITCH,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        // verify values landed in correct stacks
        assert_eq!(i.main_stack[0], StackData::Number(1));
        assert_eq!(i.alt_stack[0], StackData::Number(2));
    }

    #[test]
    fn evalsub_executes_child_script_and_pushes_result() {
        // child script: PUSH1(9); RETURN
        let child_bytes = vec![0x02, 9, 0x4F]; // PUSH1, 9, RETURN

        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: child_bytes,
            },
            Opcode::EVALSUB,
        ]);

        let mut settings = ScriptSettings::default();
        settings.allow_sandboxed_eval = true;

        let mut i = ScriptInterpreter::new(script, Some(settings));
        let r = i.exec();
        assert_eq!(r, Ok(None));
        // child result should be pushed as a buffer [9]
        assert_eq!(i.main_stack[0], StackData::Buffer(vec![9]));
    }

    use super::ScriptError;

    #[test]
    fn if_with_non_boolean_errors() {
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(2), Opcode::IF]);
        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::NotABoolean);
    }

    #[test]
    fn unterminated_if_returns_invalid_script() {
        let script = OpcodeScript::new(vec![Opcode::FALSE, Opcode::IF]);
        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::ControlFlowMalformed);
    }

    #[test]
    fn else_branch_executes_when_if_false() {
        let script = OpcodeScript::new(vec![
            Opcode::FALSE,
            Opcode::IF,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1],
            },
            Opcode::ELSE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![9],
            },
            Opcode::FI,
            Opcode::RETURN,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![9])));
    }

    #[test]
    fn break_outside_loop_is_error() {
        let script = OpcodeScript::new(vec![Opcode::BREAK, Opcode::POOL]);
        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::ControlFlowMalformed);
    }

    #[test]
    fn pool_without_loop_is_error() {
        let script = OpcodeScript::new(vec![Opcode::POOL]);
        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::ControlFlowMalformed);
    }

    #[test]
    fn toalt_and_fromalt_underflow() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::TOALT]), None);
        let err = i.exec().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow(_)));

        let mut i2 = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::FROMALT]), None);
        let err2 = i2.exec().unwrap_err();
        assert!(matches!(err2, ScriptError::StackUnderflow(_)));
    }

    #[test]
    fn switch_nested_behavior() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // main
            Opcode::SWITCH,
            Opcode::PUSHINT(2), // alt
            Opcode::SWITCH,
            Opcode::PUSHINT(3), // main
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(
            i.main_stack,
            vec![StackData::Number(1), StackData::Number(3)]
        );
        assert_eq!(i.alt_stack, vec![StackData::Number(2)]);
    }

    #[test]
    fn jmp_invalid_addresses_error() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHINT(-1), Opcode::JMP]),
            None,
        );
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::OutOfBounds);

        let mut i2 = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHINT(9999), Opcode::JMP]),
            None,
        );
        let err2 = i2.exec().unwrap_err();
        assert_eq!(err2, ScriptError::OutOfBounds);
    }

    #[test]
    fn dupn_zero_noop() {
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(7), Opcode::DUPN(0), Opcode::COUNT]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(i.main_stack.last(), Some(&StackData::Number(1)));
    }

    #[test]
    fn snapshot_restore_replaces_stack() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(5),
            Opcode::SNAPSHOT,
            Opcode::PUSHINT(6),
            Opcode::RESTORE,
            Opcode::COUNT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        // after restore, snapshot (5) should remain and COUNT pushed 1
        assert_eq!(i.main_stack[0], StackData::Number(5));
        assert_eq!(i.main_stack[1], StackData::Number(1));
    }

    #[test]
    fn snapshot_taken_on_main_restores_to_main_even_if_switched() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(11),
            Opcode::SNAPSHOT,
            Opcode::SWITCH,
            Opcode::PUSHINT(22),
            Opcode::RESTORE,
            Opcode::COUNT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        // main stack should contain snapshot value 11, alt stack should be replaced by snapshot and then COUNT pushed
        assert_eq!(i.main_stack, vec![StackData::Number(11)]);
        assert_eq!(i.alt_stack.len(), 2);
        assert_eq!(i.alt_stack[0], StackData::Number(11));
    }

    #[test]
    fn nested_if_and_depth_handling() {
        // outer true, inner false -> inner ELSE branch executes
        let script = OpcodeScript::new(vec![
            Opcode::TRUE,
            Opcode::IF,
            Opcode::FALSE,
            Opcode::IF,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1],
            },
            Opcode::ELSE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![2],
            },
            Opcode::FI,
            Opcode::FI,
            Opcode::RETURN,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec().unwrap();
        assert_eq!(r, Some(vec![2]));
    }

    #[test]
    fn else_without_if_is_error() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::ELSE]), None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::ControlFlowMalformed);
    }

    #[test]
    fn fi_without_if_is_error() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::FI]), None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::ControlFlowMalformed);
    }

    #[test]
    fn if_inside_loop_and_break_in_nested_if() {
        let script = OpcodeScript::new(vec![
            Opcode::LOOP,
            Opcode::TRUE,
            Opcode::IF,
            Opcode::BREAK,
            Opcode::FI,
            Opcode::POOL,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![99],
            },
            Opcode::RETURN,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec().unwrap();
        assert_eq!(r, Some(vec![99]));
    }

    #[test]
    fn jmp_into_if_causes_stack_underflow() {
        // target IF at index 3
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(3),
            Opcode::JMP,
            Opcode::NOP,
            Opcode::IF,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1],
            },
            Opcode::FI,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow(_)));
    }

    #[test]
    fn jmp_to_last_instruction() {
        // last instruction is NOP
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(2), Opcode::JMP, Opcode::NOP]);
        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(None));
    }

    #[test]
    fn evalsub_child_assert_failure_propagates() {
        // child: FALSE, ASSERT -> ASSERT will fail
        let child_bytes = vec![0u8, 78u8]; // FALSE = 0, ASSERT = 78
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: child_bytes,
            },
            Opcode::EVALSUB,
        ]);

        let mut settings = ScriptSettings::default();
        settings.allow_sandboxed_eval = true;

        let mut i = ScriptInterpreter::new(script, Some(settings));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::AssertionFailed);
    }

    #[test]
    fn evalsub_child_without_return_pushes_nothing() {
        let child_bytes = vec![0x02u8, 5u8]; // PUSH1 5 (no RETURN)
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: child_bytes,
            },
            Opcode::EVALSUB,
            Opcode::COUNT,
        ]);

        let mut settings = ScriptSettings::default();
        settings.allow_sandboxed_eval = true;

        let mut i = ScriptInterpreter::new(script, Some(settings));
        assert_eq!(i.exec(), Ok(None));
        // COUNT should be 0 because sub pushed nothing
        assert_eq!(i.main_stack.last(), Some(&StackData::Number(0)));
    }

    #[test]
    fn eq_buffer_vs_number_does_not_panic() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1],
            },
            Opcode::PUSHINT(1),
            Opcode::EQ,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        // should not panic and should produce a boolean
        assert_eq!(i.exec(), Ok(None));
        match i.main_stack.last() {
            Some(StackData::Boolean(_)) => {}
            other => panic!("expected boolean, got {:?}", other),
        }
    }

    #[test]
    fn nop_does_nothing_and_counts_cpu() {
        let script = OpcodeScript::new(vec![
            Opcode::NOP,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![7],
            },
            Opcode::RETURN,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![7])));
        // three opcodes executed
        assert_eq!(i.executions, 3);
    }

    #[test]
    fn count_alt_and_after_switch() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // main
            Opcode::SWITCH,
            Opcode::PUSHINT(2), // alt
            Opcode::COUNT,      // count alt -> 1
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(i.main_stack.len(), 1); // main still has 1
        // alt_stack contains the pushed value and the COUNT result number
        assert_eq!(i.alt_stack.len(), 2);
        // top of main stack is original 1
        assert_eq!(i.main_stack[0], StackData::Number(1));
        // COUNT pushed Number(1) onto alt stack
        assert_eq!(i.alt_stack.last(), Some(&StackData::Number(1)));
    }

    #[test]
    fn count_after_restore_with_empty_snapshot() {
        let script = OpcodeScript::new(vec![
            Opcode::SNAPSHOT,
            Opcode::PUSHINT(1),
            Opcode::RESTORE,
            Opcode::COUNT,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        // snapshot was empty, restore yields empty stack, COUNT pushed 0
        assert_eq!(i.main_stack.last(), Some(&StackData::Number(0)));
    }

    #[test]
    fn div_mod_by_zero_errors() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHINT(1), Opcode::PUSHINT(0), Opcode::DIV]),
            None,
        );
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::MathError);

        let mut i2 = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHINT(1), Opcode::PUSHINT(0), Opcode::MOD]),
            None,
        );
        let err2 = i2.exec().unwrap_err();
        assert_eq!(err2, ScriptError::MathError);
    }

    #[test]
    fn boolean_ops_with_non_boolean_error() {
        // use PUSHINT(2) which is not a valid boolean (only 0/1 allowed)
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHINT(2), Opcode::TRUE, Opcode::AND]),
            None,
        );
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::NotABoolean);
    }

    #[test]
    fn return_on_alt_stack_and_empty_stack() {
        // return with empty stack
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::RETURN]), None);
        let r = i.exec().unwrap();
        assert_eq!(r, Some(vec![]));

        // return on alt stack
        let script = OpcodeScript::new(vec![
            Opcode::SWITCH,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![9],
            },
            Opcode::RETURN,
        ]);
        let mut i2 = ScriptInterpreter::new(script, None);
        let r2 = i2.exec().unwrap();
        assert_eq!(r2, Some(vec![9]));
    }

    #[test]
    fn dupn_on_alt_stack_duplicates_top() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::SWITCH,
            Opcode::PUSHINT(2),
            Opcode::DUPN(3),
            Opcode::COUNT,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        // alt stack should have 1 original + 3 duplicates = 4 items, plus COUNT value -> 5
        assert_eq!(i.alt_stack.len(), 5);
        // last value is COUNT result == 4
        assert_eq!(i.alt_stack.last(), Some(&StackData::Number(4)));
    }

    #[test]
    fn copy_bubble_sink_oob_and_alt_stack() {
        // COPY out of bounds (index == len)
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::PUSHINT(3),
            Opcode::COPY,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::OutOfBounds);

        // COPY negative index
        let script2 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(-1),
            Opcode::COPY,
        ]);
        let mut i2 = ScriptInterpreter::new(script2, None);
        let err2 = i2.exec().unwrap_err();
        assert_eq!(err2, ScriptError::OutOfBounds);

        // COPY on alt stack
        let script3 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::SWITCH,
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(0),
            Opcode::COPY,
        ]);
        let mut i3 = ScriptInterpreter::new(script3, None);
        assert_eq!(i3.exec(), Ok(None));
        assert_eq!(
            i3.alt_stack,
            vec![StackData::Number(2), StackData::Number(2)]
        );

        // BUBBLE out of bounds
        let script4 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(5),
            Opcode::BUBBLE,
        ]);
        let mut i4 = ScriptInterpreter::new(script4, None);
        let err4 = i4.exec().unwrap_err();
        assert_eq!(err4, ScriptError::OutOfBounds);

        // SINK negative index
        let script5 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(-1),
            Opcode::SINK,
        ]);
        let mut i5 = ScriptInterpreter::new(script5, None);
        let err5 = i5.exec().unwrap_err();
        assert_eq!(err5, ScriptError::OutOfBounds);
    }

    #[test]
    fn slice_splice_zero_length_and_alt_stack() {
        // zero-length slice -> returns empty buffer
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::PUSHINT(0),
            Opcode::PUSHINT(0),
            Opcode::SLICE,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(i.main_stack.last(), Some(&StackData::Buffer(vec![])));

        // splice insert (length=0) to insert [9] at offset 1 -> [1,9,2,3]
        let script2 = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![9],
            },
            Opcode::PUSHINT(0),
            Opcode::PUSHINT(1),
            Opcode::SPLICE,
        ]);
        let mut i2 = ScriptInterpreter::new(script2, None);
        assert_eq!(i2.exec(), Ok(None));
        assert_eq!(
            i2.main_stack.last(),
            Some(&StackData::Buffer(vec![1, 9, 2, 3]))
        );

        // splice with empty replacement (remove range)
        // Note: SPLICE pops `offset` then `length`, so push `length` then `offset`.
        let script3 = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2, 3, 4],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![],
            },
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::SPLICE,
        ]);
        let mut i3 = ScriptInterpreter::new(script3, None);
        assert_eq!(i3.exec(), Ok(None));
        assert_eq!(
            i3.main_stack.last(),
            Some(&StackData::Buffer(vec![1, 2, 4]))
        );

        // SLICE on alt stack
        let script4 = OpcodeScript::new(vec![
            Opcode::SWITCH,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![7, 8, 9],
            },
            Opcode::PUSHINT(0),
            Opcode::PUSHINT(2),
            Opcode::SLICE,
        ]);
        let mut i4 = ScriptInterpreter::new(script4, None);
        assert_eq!(i4.exec(), Ok(None));
        assert_eq!(i4.alt_stack.last(), Some(&StackData::Buffer(vec![7, 8])));
    }

    #[test]
    fn concat_edge_cases_and_nonbuffer_panic() {
        // concat with empty buffer
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1, 2],
            },
            Opcode::CONCAT,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(i.main_stack.last(), Some(&StackData::Buffer(vec![1, 2])));

        // concat on alt stack
        let script2 = OpcodeScript::new(vec![
            Opcode::SWITCH,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![3],
            },
            Opcode::PUSHL1 {
                len: 0,
                data: vec![4],
            },
            Opcode::CONCAT,
        ]);
        let mut i2 = ScriptInterpreter::new(script2, None);
        assert_eq!(i2.exec(), Ok(None));
        assert_eq!(i2.alt_stack.last(), Some(&StackData::Buffer(vec![3, 4])));

        // non-buffer concat coerces non-buffers to buffers; resulting buffer should end with the
        // second operand's bytes (here: 2)
        let script3 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHL1 {
                len: 0,
                data: vec![2],
            },
            Opcode::CONCAT,
        ]);
        let mut it = ScriptInterpreter::new(script3, None);
        assert_eq!(it.exec(), Ok(None));
        match it.main_stack.last() {
            Some(StackData::Buffer(b)) => assert_eq!(b.last(), Some(&2u8)),
            _ => panic!("expected buffer result from CONCAT"),
        }
    }

    #[test]
    fn eq_boolean_vs_buffer_and_return_after_snapshot_restore() {
        // Boolean vs Buffer equality
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![1],
            },
            Opcode::TRUE,
            Opcode::EQ,
        ]);
        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(i.main_stack.last(), Some(&StackData::Boolean(false)));

        // RETURN after SWITCH + SNAPSHOT: snapshot taken on alt, restored to main, then RETURN
        let script2 = OpcodeScript::new(vec![
            Opcode::PUSHINT(7),
            Opcode::SWITCH,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![9],
            },
            Opcode::SNAPSHOT,
            Opcode::SWITCH,
            Opcode::RESTORE,
            Opcode::RETURN,
        ]);
        let mut i2 = ScriptInterpreter::new(script2, None);
        let r2 = i2.exec().unwrap();
        assert_eq!(r2, Some(vec![9]));

        // RETURN after RESTORE
        let script3 = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![5],
            },
            Opcode::SNAPSHOT,
            Opcode::CLEAR,
            Opcode::RESTORE,
            Opcode::RETURN,
        ]);
        let mut i3 = ScriptInterpreter::new(script3, None);
        let r3 = i3.exec().unwrap();
        assert_eq!(r3, Some(vec![5]));
    }

    #[test]
    fn can_measure_cpu_and_memory_points() {
        // CPU point = 1 OPCODE execution
        // Cursor CPU point = 1 cursor search movement in a IF,FI,LOOP,POOL,JMP
        // Memory point = 1 (a bit pushed on the stack), 2 (a byte pushed on the stack), 2 (a number pushed on the stack)

        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // 2
            Opcode::PUSHINT(2), // 4
            Opcode::PUSHINT(3), // 6
            Opcode::PUSHINT(4), // 8
            Opcode::PUSHINT(5), // 10
            Opcode::PUSHINT(6), // 12
            Opcode::ADD,        // 10
            Opcode::ADD,        // 8
            Opcode::ADD,        // 6
            Opcode::ADD,        // 4
            Opcode::ADD,        // 2
            Opcode::PUSH1(21),  // 4
            Opcode::EQ,         // 1
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0, 0, 0, 0, 0],
            }, // 11
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        i.exec().unwrap();
        assert_eq!(i.memory, 11);
        assert_eq!(i.memory_peak, 12);
        assert_eq!(i.executions, 14);
        assert_eq!(i.searches, 0);
    }

    #[test]
    fn time_opcode_works() {
        // Hard to test, we just verify if it pushes a number to the stack because that's what it should do
        let script = OpcodeScript::new(vec![Opcode::TIME]);

        let mut i = ScriptInterpreter::new(script, None);
        i.exec().unwrap();

        let res = &i.main_stack[0];
        assert!(matches!(res, &StackData::Number(_)));
    }

    #[test]
    fn can_prevent_action_if_that_is_not_allowed() {
        let mut settings = ScriptSettings::default();
        settings.allow_jump = false;
        settings.allow_eval = true;

        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0x00, 0x4D],
            }, // hidden JMP
            Opcode::EVAL,
        ]);

        let mut i = ScriptInterpreter::new(script, Some(settings));
        let res = i.exec();

        assert_eq!(res, Err(ScriptError::JumpNotAllowed));
    }

    // Pseudocode of the script:
    //
    // Script opcodes sequence:
    // PUSHINT(1)    // outer condition true
    // IF
    //   LOOP
    //     PUSHINT(0) // inner condition false
    //     IF
    //       PUSHL1([0xAA])   // inner IF branch (not taken)
    //     ELSE
    //       BREAK            // breaks out to matching POOL
    //     FI
    //     PUSHL1([0xBB])     // executed if loop not broken
    //   POOL
    // ELSE
    //   PUSHL1([0xCC])       // outer ELSE branch (not taken)
    // FI
    // PUSHL1([0xDD])         // final push before RETURN
    //
    // Execution flow:
    // outer IF true -> enter LOOP; inner IF false -> ELSE triggers BREAK -> jump to POOL,
    // then after POOL continue and push 0xBB, then exit outer IF, push 0xDD and RETURN.
    // Expected returned bytes: [0xDD]
    #[test]
    fn complex_nested_if_loop_break() {
        init_logger();

        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // outer true
            Opcode::IF,
            Opcode::LOOP,
            Opcode::PUSHINT(0), // inner false
            Opcode::IF,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0xAA],
            },
            Opcode::ELSE,
            Opcode::BREAK,
            Opcode::FI,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0xBB],
            },
            Opcode::POOL,
            Opcode::ELSE,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0xCC],
            },
            Opcode::FI,
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0xDD],
            },
            Opcode::RETURN,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let r = i.exec().unwrap();
        assert_eq!(r, Some(vec![0xDD]));
    }

    #[test]
    fn infitine_loop_should_hit_execution_limit() {
        // A infinite loop like this runs 1000 times until it hits the limit
        let mut i =
            ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::LOOP, Opcode::POOL]), None);

        let res = i.exec();
        assert_eq!(res, Err(ScriptError::ExecutionLimitExceeded));
    }

    #[test]
    fn infinite_jump_should_hit_execution_limit() {
        let mut i =
            ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::PUSH1(0), Opcode::JMP]), None);

        let res = i.exec();
        assert_eq!(res, Err(ScriptError::ExecutionLimitExceeded));
    }

    #[test]
    fn infinite_big_jump_should_hit_search_limit() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![
                Opcode::PUSH1(6),
                Opcode::JMP,
                Opcode::FALSE,
                Opcode::TRUE,
                Opcode::FALSE,
                Opcode::TRUE,
                Opcode::PUSH1(0),
                Opcode::JMP,
            ]),
            None,
        );

        let res = i.exec();
        assert_eq!(res, Err(ScriptError::SearchLimitExceeded));
    }

    #[test]
    fn big_arrays_should_hit_memory_limit() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![
                Opcode::LOOP,
                Opcode::PUSHL1 {
                    len: 0,
                    data: [0u8; 1000].to_vec(),
                },
                Opcode::POOL,
            ]),
            None,
        );

        let res = i.exec();
        assert_eq!(res, Err(ScriptError::MemoryLimitExceeded));

        // Push runs 10 times, fails the 11th time for memory limit is 10_000 by default
        // So we expect 12 cpu (LOOP, POOL and 10x PUSHL1)
        assert_eq!(i.executions, 12);
    }

    #[test]
    fn big_slice_should_hit_slice_limit() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![Opcode::PUSHL1 {
                len: 0,
                data: [0u8; 20_000].to_vec(),
            }]),
            None,
        );
        let res = i.exec();

        assert_eq!(res, Err(ScriptError::SliceLimitExceeded));
    }

    #[test]
    fn grow_stack_should_hit_stack_limit() {
        let mut i = ScriptInterpreter::new(
            OpcodeScript::new(vec![
                Opcode::PUSH1(0),
                Opcode::PUSH1(1),
                Opcode::PUSH1(2),
                Opcode::PUSH1(3),
                Opcode::LOOP,
                Opcode::DUP4,
                Opcode::POOL,
            ]),
            None,
        );
        let res = i.exec();

        assert_eq!(res, Err(ScriptError::StackHeightLimitExceeded));
    }

    #[test]
    fn opcode_limit_enforced_non_eval() {
        // Script should not exceed 100 instructions
        let mut instructions = Vec::new();
        for _ in 0..101 {
            instructions.push(Opcode::NOP);
        }

        let mut i = ScriptInterpreter::new(OpcodeScript::new(instructions), None);
        let res = i.exec();
        assert_eq!(res, Err(ScriptError::OpcodeLimitExceeded));
    }

    #[test]
    fn opcode_limit_enforced_eval() {
        // Script should not exceed 100 instructions
        for code in vec![Opcode::EVAL, Opcode::EVALSUB] {
            let mut instructions = Vec::new();
            for _ in 0..50 {
                instructions.push(Opcode::NOP);
            }

            let eval_instructions = instructions.clone();
            let config: Option<&mut SerializerConfig> = None;
            let eval_script = OpcodeScript::new(eval_instructions)
                .to_bytes(config)
                .unwrap();

            instructions.push(Opcode::PUSHL1 {
                len: 0,
                data: eval_script,
            });
            instructions.push(code);

            let mut settings = ScriptSettings::default();
            settings.allow_eval = true;
            settings.allow_sandboxed_eval = true;

            let mut i = ScriptInterpreter::new(OpcodeScript::new(instructions), Some(settings));
            let res = i.exec();
            assert_eq!(res, Err(ScriptError::OpcodeLimitExceeded));
        }
    }

    #[test]
    fn can_concat_two_bytes_into_a_number_using_number_cast() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSH1(130),
            Opcode::PUSH1(130),
            Opcode::PUSH1(1),
            Opcode::CONCAT,
            Opcode::NUMBER,
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        let res = i.exec();

        assert_eq!(res, Ok(None));
    }

    #[test]
    fn float_arithmetic_and_comparisons() {
        let script = OpcodeScript::new(vec![
            // FADD: 1.5 + 2.5 = 4.0
            Opcode::PUSHFLOAT(1.5),
            Opcode::PUSHFLOAT(2.5),
            Opcode::FADD,
            Opcode::PUSHFLOAT(4.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // FSUB: 5.5 - 2.0 = 3.5
            Opcode::PUSHFLOAT(5.5),
            Opcode::PUSHFLOAT(2.0),
            Opcode::FSUB,
            Opcode::PUSHFLOAT(3.5),
            Opcode::EQ,
            Opcode::ASSERT,
            // FMUL: 2.0 * 3.5 = 7.0
            Opcode::PUSHFLOAT(2.0),
            Opcode::PUSHFLOAT(3.5),
            Opcode::FMUL,
            Opcode::PUSHFLOAT(7.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // FDIV: 7.5 / 2.5 = 3.0
            Opcode::PUSHFLOAT(7.5),
            Opcode::PUSHFLOAT(2.5),
            Opcode::FDIV,
            Opcode::PUSHFLOAT(3.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // FMOD: 7.5 % 2.0 = 1.5
            Opcode::PUSHFLOAT(7.5),
            Opcode::PUSHFLOAT(2.0),
            Opcode::FMOD,
            Opcode::PUSHFLOAT(1.5),
            Opcode::EQ,
            Opcode::ASSERT,
            // FLT: 2.0 < 3.0 => true
            Opcode::PUSHFLOAT(2.0),
            Opcode::PUSHFLOAT(3.0),
            Opcode::FLT,
            Opcode::ASSERT,
            // FGT: 3.0 > 2.0 => true
            Opcode::PUSHFLOAT(3.0),
            Opcode::PUSHFLOAT(2.0),
            Opcode::FGT,
            Opcode::ASSERT,
            // FLTE: 2.0 <= 2.0 => true
            Opcode::PUSHFLOAT(2.0),
            Opcode::PUSHFLOAT(2.0),
            Opcode::FLTE,
            Opcode::ASSERT,
            // FGTE: 3.0 >= 2.0 => true
            Opcode::PUSHFLOAT(3.0),
            Opcode::PUSHFLOAT(2.0),
            Opcode::FGTE,
            Opcode::ASSERT,
            // FMIN: min(2.5, 3.5) = 2.5
            Opcode::PUSHFLOAT(2.5),
            Opcode::PUSHFLOAT(3.5),
            Opcode::FMIN,
            Opcode::PUSHFLOAT(2.5),
            Opcode::EQ,
            Opcode::ASSERT,
            // FMAX: max(2.5, 3.5) = 3.5
            Opcode::PUSHFLOAT(2.5),
            Opcode::PUSHFLOAT(3.5),
            Opcode::FMAX,
            Opcode::PUSHFLOAT(3.5),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn float_number_casting() {
        let script = OpcodeScript::new(vec![
            // Cast int to float: 42 -> 42.0
            Opcode::PUSHINT(42),
            Opcode::FLOAT,
            Opcode::PUSHFLOAT(42.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // Cast float to int: 13.0 -> 13
            Opcode::PUSHFLOAT(13.0),
            Opcode::NUMBER,
            Opcode::PUSHINT(13),
            Opcode::EQ,
            Opcode::ASSERT,
            // Cast float with fraction to int: 13.7 -> 13 (should truncate)
            Opcode::PUSHFLOAT(13.7),
            Opcode::NUMBER,
            Opcode::PUSHINT(13),
            Opcode::EQ,
            Opcode::ASSERT,
            // Compare float to number
            Opcode::PUSHINT(10),
            Opcode::PUSHFLOAT(3.33),
            Opcode::PUSHFLOAT(6.66),
            Opcode::FADD,
            Opcode::ROUND,
            Opcode::NUMBER,
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }

    #[test]
    fn float_floor_ceil_round_rounede() {
        let script = OpcodeScript::new(vec![
            // FLOOR: 3.7 -> 3.0
            Opcode::PUSHFLOAT(3.7),
            Opcode::FLOOR,
            Opcode::PUSHFLOAT(3.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // CEIL: 3.2 -> 4.0
            Opcode::PUSHFLOAT(3.2),
            Opcode::CEIL,
            Opcode::PUSHFLOAT(4.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // ROUND: 2.5 -> 3.0, 2.4 -> 2.0
            Opcode::PUSHFLOAT(2.5),
            Opcode::ROUND,
            Opcode::PUSHFLOAT(3.0),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHFLOAT(2.4),
            Opcode::ROUND,
            Opcode::PUSHFLOAT(2.0),
            Opcode::EQ,
            Opcode::ASSERT,
            // ROUNDE: 2.5 -> 2.0 (ties to even), 3.5 -> 4.0
            Opcode::PUSHFLOAT(2.5),
            Opcode::ROUNDE,
            Opcode::PUSHFLOAT(2.0),
            Opcode::EQ,
            Opcode::ASSERT,
            Opcode::PUSHFLOAT(3.5),
            Opcode::ROUNDE,
            Opcode::PUSHFLOAT(4.0),
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut i = ScriptInterpreter::new(script, None);
        assert_eq!(i.exec(), Ok(None));
    }
}
