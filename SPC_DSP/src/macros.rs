use state::State;
use SPC_DSP::counter_mask as counter_mask;

macro_rules! clamp16 {
    ( $io:expr ) => {
        {
            if ($io as i16) != $io {
                $io = ($io >> 31) ^ 0x7FFF; 
            }
        }
    };
}

macro_rules! read_counter {
    ( $rate:expr, $state:expr) => {
        (*$state.counter_select[$rate] & counter_mask![$rate])
    }
}

//TODO some tricks because you can't use if-else in static invocation
//will eventually be fixed in Rust
//but for now hacky implementation
macro_rules! rate {
   ( $rate:expr, $div:expr ) => {
        (
            ($rate >= $div) as i32 * ($rate / $div * 8 - 1) +
            ($rate <  $div) as i32 * ($rate - 1)
        ) as u32
   }
}

// m.foo = regs[GlobalRegisters::r_kon] 
macro_rules! reg {
    ($n:path) => {
        {   
            m.regs[GlobalRegisters::concat_idents!(r_, $n)]
        }
    }
}
