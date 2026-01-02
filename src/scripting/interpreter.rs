use std::cmp;

use binary_codec::BinaryDeserializer;
use chrono::Utc;

use crate::{packets::base::datetime::PlabbleDateTime, scripting::opcode::{Opcode, OpcodeScript}};

use super::stack::StackData;

#[derive(Debug)]
pub struct ScriptInterpreter {
    main_stack: Vec<StackData>,
    alt_stack: Vec<StackData>,
    snapshot: Vec<StackData>,
    snapshot_memory: usize,

    script: OpcodeScript,
    cursor: usize,
    use_alt_stack: bool,
    cpu: usize,
    cursor_cpu: usize,
    memory: usize,
    memory_peak: usize,
}

#[derive(Debug, PartialEq)]
pub enum ScriptError {
    /// When the stack is empty while n items are required
    StackUnderflow(usize),

    /// When an operation expected a number but got something else
    NotANumber,

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
}

impl ScriptInterpreter {
    pub fn new(script: OpcodeScript) -> Self {
        ScriptInterpreter {
            main_stack: Vec::new(),
            alt_stack: Vec::new(),
            snapshot: Vec::new(),
            snapshot_memory: 0,

            cursor: 0,
            script,
            use_alt_stack: false,
            cpu: 0,
            cursor_cpu: 0,
            memory: 0,
            memory_peak: 0,
        }
    }

    fn calculate_memory(&mut self) -> usize {
        self.stack().iter().map(|i| i.memory()).sum()
    }

    fn stack(&mut self) -> &mut Vec<StackData> {
        if self.use_alt_stack {
            &mut self.alt_stack
        } else {
            &mut self.main_stack
        }
    }

    fn alt_stack(&mut self) -> &mut Vec<StackData> {
        if self.use_alt_stack {
            &mut self.main_stack
        } else {
            &mut self.alt_stack
        }
    }

    fn ensure_stack_size(&mut self, size: usize) -> Result<(), ScriptError> {
        if self.stack().len() < size {
            return Err(ScriptError::StackUnderflow(size - self.stack().len()));
        }
        Ok(())
    }

    fn pop(&mut self) -> Option<StackData> {
        let item = self.stack().pop();
        if item.is_none() {
            return None;
        }

        let item = item.unwrap();
        self.memory -= item.memory();

        Some(item)
    }

    fn push(&mut self, item: StackData) {
        self.memory += item.memory();
        self.memory_peak = cmp::max(self.memory, self.memory_peak);
        self.stack().push(item);
    }

    fn pop_number(&mut self) -> Result<i128, ScriptError> {
        self.pop()
            .and_then(|n| n.as_number())
            .ok_or(ScriptError::NotANumber)
    }

    fn pop_boolean(&mut self) -> Result<bool, ScriptError> {
        self.pop()
            .and_then(|b| b.as_boolean())
            .ok_or(ScriptError::NotABoolean)
    }

    fn check_equality(&mut self) -> Result<bool, ScriptError> {
        self.ensure_stack_size(2)?;
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();

        match (a, b) {
            (StackData::Boolean(a), StackData::Boolean(b)) => return Ok(a == b),
            (StackData::Number(a), StackData::Number(b)) => return Ok(a == b),
            (StackData::Byte(a), StackData::Byte(b)) => return Ok(a == b),
            (StackData::Number(a), StackData::Byte(b)) => return Ok(a == b as i128),
            (StackData::Byte(a), StackData::Number(b)) => return Ok(a as i128 == b),
            (StackData::Boolean(a), StackData::Number(b)) => {
                return Ok((if a { 1 } else { 0 }) == b);
            }
            (StackData::Number(a), StackData::Boolean(b)) => {
                return Ok(a == (if b { 1 } else { 0 }));
            }
            (StackData::Boolean(a), StackData::Byte(b)) => return Ok((if a { 1 } else { 0 }) == b),
            (StackData::Byte(a), StackData::Boolean(b)) => return Ok(a == (if b { 1 } else { 0 })),
            (a, b) => {
                let a = a.as_buffer().expect("Failed to convert to buffer");
                let b = b.as_buffer().expect("Failed to convert to buffer");

                return Ok(a == b);
            }
        }
    }

    pub fn exec(&mut self) -> Result<Option<Vec<u8>>, ScriptError> {
        while self.cursor < self.script.instructions.len() {
            let res = self.exec_next()?;
            if res.is_some() {
                return Ok(res);
            }
        }

        Ok(None)
    }

    pub fn exec_next(&mut self) -> Result<Option<Vec<u8>>, ScriptError> {
        if self.cursor >= self.script.instructions.len() {
            return Ok(None);
        }

        let opcode = self.script.instructions[self.cursor].clone();
        self.cpu += 1; // Every opcode execution costs 1 CPU cycle
        println!("Executing opcode: {:?}", opcode);

        match opcode {
            Opcode::FALSE => self.push(StackData::Boolean(false)),
            Opcode::TRUE => self.push(StackData::Boolean(true)),
            Opcode::PUSH1(data) => self.push(StackData::Buffer(vec![data])),
            Opcode::PUSH2(data) => self.push(StackData::Buffer(data.to_vec())),
            Opcode::PUSH4(data) => self.push(StackData::Buffer(data.to_vec())),
            Opcode::PUSHL1 { len: _, data } => self.push(StackData::Buffer(data)),
            Opcode::PUSHL2 { len: _, data } => self.push(StackData::Buffer(data)),
            Opcode::PUSHL4 { len: _, data } => self.push(StackData::Buffer(data)),
            Opcode::PUSHINT(val) => self.push(StackData::Number(val)),
            Opcode::ADD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_add(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::SUB => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_sub(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::MUL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_mul(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::DIV => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_div(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::MOD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_rem(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::NEG => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = a.checked_neg().ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::ABS => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = a.checked_abs().ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::LT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b < a));
            }
            Opcode::GT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b > a));
            }
            Opcode::LTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b <= a));
            }
            Opcode::GTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Boolean(b >= a));
            }
            Opcode::MIN => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Number(cmp::min(a, b)));
            }
            Opcode::MAX => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.push(StackData::Number(cmp::max(a, b)));
            }
            Opcode::BAND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a & b;
                self.push(StackData::Number(c));
            }
            Opcode::BOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a | b;
                self.push(StackData::Number(c));
            }
            Opcode::BXOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a ^ b;
                self.push(StackData::Number(c));
            }
            Opcode::BSHL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shl(b as u32).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::BSHR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shr(b as u32).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::BNOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = !a;
                self.push(StackData::Number(c));
            }
            Opcode::NOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(!a));
            }
            Opcode::AND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a && b));
            }
            Opcode::OR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a || b));
            }
            Opcode::XOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.push(StackData::Boolean(a ^ b));
            }
            Opcode::EQ => {
                let eq = self.check_equality()?;
                self.push(StackData::Boolean(eq));
            }
            Opcode::NEQ => {
                let eq = self.check_equality()?;
                self.push(StackData::Boolean(!eq));
            }
            Opcode::POW => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b: u32 = self
                    .pop_number()?
                    .try_into()
                    .map_err(|_| ScriptError::MathError)?;
                let c = a.checked_pow(b).ok_or(ScriptError::MathError)?;
                self.push(StackData::Number(c));
            }
            Opcode::SQRT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                if a < 0 {
                    return Err(ScriptError::MathError);
                }
                let c = (a as f64).sqrt() as i128;
                self.push(StackData::Number(c));
            }
            Opcode::NOP => { /* NOP = do nothing */ }
            Opcode::IF => {
                self.ensure_stack_size(1)?;
                let condition = self.pop_boolean()?;
                if !condition {
                    // Skip to ELSE or FI
                    let mut depth = 1;
                    while depth > 0 {
                        self.cursor += 1;
                        self.cursor_cpu += 1;
                        if self.cursor >= self.script.instructions.len() {
                            return Err(ScriptError::InvalidScript);
                        }
                        match self.script.instructions[self.cursor] {
                            Opcode::IF => depth += 1,
                            Opcode::ELSE => {
                                if depth == 1 {
                                    break;
                                }
                            }
                            Opcode::FI => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            Opcode::ELSE => {
                // Ensure there is a matching IF before this ELSE
                let mut rev_depth: isize = 0;
                let mut found_if = false;
                if self.cursor == 0 {
                    return Err(ScriptError::InvalidScript);
                }
                let mut j = self.cursor as isize - 1;
                while j >= 0 {
                    match self.script.instructions[j as usize] {
                        Opcode::FI => rev_depth += 1,
                        Opcode::IF => {
                            if rev_depth == 0 {
                                found_if = true;
                                break;
                            } else {
                                rev_depth -= 1;
                            }
                        }
                        _ => {}
                    }
                    j -= 1;
                    self.cursor_cpu += 1;
                }
                if !found_if {
                    return Err(ScriptError::InvalidScript);
                }

                // Skip to FI
                let mut depth = 1;
                while depth > 0 {
                    self.cursor += 1;
                    self.cursor_cpu += 1;

                    if self.cursor >= self.script.instructions.len() {
                        return Err(ScriptError::InvalidScript);
                    }
                    match self.script.instructions[self.cursor] {
                        Opcode::IF => depth += 1,
                        Opcode::FI => depth -= 1,
                        _ => {}
                    }
                }
            }
            Opcode::FI => {
                // Validate there is a matching IF earlier
                let mut rev_depth: isize = 0;
                let mut found_if = false;
                if self.cursor == 0 {
                    return Err(ScriptError::InvalidScript);
                }
                let mut j = self.cursor as isize - 1;
                while j >= 0 {
                    match self.script.instructions[j as usize] {
                        Opcode::FI => rev_depth += 1,
                        Opcode::IF => {
                            if rev_depth == 0 {
                                found_if = true;
                                break;
                            } else {
                                rev_depth -= 1;
                            }
                        }
                        _ => {}
                    }
                    j -= 1;
                    self.cursor_cpu += 1;
                }
                if !found_if {
                    return Err(ScriptError::InvalidScript);
                }
            }
            Opcode::BREAK => {
                // Ensure there's an enclosing LOOP (search backwards)
                let mut rev_depth = 0isize;
                let mut found_loop = false;
                if self.cursor == 0 {
                    return Err(ScriptError::InvalidScript);
                }
                let mut i = self.cursor as isize - 1;
                while i >= 0 {
                    match self.script.instructions[i as usize] {
                        Opcode::POOL => rev_depth += 1,
                        Opcode::LOOP => {
                            if rev_depth == 0 {
                                found_loop = true;
                                break;
                            } else {
                                rev_depth -= 1;
                            }
                        }
                        _ => {}
                    }
                    i -= 1;
                    self.cursor_cpu += 1;
                }
                if !found_loop {
                    return Err(ScriptError::InvalidScript);
                }

                // Skip to next POOL (forward), taking nesting into account
                let mut depth = 0;
                self.cursor += 1;
                while self.cursor < self.script.instructions.len() {
                    match self.script.instructions[self.cursor] {
                        Opcode::LOOP => depth += 1,
                        Opcode::POOL => {
                            if depth == 0 {
                                break;
                            } else {
                                depth -= 1;
                            }
                        }
                        _ => {}
                    }

                    self.cursor += 1;
                    self.cursor_cpu += 1;
                }
                if self.cursor >= self.script.instructions.len() {
                    return Err(ScriptError::InvalidScript);
                }
            }
            Opcode::LOOP => {
                // Just continue execution
            }
            Opcode::POOL => {
                // Jump back to the corresponding LOOP
                let mut depth = 0;
                while self.cursor > 0 {
                    match self.script.instructions[self.cursor] {
                        Opcode::POOL => depth += 1,
                        Opcode::LOOP => {
                            if depth == 0 {
                                break;
                            } else {
                                depth -= 1;
                            }
                        }
                        _ => {}
                    }
                    if self.cursor == 0 {
                        break;
                    }
                    self.cursor -= 1;
                    self.cursor_cpu += 1;
                }
                if self.script.instructions.get(self.cursor) != Some(&Opcode::LOOP) {
                    return Err(ScriptError::InvalidScript);
                }
            }
            Opcode::JMP => {
                self.ensure_stack_size(1)?;
                let address = self.pop_number()?;
                if address < 0 || (address as usize) >= self.script.instructions.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                self.cursor_cpu += address.abs_diff(self.cursor as i128) as usize;
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
                self.push(top);
            }
            Opcode::DUP2 => {
                self.ensure_stack_size(2)?;
                let len = self.stack().len();
                let first = self.stack()[len - 2].clone();
                let second = self.stack()[len - 1].clone();
                self.push(first);
                self.push(second);
            }
            Opcode::DUP3 => {
                self.ensure_stack_size(3)?;
                let len = self.stack().len();
                let first = self.stack()[len - 3].clone();
                let second = self.stack()[len - 2].clone();
                let third = self.stack()[len - 1].clone();
                self.push(first);
                self.push(second);
                self.push(third);
            }
            Opcode::DUP4 => {
                self.ensure_stack_size(4)?;
                let len = self.stack().len();
                let first = self.stack()[len - 4].clone();
                let second = self.stack()[len - 3].clone();
                let third = self.stack()[len - 2].clone();
                let fourth = self.stack()[len - 1].clone();
                self.push(first);
                self.push(second);
                self.push(third);
                self.push(fourth);
            }
            Opcode::DUPN(n) => {
                self.ensure_stack_size(1)?;
                let top = self.stack().last().unwrap().clone();
                for _ in 0..n {
                    self.push(top.clone());
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
                self.push(item);
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
                self.snapshot_memory = self.memory;
                self.snapshot = self.stack().clone();
            }
            Opcode::RESTORE => {
                let snapshot = self.snapshot.clone();
                self.snapshot.clear();
                self.memory = self.snapshot_memory;
                self.snapshot_memory = 0;

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

                self.push(StackData::Buffer(combined));
            }
            Opcode::COUNT => {
                let length = self.stack().len() as i128;
                self.push(StackData::Number(length));
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
                self.push(StackData::Number(length));
            }
            Opcode::REVERSE => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let mut bytes = item.as_buffer().expect("Failed to convert to buffer");
                bytes.reverse();
                self.push(StackData::Buffer(bytes));
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

                self.push(StackData::Buffer(slice));
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
                self.push(StackData::Buffer(bytes));
            }

            /* Crypto operations */
            Opcode::HASH => todo!(),
            Opcode::SIGN => todo!(),
            Opcode::VERIFY => todo!(),
            Opcode::ENCRYPT => todo!(),
            Opcode::DECRYPT => todo!(),

            /* Special opcodes */
            Opcode::TIME => {
                let now = PlabbleDateTime(Utc::now());
                self.push(StackData::Number(now.timestamp() as i128));
            },
            Opcode::EVALSUB => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                let config: Option<&mut binary_codec::SerializerConfig> = None;
                let script = OpcodeScript::from_bytes(&bytes, config)
                    .map_err(|_| ScriptError::InvalidScript)?;

                let mut sub_interpreter = ScriptInterpreter::new(script);
                let result = sub_interpreter.exec()?;

                if let Some(result_bytes) = result {
                    self.push(StackData::Buffer(result_bytes));
                }

                self.cpu += sub_interpreter.cpu; // Add sub-interpreter CPU usage to parent
            }
            Opcode::EVAL => {
                self.ensure_stack_size(1)?;
                let item = self.pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                let config: Option<&mut binary_codec::SerializerConfig> = None;
                let script = OpcodeScript::from_bytes(&bytes, config)
                    .map_err(|_| ScriptError::InvalidScript)?;

                // Insert new script instructions at current position
                self.script.instructions.splice(
                    self.cursor + 1..self.cursor + 1,
                    script.instructions.into_iter(),
                );
            }
        };

        self.cursor += 1;

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::scripting::{
        interpreter::ScriptInterpreter,
        opcode::{Opcode, OpcodeScript},
        stack::StackData,
    };

    #[test]
    fn can_create_and_break_a_loop() {
        // Generate [1,2,3]
        let script = OpcodeScript::new(vec![
            //                      CPU         | Cursor CPU  | Memory
            Opcode::LOOP,       // 1
            Opcode::COUNT,      // 2 10 18         12 22 32    +2
            Opcode::PUSHINT(2), // 3 11 19         11 21 31    +2
            Opcode::EQ,         // 4 12 20         10 20 30    -4, +1
            Opcode::IF,         // 5 13 21         9  19 29    -1
            Opcode::BREAK,      //      22       1 8  18 28
            Opcode::FI,         //               2 7  17 27
            Opcode::COUNT,      // 6 14            6  16 26    +2
            Opcode::PUSHINT(1), // 7 15            5  15 25    +2
            Opcode::ADD,        // 8 16            4  14 24    -4, +2
            Opcode::POOL,       // 9 17          3    13 23
            Opcode::PUSHINT(3), //      23                     +2
        ]);

        let mut interpreter = ScriptInterpreter::new(script);
        interpreter.exec().unwrap();

        assert_eq!(23, interpreter.cpu);
        assert_eq!(32, interpreter.cursor_cpu);
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

        let mut interpreter = ScriptInterpreter::new(script);
        let result = interpreter.exec();

        assert_eq!(result, Ok(None));
        println!("{:?}", interpreter);
    }

    #[test]
    fn can_do_eval() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(10),
            Opcode::PUSHL1 {
                len: 0,
                data: vec![0x02, 5, 0x02, 2, 0xB],
            },
            Opcode::EVAL,
            Opcode::EQ,
            Opcode::ASSERT,
        ]);

        let mut interpreter = ScriptInterpreter::new(script);
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

        let mut interpreter = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![9])));
        // Make sure NOP is skipped
        assert_eq!(i.cpu, 4);
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

        let mut i2 = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
        assert_eq!(i.exec(), Ok(None));
        // verify values landed in correct stacks
        assert_eq!(i.main_stack[0], StackData::Number(1));
        assert_eq!(i.alt_stack[0], StackData::Number(2));
    }

    #[test]
    fn evalsub_executes_child_script_and_pushes_result() {
        // child script: PUSH1(9); RETURN
        let child_bytes = vec![0x02, 9, 0x32]; // PUSH1, 9, RETURN

        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: child_bytes,
            },
            Opcode::EVALSUB,
        ]);

        let mut i = ScriptInterpreter::new(script);
        let r = i.exec();
        assert_eq!(r, Ok(None));
        // child result should be pushed as a buffer [9]
        assert_eq!(i.main_stack[0], StackData::Buffer(vec![9]));
    }

    use super::ScriptError;

    #[test]
    fn if_with_non_boolean_errors() {
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(2), Opcode::IF]);
        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::NotABoolean);
    }

    #[test]
    fn unterminated_if_returns_invalid_script() {
        let script = OpcodeScript::new(vec![Opcode::FALSE, Opcode::IF]);
        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::InvalidScript);
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

        let mut i = ScriptInterpreter::new(script);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![9])));
    }

    #[test]
    fn break_outside_loop_is_error() {
        let script = OpcodeScript::new(vec![Opcode::BREAK, Opcode::POOL]);
        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::InvalidScript);
    }

    #[test]
    fn pool_without_loop_is_error() {
        let script = OpcodeScript::new(vec![Opcode::POOL]);
        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::InvalidScript);
    }

    #[test]
    fn toalt_and_fromalt_underflow() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::TOALT]));
        let err = i.exec().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow(_)));

        let mut i2 = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::FROMALT]));
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

        let mut i = ScriptInterpreter::new(script);
        assert_eq!(i.exec(), Ok(None));
        assert_eq!(
            i.main_stack,
            vec![StackData::Number(1), StackData::Number(3)]
        );
        assert_eq!(i.alt_stack, vec![StackData::Number(2)]);
    }

    #[test]
    fn jmp_invalid_addresses_error() {
        let mut i =
            ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::PUSHINT(-1), Opcode::JMP]));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::OutOfBounds);

        let mut i2 =
            ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::PUSHINT(9999), Opcode::JMP]));
        let err2 = i2.exec().unwrap_err();
        assert_eq!(err2, ScriptError::OutOfBounds);
    }

    #[test]
    fn dupn_zero_noop() {
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(7), Opcode::DUPN(0), Opcode::COUNT]);
        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
        let r = i.exec().unwrap();
        assert_eq!(r, Some(vec![2]));
    }

    #[test]
    fn else_without_if_is_error() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::ELSE]));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::InvalidScript);
    }

    #[test]
    fn fi_without_if_is_error() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::FI]));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::InvalidScript);
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

        let mut i = ScriptInterpreter::new(script);
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

        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow(_)));
    }

    #[test]
    fn jmp_to_last_instruction() {
        // last instruction is NOP
        let script = OpcodeScript::new(vec![Opcode::PUSHINT(2), Opcode::JMP, Opcode::NOP]);
        let mut i = ScriptInterpreter::new(script);
        let r = i.exec();
        assert_eq!(r, Ok(None));
    }

    #[test]
    fn evalsub_child_assert_failure_propagates() {
        // child: FALSE, ASSERT -> ASSERT will fail
        let child_bytes = vec![0u8, 49u8]; // FALSE = 0, ASSERT = 49
        let script = OpcodeScript::new(vec![
            Opcode::PUSHL1 {
                len: 0,
                data: child_bytes,
            },
            Opcode::EVALSUB,
        ]);
        let mut i = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
        let r = i.exec();
        assert_eq!(r, Ok(Some(vec![7])));
        // three opcodes executed
        assert_eq!(i.cpu, 3);
    }

    #[test]
    fn count_alt_and_after_switch() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(1), // main
            Opcode::SWITCH,
            Opcode::PUSHINT(2), // alt
            Opcode::COUNT,      // count alt -> 1
        ]);
        let mut i = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
        assert_eq!(i.exec(), Ok(None));
        // snapshot was empty, restore yields empty stack, COUNT pushed 0
        assert_eq!(i.main_stack.last(), Some(&StackData::Number(0)));
    }

    #[test]
    fn div_mod_by_zero_errors() {
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(0),
            Opcode::DIV,
        ]));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::MathError);

        let mut i2 = ScriptInterpreter::new(OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(0),
            Opcode::MOD,
        ]));
        let err2 = i2.exec().unwrap_err();
        assert_eq!(err2, ScriptError::MathError);
    }

    #[test]
    fn boolean_ops_with_non_boolean_error() {
        // use PUSHINT(2) which is not a valid boolean (only 0/1 allowed)
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![
            Opcode::PUSHINT(2),
            Opcode::TRUE,
            Opcode::AND,
        ]));
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::NotABoolean);
    }

    #[test]
    fn return_on_alt_stack_and_empty_stack() {
        // return with empty stack
        let mut i = ScriptInterpreter::new(OpcodeScript::new(vec![Opcode::RETURN]));
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
        let mut i2 = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i = ScriptInterpreter::new(script);
        let err = i.exec().unwrap_err();
        assert_eq!(err, ScriptError::OutOfBounds);

        // COPY negative index
        let script2 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(-1),
            Opcode::COPY,
        ]);
        let mut i2 = ScriptInterpreter::new(script2);
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
        let mut i3 = ScriptInterpreter::new(script3);
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
        let mut i4 = ScriptInterpreter::new(script4);
        let err4 = i4.exec().unwrap_err();
        assert_eq!(err4, ScriptError::OutOfBounds);

        // SINK negative index
        let script5 = OpcodeScript::new(vec![
            Opcode::PUSHINT(1),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(-1),
            Opcode::SINK,
        ]);
        let mut i5 = ScriptInterpreter::new(script5);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i2 = ScriptInterpreter::new(script2);
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
        let mut i3 = ScriptInterpreter::new(script3);
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
        let mut i4 = ScriptInterpreter::new(script4);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i2 = ScriptInterpreter::new(script2);
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
        let mut it = ScriptInterpreter::new(script3);
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
        let mut i = ScriptInterpreter::new(script);
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
        let mut i2 = ScriptInterpreter::new(script2);
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
        let mut i3 = ScriptInterpreter::new(script3);
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

        let mut i = ScriptInterpreter::new(script);
        i.exec().unwrap();
        assert_eq!(i.memory, 11);
        assert_eq!(i.memory_peak, 12);
        assert_eq!(i.cpu, 14);
        assert_eq!(i.cursor_cpu, 0);
    }

    #[test]
    fn time_opcode_works() {
        // Hard to test, we just verify if it pushes a number to the stack because that's what it should do
        let script = OpcodeScript::new(vec![
            Opcode::TIME
        ]);

        let mut i = ScriptInterpreter::new(script);
        i.exec().unwrap();
        
        let res = &i.main_stack[0];
        assert!(matches!(res, &StackData::Number(_)));
    }
}
