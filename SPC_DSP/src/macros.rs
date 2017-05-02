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

