use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
    Pop,      // 0
    Push(u8), // 1
    Load,     // 2
    Store,    // 3
    Add,      // 4
    Sub,      // 5
    Mul,      // 6
    Div,      // 7
    Lt,       // 8
    LtEq,     // 9
    Gt,       // 10
    GtEq,     // 11
    Jump,     // 12
    Call,     // 13
    Ret,      // 14
    Print,    // 15
    JumpIf,   // 16
    BoolAnd,  // 17
    BoolOr,   // 18
    BoolNot,  // 19
    Halt,     // 20
    Swap,     // 21
    Eq,       // 22
    Neq,      // 23
}

impl Opcode {
    pub fn execute(
        self,
        ip: &mut u8,
        stack: &mut Vec<u8>,
        vars: &mut HashMap<u8, u8>,
        call_stack: &mut Vec<u8>,
    ) -> bool {
        println!(
            "Start {:?} @0x{:04x}, {:x?}, {:x?}, {:x?}",
            self, ip, stack, vars, call_stack
        );
        match self {
            Self::Push(imm8) => {
                stack.push(imm8);
            }
            Self::Pop => {
                stack.pop().expect("value on stack for pop");
            }
            Self::Load => {
                let idx = stack.pop().expect("index on stack for load");
                stack.push(*vars.get(&idx).unwrap_or(&0));
            }
            Self::Store => {
                let idx = stack.pop().expect("index on stack for store");
                let val = stack.pop().expect("value on stack for store");
                vars.insert(idx, val);
            }
            Self::Print => {
                let val = stack.pop().expect("value on stack for print");
                println!("@0x{:04x}: print 0x{:02x}/{1}", *ip, val);
            }
            Self::Call => {
                let new_ip = stack.pop().expect("new instruction pointer for call");
                call_stack.push(*ip);
                *ip = new_ip;
            }
            Self::Neq => {
                let val_a = stack.pop().expect("first value for equality");
                let val_b = stack.pop().expect("second value for equality");
                if val_a != val_b {
                    stack.push(1);
                } else {
                    stack.push(0);
                }
            }
            Self::Swap => {
                let val_a = stack.pop().expect("first value for swap");
                let val_b = stack.pop().expect("second value for swap");
                stack.push(val_a);
                stack.push(val_b);
            }
            Self::JumpIf => {
                let new_ip = stack
                    .pop()
                    .expect("new instruction pointer for conditional jump");
                let cond = stack.pop().expect("condition for conditional jump");
                if cond == 1 {
                    *ip = new_ip;
                }
            }
            Self::Ret => {
                let ret_ptr = call_stack
                    .pop()
                    .expect("return pointer on call stack (something got clobbered)");
                *ip = ret_ptr;
            }
            Self::Halt => {
                println!("HALT");
                return false;
            }
            Self::Sub => {
                let val_a = stack.pop().expect("first value for subtraction");
                let val_b = stack.pop().expect("second value for subtraction");
                stack.push(val_a.wrapping_sub(val_b));
            }
            Self::Mul => {
                let val_a = stack.pop().expect("first value for multiplication");
                let val_b = stack.pop().expect("second value for multiplication");
                stack.push(val_a.wrapping_mul(val_b));
            }
            _ => panic!("invalid or unimplemented instruction `{:?}`", self),
        }
        println!(
            "  End {:?} @0x{:04x}, {:x?}, {:x?}, {:x?}\n",
            self, ip, stack, vars, call_stack
        );
        true
    }
}
