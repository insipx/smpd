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

macro_rules! rate {
   ( $rate:expr, $div:expr ) => {
        (
            ($rate >= $div) as i32 * ($rate / $div * 8 - 1) +
            ($rate <  $div) as i32 * ($rate - 1)
        ) as u32
   }
}

// m.foo = (self|m).regs[GlobalRegisters::r_kon] 
macro_rules! reg {
    ($n:ident, $m:path) => {
        $m.regs[concat_idents!(r_, $n)]
    }
}

