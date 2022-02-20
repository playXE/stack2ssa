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
        Op::LSet(1),
        Op::LGet(0),
        Op::LGet(1),
        Op::Add,
        Op::Ret,
    ];

    let code = jit.compile(&code, 2, 2);
    let func = unsafe { std::mem::transmute::<_, extern "C" fn(i32, i32) -> i32>(code) };

    println!("{}", func(5, 4));
}
