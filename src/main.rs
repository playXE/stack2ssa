use stack2ssa::{lower::JIT, opcodes::Op};

fn pow2(mut x: i32) -> i32 {
    let mut acc = 0;

    for _ in 0..x {
        acc = acc * x;
    }
    acc
}

fn main() {
    let mut jit = JIT::new();

    let code = vec![
        Op::LSet(0),
        Op::Push(1),
        Op::LSet(1),
        Op::Push(2),
        Op::LSet(2),
        // loop condition
        Op::LGet(0),
        Op::LGet(2),
        Op::Le,
        Op::JumpIfZero(18),
        // loop body
        Op::LGet(2),
        Op::LGet(1),
        Op::Mul,
        Op::LSet(1),
        Op::LGet(2),
        Op::Push(1),
        Op::Add,
        Op::LSet(2),
        Op::Jump(5),
        // loop end
        Op::LGet(1),
        Op::Ret,
    ];

    let code = jit.compile(&code, 1, 3);
    let func = unsafe { std::mem::transmute::<_, extern "C" fn(i32) -> i32>(code) };

    println!("{}", func(5));
}
