use registers::EnvMode;
use sizes::Sizes;
use state::State;

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
pub trait Emulator {
    
    static const m:State;

    fn init(&mut self, ram_64K: u32);

    fn load(&mut self, regs: [u8; Sizes::REGISTER_COUNT as usize]) -> &mut [u8] {

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


impl Voice for Emulator {

    fn init(&mut self, ram_64K: u32) {
        self.ram = ram_64K; 
        self.mute_voices(0);
        self.disable_surround(false);
        self.set_output(0,0);
        self.reset();

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

    pub fn load(&mut self, regs: [u8; Sizes::REGISTER_COUNT as usize]) -> &mut [u8] {
        self.regs = regs;

        let mut i:isize;
        //be careful here
        for i in (0..Sizes::VOICE_COUNT).rev() {
            self.voices[i].brr_offset = 1;
            self.voices[i].buf_pos = &self.voices[i].buf;
        }
        self.new_kon = reg!(kon);
        
        self.mute_voices( self.mute_mask );
        self.soft_reset_common();
    }
}



