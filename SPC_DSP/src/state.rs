use registers::globalRegisters;
use registers::voiceRegisters;
use registers::envMode;
use sizes::Sizes;
use SPC_DSP::Voice;


pub struct State<'a> {
    //TODO
    regs: [u8; Sizes::REGISTER_COUNT as usize ],
    echo_hist: [[&'a mut isize; 2]; (Sizes::ECHO_HIST_SIZE*2) as usize],
    /*echo_hist_pos: [&'a mut isize; 2], //&echo hist[0 to 7]*/ //ignoring this for now
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

impl<'a> State<'a> {
    
    pub fn load(regs: &mut [u8]) -> &mut [u8]{
        return regs; 
    }
   
    pub fn extra(&self) -> &[u8] {
        return self.extra; 
    }

    pub fn out_pos(&self) -> i8 {
        return *self.out; 
    }

    fn sample_count(&self) -> isize {
        return *self.out as isize - *self.out_begin as isize;
    }

    fn read(&self, addr: isize) -> u8 {
        assert!(addr < Sizes::REGISTER_COUNT as isize);
        return self.regs[addr as usize];
    }

    //won't work either. Need an init/create func to create the
    //structs we are going to modify
    //i'm just going straight from C++
    fn write(&mut self, addr: isize, data: isize) {
        assert!(addr < Sizes::REGISTER_COUNT as isize);
        self.regs[addr as usize] = data as u8;
        let low: isize = addr & 0x0F;

        //voice volumes
        if low < 0x2 {
            self.update_voice_vol(low ^ addr);
        } else if low == 0xC {
            if addr == globalRegisters::r_kon as isize {
                self.new_kon = data;
            }

            // always cleared, regardless of data written
            if addr == globalRegisters::r_endx as isize {
                self.regs[globalRegisters::r_endx as usize] = 0; 
            }
        }
    }


    pub fn init_counter(&mut self) {
        *self.counters [0] =     1; 
        *self.counters [1] =     0;
        *self.counters [2] = (!0) << 5; // FFFFFFE0 ie: 4 bytes, last 5 bits 0
        *self.counters [3] =  0x0B;

        let mut n = 2;

        for i in 0..32 {

            self.counter_select [i] = &mut self.counters[n];
            
            //TODO: Make sure this is OK
            n -= 1;
            if n == 0 {
                n = 3; 
            }
        }
        self.counter_select [0] = &mut self.counters [0];
        self.counter_select [0] = &mut self.counters [2];
    }

    pub fn run_counter(&self, i: isize) {
        let mut n = self.counters[i];
        
        //TODO make sure this is OK
        if !(n & 7) {
            n -= 6-i; 
        }
        n-=1;

        self.counters[i as usize] = n;
    }

    pub fn soft_reset_common(&mut self) {
        // require (m.ram)
        self.noise              = 0x4000;
        /* *self.echo_hist_pos      = self.echo_hist; //TODO not sure if right */ // ignoring this until further notice
        self.every_other_sample = 1;
        self.echo_offset        = 0;
        self.phase              = 0;

        self.init_counter();
    }
    
    // don't need this?
    /* fn write_outline(addr: isize, data: isize); */

    //TODO: no way will this work, using it as a basis
    pub fn update_voice_vol(&mut self, addr: isize) {
        let mut l:isize = self.regs[(addr + voiceRegisters::v_voll as isize) as usize] as isize;
        let mut r:isize = self.regs[(addr + voiceRegisters::v_volr as isize) as usize] as isize;
        if l * r < self.surround_threshold {
            //signs differ, so negate those that are negative
            l ^= l >> 7;
            r ^= r >> 7;
        }
        let v = &self.voices[(addr >> 4) as usize];
        let enabled: isize = v.enabled;
        v.volume[0] = (l as isize) & enabled;
        v.volume[1] = (r as isize) & enabled;
    }

    pub fn disable_surround(disable: bool, state: &mut State) {
        if disable {
            state.surround_threshold = 0;
        } else {
            state.surround_threshold = -0x4000;
        }
    }


}
