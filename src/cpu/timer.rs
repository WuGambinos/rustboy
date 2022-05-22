

pub struct Clock {
    main: u8,
    sub: u8,
    div: u8,
}

pub struct Reg {
    /// DIV (Divider Register)
    div: u8,
    /// TIMA (Timer Counter)
    tima: u8,
    /// TMA (Timer Modulo)
    tma: u8,
    /// TAC (Timer Control)
    tac: u8,
}

pub struct Timer {
    clock: Clock,
    reg: Reg,
}


impl Timer {
    fn new(&self) -> Self {
        Self {
            clock: Clock { main: 0, sub: 0, div: 0 },
            reg: Reg { div: 0, tima: 0, tma: 0, tac: 0 },
        }
    }

    fn inc(&self)  {
        //Increment by  the last opcode's time

        //No opcodes take longer then 4 M-times
        //So we only need to check for overflow once

    }

}