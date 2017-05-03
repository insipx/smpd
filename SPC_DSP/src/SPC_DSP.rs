use registers::GlobalRegisters;
use registers::EnvMode;
use sizes::Sizes;
use state::State;
use macros;

//TODO some tricks because you can't use if-else in static invocation
//will eventually be fixed in Rust
//but for now hacky implementation

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
    pub buf_pos: isize, // place in buffer where next samples will be decoded
    interp_pos: isize, // relative fractional positoin in sample (0x1000 = 1.0)
    brr_addr: isize, // address of current BRR block
    pub brr_offset: isize, // current decoding offset in BRR block
    kon_delay: isize, // KON delay/current setup phase
    env_mode: EnvMode,
    env: isize, // current envelope level
    hidden_env: isize, // used by GAIN mode 7, obscure quirk
    pub volume: [&'a mut isize; 2], // copy of volume from DSP registers, with surround disabled
    pub enabled: isize, // -1 if enabled, 0 if muted
                    //TODO: Consider changing enabled to bool
}


//TODO: This probably will work, but it's organization sucks, I think.
pub trait Emulator<'a> {
    
    const m:State<'a>;

    fn init(&self, ram_64K: u32);

    fn load(&mut self, regs: [u8; Sizes::REGISTER_COUNT as usize]);

    // Runs DSP for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    fn run(clock_count: isize);
}


impl<'a> Emulator<'a> for Voice<'a> {
   
    //global state
    const m:State<'a> = state::create();

    fn init(&self, ram_64K: u32) {
        Self::m.ram = ram_64K; 
        Self::m.mute_voices(0);
        Self::m.disable_surround(false);
        Self::m.set_output(0,0);
        Self::m.reset();

        //debug
        if NDEBUG {
            assert_eq!(0x8000 as i16, -0x8000);
            assert!( (-1 >> 1) == -1 );
            let mut i:i16;
            i = 0x8000; clamp16!(i); assert!(i == 0x7FFF);
            i = -0x8001; clamp16!(i); assert!(i == -0x8000);
        }

        //SPC_DSP has a verify byte order; but i will forgo this for now
    }

    fn load(&mut self, regs: [u8; Sizes::REGISTER_COUNT as usize]) {
        m.regs = regs;

        let mut i:isize;
        //be careful here
        for i in (0..Sizes::VOICE_COUNT).rev() {
            Self::m.voices[i].brr_offset = 1;
            Self::m.voices[i].buf_pos = &Self::m.voices[i].buf;
        }
        m.new_kon = reg!(kon, m);
        
        m.mute_voices( m.mute_mask );
        m.soft_reset_common();
    }

    fn run(clock_count: isize) {
        unimplemented!(); 
    }
}



