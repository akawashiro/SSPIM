type RegId                = usize;
type Word                 = i32;
type Const                = i32;
const N_REGISTER : usize  = 32;
const MEMORY_SIZE : usize = (1<<16);

struct MachineState{
    register:[Word; N_REGISTER ],
    memory:[Word; MEMORY_SIZE ]
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
    Addi(RegId,  RegId, Const),
    Addiu(RegId, RegId, Const),
    Slti(RegId,  RegId, Const),
    Sltiu(RegId, RegId, Const),
    Andi(RegId,  RegId, Const),
    Ori(RegId,   RegId, Const),
    Xori(RegId,  RegId, Const),
    Lui(RegId,   Const)
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
        Nor(r1,r2,r3)  => ms.register[r1] = !(ms.register[r2] | ms.register[r3]),
        Slt(r1,r2,r3)  => ms.register[r1] = (ms.register[r2] < ms.register[r3]) as Word,
        Sltu(r1,r2,r3) => ms.register[r1] = (ms.register[r2] < ms.register[r3]) as Word,
        Addi(r1,r2,c)  => ms.register[r1] = ms.register[r2] + c,
        Addiu(r1,r2,c) => ms.register[r1] = ms.register[r2] + c,
        Slti(r1,r2,c)  => ms.register[r1] = (ms.register[r2] < c) as Word,
        Sltiu(r1,r2,c) => ms.register[r1] = (ms.register[r2] < c) as Word,
        Andi(r1,r2,c)  => ms.register[r1] = ms.register[r2] & c,
        Ori(r1,r2,c)   => ms.register[r1] = ms.register[r2] | c,
        Xori(r1,r2,c)  => ms.register[r1] = ms.register[r2] ^ c,
        Lui(r1,c)      => ms.register[r1] |= c << 16,
    }
}

fn show(ms: &MachineState){
    for i in 0..N_REGISTER {
        print!("register[{}]={} ",i, ms.register[i]);
        if i%4 == 3 {
            print!("\n");
        }
    }
}

fn main() {
    use Instruction::*;
    let mut ms = MachineState { register : [0; 32], memory : [0; (1<<16)] };
    execute(&mut ms, Addi(0,1,10));
    show(&ms);
}

