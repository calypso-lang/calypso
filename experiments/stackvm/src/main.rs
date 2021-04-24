use std::collections::HashMap;

mod ops;

use ops::*;

fn main() {
    let mut ip = 0;
    let mut stack = vec![];
    let mut call_stack = vec![];
    let mut vars = HashMap::new();

    /*
    @0:
        print (call @1 [5, 1])
        hlt
    @1:
        store 0 $0
        store 1 $1
        jump_if @2 (neq? 0 (load 0))
        ret [load 1]
    @2:
        ret (call @1 (sub (load 0) 1) (mul (load 1) (load 0)))
    */

    /*
    @0:
        push 1
        push 5
        push @1
        call
        print
        halt
    @1:
        push 0
        store
        push 1
        store
        push 0
        load
        push 0
        neq?
        push @2
        jump_if
        push 0
        ret
    @2:
        push 0
        load
        push 1
        load
        mul
        push 1
        push 0
        load
        sub
        push @1
        call
        ret
    */
    let bc: Vec<u8> = vec![
        // @0:
        1, 1, // push 1
        1, 5, // push 5
        1, 9,  // push @1
        13, // call
        15, // print
        20, // halt
        // @1:
        1, 0, // push 0
        3, // store
        1, 1, // push 1
        3, // store
        1, 0, // push 0
        2, // load
        1, 0,  // push 0
        23, // neq?
        1, 28, // push @2
        16, // jump_if
        1, 1,  // push 1
        2,  // load
        14, // ret
        // @2:
        1, 0, // push 0
        2, // load
        1, 1, // push 1
        2, // load
        6, // mul
        1, 1, // push 1
        1, 0, // push 0
        2, // load
        5, // sub
        1, 9,  // push @1
        13, // call
        14, // ret
    ];
    loop {
        let opcode = bc.get(ip as usize);
        if opcode.is_none() {
            break;
        }
        ip += 1;
        let opcode = match opcode.unwrap() {
            0 => Opcode::Pop,
            1 => {
                let imm8 = bc.get(ip as usize).expect("immediate arg to push");
                ip += 1;
                Opcode::Push(*imm8)
            }
            2 => Opcode::Load,
            3 => Opcode::Store,
            4 => Opcode::Add,
            5 => Opcode::Sub,
            6 => Opcode::Mul,
            7 => Opcode::Div,
            8 => Opcode::Lt,
            9 => Opcode::LtEq,
            10 => Opcode::Gt,
            11 => Opcode::GtEq,
            12 => Opcode::Jump,
            13 => Opcode::Call,
            14 => Opcode::Ret,
            15 => Opcode::Print,
            16 => Opcode::JumpIf,
            17 => Opcode::BoolAnd,
            18 => Opcode::BoolOr,
            19 => Opcode::BoolNot,
            20 => Opcode::Halt,
            21 => Opcode::Swap,
            22 => Opcode::Eq,
            23 => Opcode::Neq,
            opcode => panic!("invalid or unimplemented instruction {}", opcode),
        };
        if !opcode.execute(&mut ip, &mut stack, &mut vars, &mut call_stack) {
            break;
        }
    }
}
