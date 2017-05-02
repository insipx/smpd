use registers::EnvMode;
use sizes::Sizes;
use state::State;

pub const SPC_NO_COPY_STATE_FUNCS: isize = 1;
pub const SPC_LESS_ACCURATE: isize = 1;

//TODO some tricks because you can't use if-else in static invocation
//will eventually be fixed in Rust
//but for now hacky implementation
#[macro_use]
macro_rules! rate {
   ( $rate:expr, $div:expr ) => {
        (
            ($rate >= $div) as i32 * ($rate / $div * 8 - 1) +
            ($rate <  $div) as i32 * ($rate - 1)
        ) as u32
   }
}

pub static counter_mask: [u32; 32] =
[
	rate!(   2,2), rate!(2048,4), rate!(1536,3),
	rate!(1280,5), rate!(1024,4), rate!( 768,3),
	rate!( 640,5), rate!( 512,4), rate!( 384,3),
	rate!( 320,5), rate!( 256,4), rate!( 192,3),
	rate!( 160,5), rate!( 128,4), rate!(  96,3),
	rate!(  80,5), rate!(  64,4), rate!(  48,3),
	rate!(  40,5), rate!(  32,4), rate!(  24,3),
	rate!(  20,5), rate!(  16,4), rate!(  12,3),
	rate!(  10,5), rate!(   8,4), rate!(   6,3),
	rate!(   5,5), rate!(   4,4), rate!(   3,3),
	               rate!(   2,4),
	               rate!(   1,4)
];

pub struct Voice<'a> {
    // decoded samples. should be twice the size to simplify wrap handling
    buf: [isize; (Sizes::BRR_BUF_SIZE * 2) as usize],
    buf_pos: isize, // place in buffer where next samples will be decoded
    interp_pos: isize, // relative fractional positoin in sample (0x1000 = 1.0)
    brr_addr: isize, // address of current BRR block
    brr_offset: isize, // current decoding offset in BRR block
    kon_delay: isize, // KON delay/current setup phase
    env_mode: EnvMode,
    env: isize, // current envelope level
    hidden_env: isize, // used by GAIN mode 7, obscure quirk
    pub volume: [&'a mut isize; 2], // copy of volume from DSP registers, with surround disabled
    pub enabled: isize, // -1 if enabled, 0 if muted
                    //TODO: Consider changing enabled to bool
}


//TODO: This probably will work, but it's organization sucks, I think.
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
}


