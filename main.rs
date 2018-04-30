type RegId = usize;
type Word = i32;

struct MachineState{
    register:[Word; 32 ],
    memory:[Word; (1<<16) ]
}

enum Instruction{
    Add(RegId,   RegId, RegId),
    Addu(RegId,  RegId, RegId),
    Sub(RegId,   RegId, RegId),
    Subu(RegId,  RegId, RegId),
    And(RegId,   RegId, RegId),
    Or(RegId,    RegId, RegId),
    Xor(RegId,   RegId, RegId),
    Nor(RegId,   RegId, RegId),
    Slt(RegId,   RegId, RegId),
    Sltu(RegId,  RegId, RegId),
    Addi(RegId,  RegId, RegId),
    Addiu(RegId, RegId, RegId),
    Slti(RegId,  RegId, RegId),
    Sltiu(RegId, RegId, RegId),
    Andi(RegId,  RegId, RegId),
    Ori(RegId,   RegId, RegId),
    Xori(RegId,  RegId, RegId),
    Lui(RegId,   RegId, RegId)
}

fn execute(ms: &mut MachineState, ist: Instruction){
    use Instruction::*;
    match ist{
        Add(r1,r2,r3)  => ms.register[r1] = ms.register[r2] + ms.register[r3],
        Addu(r1,r2,r3) => ms.register[r1] = ms.register[r2] + ms.register[r3],
        Sub(r1,r2,r3)  => ms.register[r1] = ms.register[r2] - ms.register[r3],
        Subu(r1,r2,r3) => ms.register[r1] = ms.register[r2] - ms.register[r3],
        And(r1,r2,r3)  => ms.register[r1] = ms.register[r2] & ms.register[r3],
        Or(r1,r2,r3)   => ms.register[r1] = ms.register[r2] | ms.register[r3],
        Xor(r1,r2,r3)  => ms.register[r1] = ms.register[r2] ^ ms.register[r3],
        _              => println!("Hello")
    }
}

fn main() {
    let mut ms = MachineState { register : [0; 32], memory : [0; (1<<16)] };
    ms.register[1] = 10;
    let i = Instruction::Add(1,1,1);
    println!("Hello")
}

