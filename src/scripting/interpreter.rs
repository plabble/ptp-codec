use std::cmp;

use binary_codec::BinaryDeserializer;

use crate::scripting::opcode::{Opcode, OpcodeScript};

use super::stack::StackData;

#[derive(Debug)]
pub struct ScriptInterpreter {
    main_stack: Vec<StackData>,
    alt_stack: Vec<StackData>,
    snapshot: Vec<StackData>,
    script: OpcodeScript,
    cursor: usize,
    use_alt_stack: bool,
    cpu: usize,
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
            cursor: 0,
            script,
            use_alt_stack: false,
            cpu: 0,
        }
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

    fn pop_number(&mut self) -> Result<i128, ScriptError> {
        self.stack()
            .pop()
            .and_then(|n| n.as_number())
            .ok_or(ScriptError::NotANumber)
    }

    fn pop_boolean(&mut self) -> Result<bool, ScriptError> {
        self.stack()
            .pop()
            .and_then(|b| b.as_boolean())
            .ok_or(ScriptError::NotABoolean)
    }

    fn check_equality(&mut self) -> Result<bool, ScriptError> {
        self.ensure_stack_size(2)?;
        let a = self.stack().pop().unwrap();
        let b = self.stack().pop().unwrap();

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
        println!("Executing opcode: {:?}", opcode);

        match opcode {
            Opcode::FALSE => self.stack().push(StackData::Boolean(false)),
            Opcode::TRUE => self.stack().push(StackData::Boolean(true)),
            Opcode::PUSH1(data) => self.stack().push(StackData::Buffer(vec![data])),
            Opcode::PUSH2(data) => self.stack().push(StackData::Buffer(data.to_vec())),
            Opcode::PUSH4(data) => self.stack().push(StackData::Buffer(data.to_vec())),
            Opcode::PUSHL1 { len: _, data } => self.stack().push(StackData::Buffer(data)),
            Opcode::PUSHL2 { len: _, data } => self.stack().push(StackData::Buffer(data)),
            Opcode::PUSHL4 { len: _, data } => self.stack().push(StackData::Buffer(data)),
            Opcode::PUSHINT(val) => self.stack().push(StackData::Number(val)),

            /* Numeric / math */
            Opcode::ADD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_add(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::SUB => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_sub(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::MUL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_mul(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::DIV => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_div(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::MOD => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_rem(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::NEG => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = a.checked_neg().ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::ABS => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = a.checked_abs().ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::LT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Boolean(b < a));
            }
            Opcode::GT => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Boolean(b > a));
            }
            Opcode::LTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Boolean(b <= a));
            }
            Opcode::GTE => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Boolean(b >= a));
            }
            Opcode::MIN => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Number(cmp::min(a, b)));
            }
            Opcode::MAX => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b = self.pop_number()?;
                self.stack().push(StackData::Number(cmp::max(a, b)));
            }
            Opcode::BAND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a & b;
                self.stack().push(StackData::Number(c));
            }
            Opcode::BOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a | b;
                self.stack().push(StackData::Number(c));
            }
            Opcode::BXOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a ^ b;
                self.stack().push(StackData::Number(c));
            }
            Opcode::BSHL => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shl(b as u32).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::BSHR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_number()?;
                let a = self.pop_number()?;
                let c = a.checked_shr(b as u32).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::BNOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                let c = !a;
                self.stack().push(StackData::Number(c));
            }

            /* Boolean / logic */
            Opcode::NOT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_boolean()?;
                self.stack().push(StackData::Boolean(!a));
            }
            Opcode::AND => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.stack().push(StackData::Boolean(a && b));
            }
            Opcode::OR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.stack().push(StackData::Boolean(a || b));
            }
            Opcode::XOR => {
                self.ensure_stack_size(2)?;
                let b = self.pop_boolean()?;
                let a = self.pop_boolean()?;
                self.stack().push(StackData::Boolean(a ^ b));
            }
            Opcode::EQ => {
                let eq = self.check_equality()?;
                self.stack().push(StackData::Boolean(eq));
            }
            Opcode::NEQ => {
                let eq = self.check_equality()?;
                self.stack().push(StackData::Boolean(!eq));
            }

            /* Advanced math */
            Opcode::POW => {
                self.ensure_stack_size(2)?;
                let a = self.pop_number()?;
                let b: u32 = self
                    .pop_number()?
                    .try_into()
                    .map_err(|_| ScriptError::MathError)?;
                let c = a.checked_pow(b).ok_or(ScriptError::MathError)?;
                self.stack().push(StackData::Number(c));
            }
            Opcode::SQRT => {
                self.ensure_stack_size(1)?;
                let a = self.pop_number()?;
                if a < 0 {
                    return Err(ScriptError::MathError);
                }
                let c = (a as f64).sqrt() as i128;
                self.stack().push(StackData::Number(c));
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
                        if self.cursor >= self.script.instructions.len() {
                            break;
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
                // Skip to FI
                let mut depth = 1;
                while depth > 0 {
                    self.cursor += 1;
                    if self.cursor >= self.script.instructions.len() {
                        break;
                    }
                    match self.script.instructions[self.cursor] {
                        Opcode::IF => depth += 1,
                        Opcode::FI => depth -= 1,
                        _ => {}
                    }
                }
            }
            Opcode::FI => {
                // Just continue execution
            }
            Opcode::BREAK => {
                // Skip to next POOL
                let mut depth = 0;
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
                    self.cursor -= 1;
                }
            }

            Opcode::JMP => todo!(),

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
                    let buffer = item.as_buffer().ok_or(ScriptError::NotABoolean)?;
                    stack_data.extend_from_slice(&buffer);
                }
                return Ok(Some(stack_data));
            }

            /* Stack manipulation */
            Opcode::DUP => {
                self.ensure_stack_size(1)?;
                let top = self.stack().last().unwrap().clone();
                self.stack().push(top);
            }
            Opcode::DUP2 => {
                self.ensure_stack_size(2)?;
                let len = self.stack().len();
                let first = self.stack()[len - 2].clone();
                let second = self.stack()[len - 1].clone();
                self.stack().push(first);
                self.stack().push(second);
            }
            Opcode::DUP3 => {
                self.ensure_stack_size(3)?;
                let len = self.stack().len();
                let first = self.stack()[len - 3].clone();
                let second = self.stack()[len - 2].clone();
                let third = self.stack()[len - 1].clone();
                self.stack().push(first);
                self.stack().push(second);
                self.stack().push(third);
            }
            Opcode::DUP4 => {
                self.ensure_stack_size(4)?;
                let len = self.stack().len();
                let first = self.stack()[len - 4].clone();
                let second = self.stack()[len - 3].clone();
                let third = self.stack()[len - 2].clone();
                let fourth = self.stack()[len - 1].clone();
                self.stack().push(first);
                self.stack().push(second);
                self.stack().push(third);
                self.stack().push(fourth);
            }
            Opcode::DUPN(n) => {
                self.ensure_stack_size(1)?;
                let top = self.stack().last().unwrap().clone();
                for _ in 0..n {
                    self.stack().push(top.clone());
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
                self.stack().pop();
            }
            Opcode::COPY => {
                self.ensure_stack_size(1)?;
                let n = self.pop_number()?;
                if n < 0 || n as usize >= self.stack().len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let item = self.stack()[n as usize].clone();
                self.stack().push(item);
            }
            Opcode::BUBBLE => {
                self.ensure_stack_size(1)?;
                let n = self.pop_number()?;
                if n < 0 || n as usize >= self.stack().len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let item = self.stack().remove(n as usize);
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
                let item = self.stack().pop().unwrap();
                self.alt_stack().push(item);
            }
            Opcode::FROMALT => {
                if self.alt_stack().is_empty() {
                    return Err(ScriptError::StackUnderflow(1));
                }
                let item = self.alt_stack().pop().unwrap();
                self.stack().push(item);
            }
            Opcode::SNAPSHOT => {
                self.snapshot = self.stack().clone();
            }
            Opcode::RESTORE => {
                let snapshot = self.snapshot.clone();
                self.snapshot.clear();
                self.stack().extend_from_slice(&snapshot);
            }
            Opcode::CLEAR => {
                self.stack().clear();
            }
            Opcode::SWITCH => {
                self.use_alt_stack = !self.use_alt_stack;
            }
            Opcode::CONCAT => {
                self.ensure_stack_size(2)?;
                let a = self.stack().pop().unwrap();
                let b = self.stack().pop().unwrap();

                let a_bytes = a.as_buffer().expect("Failed to convert to buffer");
                let b_bytes = b.as_buffer().expect("Failed to convert to buffer");

                let mut combined = a_bytes;
                combined.extend_from_slice(&b_bytes);

                self.stack().push(StackData::Buffer(combined));
            }
            Opcode::COUNT => {
                let length = self.stack().len() as i128;
                self.stack().push(StackData::Number(length));
            }

            Opcode::SERVER => todo!(),
            Opcode::SELECT => todo!(),
            Opcode::READ => todo!(),
            Opcode::WRITE => todo!(),
            Opcode::APPEND => todo!(),
            Opcode::DELETE => todo!(),

            Opcode::LEN => {
                self.ensure_stack_size(1)?;
                let item = self.stack().pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");
                let length = bytes.len() as i128;
                self.stack().push(StackData::Number(length));
            }
            Opcode::REVERSE => {
                self.ensure_stack_size(1)?;
                let item = self.stack().pop().unwrap();
                let mut bytes = item.as_buffer().expect("Failed to convert to buffer");
                bytes.reverse();
                self.stack().push(StackData::Buffer(bytes));
            }
            Opcode::SLICE => {
                self.ensure_stack_size(3)?;
                let length = self.pop_number()?;
                let offset = self.pop_number()?;
                let item = self.stack().pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                if offset < 0 || length < 0 || (offset as usize) + (length as usize) > bytes.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                let slice = bytes
                    .get(offset as usize..(offset as usize) + (length as usize))
                    .unwrap()
                    .to_vec();

                self.stack().push(StackData::Buffer(slice));
            }
            Opcode::SPLICE => {
                self.ensure_stack_size(3)?;
                let offset = self.pop_number()?;
                let length = self.pop_number()?;
                let item = self.stack().pop().unwrap();
                let splice_data = item.as_buffer().expect("Failed to convert to buffer");
                let item = self.stack().pop().unwrap();
                let mut bytes = item
                    .as_buffer()
                    .expect("Failed to convert to buffer");

                if offset < 0 || length < 0 || (offset as usize) + (length as usize) > bytes.len() {
                    return Err(ScriptError::OutOfBounds);
                }

                bytes.splice((offset as usize)..((offset + length) as usize), splice_data);
                self.stack().push(StackData::Buffer(bytes));
            }

            Opcode::HASH => todo!(),
            Opcode::SIGN => todo!(),
            Opcode::VERIFY => todo!(),
            Opcode::ENCRYPT => todo!(),
            Opcode::DECRYPT => todo!(),
            Opcode::EVALSUB => {
                self.ensure_stack_size(1)?;
                let item = self.stack().pop().unwrap();
                let bytes = item.as_buffer().expect("Failed to convert to buffer");

                let config: Option<&mut binary_codec::SerializerConfig> = None;
                let script = OpcodeScript::from_bytes(&bytes, config)
                    .map_err(|_| ScriptError::InvalidScript)?;

                let mut sub_interpreter = ScriptInterpreter::new(script);
                let result = sub_interpreter.exec()?;

                if let Some(result_bytes) = result {
                    self.stack().push(StackData::Buffer(result_bytes));
                }

                self.cpu += sub_interpreter.cpu; // Add sub-interpreter CPU usage to parent
            }
            Opcode::EVAL => {
                self.ensure_stack_size(1)?;
                let item = self.stack().pop().unwrap();
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
        self.cpu += 1; // Every opcode costs 1 CPU cycle

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::scripting::{
        interpreter::ScriptInterpreter,
        opcode::{Opcode, OpcodeScript}, stack::StackData,
    };

    #[test]
    fn can_create_and_break_a_loop() {
        // Generate [1,2,3,4,5,6,7,8,9,10]
        let script = OpcodeScript::new(vec![
            Opcode::LOOP,
            Opcode::COUNT,
            Opcode::PUSHINT(9),
            Opcode::EQ,
            Opcode::IF,
            Opcode::BREAK,
            Opcode::FI,
            Opcode::COUNT,
            Opcode::PUSHINT(1),
            Opcode::ADD,
            Opcode::POOL,
            Opcode::PUSHINT(10),
        ]);

        let mut interpreter = ScriptInterpreter::new(script);
        interpreter.exec().unwrap();

        println!("{:?}", interpreter);
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
            &[StackData::Buffer(vec![1,2,3, 4, 5, 6, 7, 8, 9])][..]
        );
    }
}
