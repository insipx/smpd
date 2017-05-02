use registers::globalRegisters;
use registers::voiceRegisters;
use registers::envMode;
use sizes::Sizes;
use state::State;

pub const SPC_NO_COPY_STATE_FUNCS: isize = 1;
pub const SPC_LESS_ACCURATE: isize = 1;


pub struct Voice {
    // decoded samples. should be twice the size to simplify wrap handling
    buf: [isize; (Sizes::BRR_BUF_SIZE * 2) as usize],
    buf_pos: isize, // place in buffer where next samples will be decoded
    interp_pos: isize, // relative fractional positoin in sample (0x1000 = 1.0)
    brr_addr: isize, // address of current BRR block
    brr_offset: isize, // current decoding offset in BRR block
    kon_delay: isize, // KON delay/current setup phase
    env_mode: envMode,
    env: isize, // current envelope level
    hidden_env: isize, // used by GAIN mode 7, obscure quirk
    pub volume: [isize; 2], // copy of volume from DSP registers, with surround disabled
    pub enabled: isize, // -1 if enabled, 0 if muted
                    //TODO: Consider changing enabled to bool
}


//TODO: This probably won't work, but it's a start
pub trait Emulator {
    //Setup
    fn init(ram_64K: &mut u32);
    fn set_output(sample_t: i16);

    //resets DSP to power-on state
    // Emulation
    fn reset();
    //Emulates pressing reset switch on SNES
    fn soft_reset();
    // Reads/writes DSP registers. For accuracy, you must first call spc_run_dsp()

    // Runs DSP for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    fn run(clock_count: isize);

    // Sound control
    fn mute_voices(mask: isize);
}
