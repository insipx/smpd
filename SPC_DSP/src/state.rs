use sizes::Sizes;
use registers::globalRegisters;
use registers::voiceRegisters;
use registers::envMode;

struct State<'a> {
    //TODO
    regs: [u8; Sizes::REGISTER_COUNT as usize ],
    echo_hist: [[&'a mut isize; (Sizes::ECHO_HIST_SIZE*2) as usize]; 2],
    // *echo_hist_pos[2]
    every_other_sample: isize,
    kon: isize,
    noise: isize,
    echo_offset: isize,
    echo_length: isize,
    phase: isize,
    counters: [&'a mut usize; 4],
    new_kon: isize,
    t_koff: isize,
    voices: [Voice; Sizes::VOICE_COUNT as usize],
    counter_select: [&'a mut usize; 32],
    ram: &'a mut u8, // 64K shared RAM between DSP and SMP
    mute_mask: isize,
    surround_threshold: isize,
    out: &'a i8,
    out_end: &'a mut i8,
    out_begin: &'a mut i8,
    extra: [i8; Sizes::EXTRA_SIZE as usize],
}

impl State<'a> {
    
    fn load(regs: &mut [u8]) {
        return regs; 
    }
   
    fn extra(&self) -> u8 {
        return self.extra; 
    }

    fn out_pos(&self) -> u8 {
        return self.out; 
    }

    fn init_counter(&mut self) {
        self.counters [0] =     1; 
        self.counters [1] =     0;
        self.counters [2] = -0x20u;
        self.counters [3] =  0x0B;

        let mut n = 2;

        for i in 0..32 {
            self.counter_select [i] = &self.counters[n];
            if !--n {
                n = 3; 
            }
        }
        self.counter_select [0] = &self.counters [0];
        self.counter_select [0] = &self.counters [2];
    }

    fn run_counter(&self, i: isize) {
        let mut n = self.counters[i];

        if !(n-- & 7) {
            n -= 6 -i; 
        }

        self.counters[i] = n;
    }

    fn soft_reset_common(&mut self) {
        // require (m.ram)
        self.noise              = 0x4000;
        self.echo_hist_pos      = self.echo_hist;
        self.every_other_sample = 1;
        self.echo_offset        = 0;
        self.phase              = 0;

        self.init_counter();
    }
    
    // don't need this?
    /* fn write_outline(addr: isize, data: isize); */

    //TODO: no way will this work, using it as a basis
    fn update_voice_vol(&mut Self, addr: isize) {
        let mut l = &self.regs[(addr + voiceRegisters::v_voll as isize) as usize];
        let mut r = &self.regs[(addr + voiceRegisters::v_volr as isize) as usize];
        if l * r < &self.surround_threshold {
            //signs differ, so negate those that are negative
            l ^= l >> 7;
            r ^= r >> 7;
        }
        let v = &self.voices[(addr >> 4) as usize];
        let enabled: isize = v.enabled;
        v.volume[0] = l & enabled;
        v.volume[1] = r & enabled;
    }

    fn disable_surround(disable: bool, state: &mut State) {
        if disable {
            state.surround_threshold = 0;
        } else {
            state.surround_threshold = -0x4000;
        }
    }


}
