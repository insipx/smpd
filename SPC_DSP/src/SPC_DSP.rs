use voice::Voice;
use state::State;


pub const SPC_NO_COPY_STATE_FUNCS:isize = 1;
pub const SPC_LESS_ACCURATE:isize = 1;

struct Sizes;
impl Sizes {
    const VOICE_COUNT:u8 = 8;
    const REGISTER_COUNT:u8 = 128;
    const ECHO_HIST_SIZE:u8 = 8;
    const BRR_BUF_SIZE:u8 = 12;
    const EXTRA_SIZE:u8 = 16;
}

#[derive(Clone, Copy)]
enum globalRegisters {
    r_mvoll = 0x0C,
    r_mvolr = 0x1C,
    r_evoll = 0x2C,
    r_evolr = 0x3C,
    r_kon = 0x4C,
    r_koff = 0x5C,
    r_flg = 0x6C,
    r_endx = 0x7C,
    r_efb = 0x0D,
    r_pmon = 0x2D,
    r_non = 0x3D,
    r_eon = 0x4D,
    r_dir = 0x5D,
    r_esa = 0x6D,
    r_edl = 0x7D,
    r_fir = 0x0F, // 8 coefficients at 0x0F, 0x1F ... 0x7F
}

#[derive(Clone, Copy)]
enum voiceRegisters {
    v_voll = 0x00,
    v_volr = 0x01,
    v_pitchl = 0x02,
    v_pitchh = 0x03,
    v_srcn = 0x04,
    v_adsr0 = 0x05,
    v_adsr1 = 0x06,
    v_gain = 0x07,
    v_envx = 0x08,
    v_outx = 0x09,
}

#[derive(Clone, Copy)]
enum envMode {
    env_release,
    env_attack,
    env_decay,
    env_sustain,
}

//TODO: This probably won't work, but it's a start
pub trait Emulator {
//Setup
    fn init(ram_64K: &mut u32);
    fn set_output(sample_t: i16);
    fn sample_count() -> isize {
        return State::out - State::out_begin; 
    }
    //resets DSP to power-on state
// Emulation 
    fn reset();
    //Emulates pressing reset switch on SNES
    fn soft_reset();
    // Reads/writes DSP registers. For accuracy, you must first call spc_run_dsp()
    fn read(addr: isize) -> isize {
        assert!(addr < Sizes::REGISTER_COUNT as isize); 
        return State::regs[addr];
    }
    
    //won't work either. Need an init/create func to create the 
    //structs we are going to modify
    //i'm just going straight from C++ 
    fn write(addr: isize, data: isize, state: &mut State) {
        assert!(addr < Sizes::REGISTER_COUNT as isize);
        state.regs[addr] = data as u8;
        let low:isize = addr & 0x0F;
        
        // voice volumes
        if  low < 0x2 {
            Emulator::update_voice_vol( low ^ addr, state);
        } else if  low == 0xC {
            if addr == globalRegisters::r_kon as isize {
                state.new_kon = data as u8;
            } 
            
            // always cleared, regardless of data written
            if addr == globalRegisters::r_endx as isize {
                state.regs[globalRegisters::r_endx] = 0; 
            }
        }
    }

    // Runs DPS for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    fn run(clock_count: isize);

// Sound control
    fn mute_voices(mask: isize);
    fn disable_surround(disable: bool, state: &mut State) {
        if disable {
            state.surround_threshold = 0; 
        } else {
            state.surround_threshold = -0x4000; 
        }
    }

// State
    fn load(regs: &mut [u8]);
    fn out_pos() -> u8;
    fn extra() -> u8;

    fn init_counter();
    fn run_count();
    fn soft_reset_common();
    fn write_outline(addr: isize, data:isize);

    //TODO: no way will this work, using it as a basis
    fn update_voice_vol(addr: isize, state: &mut State) {
        let mut l = state.regs[addr + voiceRegisters::v_voll];
        let mut r = state.regs[addr + voiceRegisters::v_volr];
        if l*r < state.surround_threshold {
            //signs differ, so negate those that are negative
            l ^= l >> 7;
            r ^= r >> 7;
        }
        let &mut v:Voice = state.voices[addr >> 4];
        let enabled:isize = v.enabled;
        v.volume[0] = l & enabled;
        v.volume[1] = r & enabled;
    }
}
