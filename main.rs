type RegId = i32
type Word = i32

struct MachineState{
    register:[Word; 32 ],
    memory:[Word; (1<<16) ]
}

enum Instruction{
    Add(RegId,RegId,RegId),
    Addu(RegId,RegId,RegId),
    Sub(RegId,RegId,RegId),
    Subu(RegId,RegId,RegId),
    And(RegId,RegId,RegId),
    Or(RegId,RegId,RegId),
    Xor(RegId,RegId,RegId),
    Nor(RegId,RegId,RegId),
    Slt(RegId,RegId,RegId),
    Sltu(RegId,RegId,RegId),
    Addi(RegId,RegId,RegId),
    Addiu(RegId,RegId,RegId),
    Slti(RegId,RegId,RegId),
    Sltiu(RegId,RegId,RegId),
    Andi(RegId,RegId,RegId),
    Ori(RegId,RegId,RegId),
    Xori(RegId,RegId,RegId),
    Lui(RegId,RegId,RegId)
}

fn main() {
    let mut ms = MachineState { register : [0; 32], memory : [0; (1<<16)] };
    ms.register[1] = 10;
    println!("Hello")
}

