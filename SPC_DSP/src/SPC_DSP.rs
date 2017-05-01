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
    volume: [isize; 2], // copy of volume from DSP registers, with surround disabled
    enabled: isize, // -1 if enabled, 0 if muted
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
    fn read<'a>(addr: isize, state: &'a State) -> u8 {
        assert!(addr < Sizes::REGISTER_COUNT as isize);
        return state.regs[addr as usize];
    }

    //won't work either. Need an init/create func to create the
    //structs we are going to modify
    //i'm just going straight from C++
    fn write(addr: isize, data: isize, state: &mut State) {
        assert!(addr < Sizes::REGISTER_COUNT as isize);
        state.regs[addr as usize] = data as u8;
        let low: isize = addr & 0x0F;

        //voice volumes
        if low < 0x2 {
            State::update_voice_vol(low ^ addr, state);
        } else if low == 0xC {
            if addr == globalRegisters::r_kon as isize {
                state.new_kon = data;
            }

            // always cleared, regardless of data written
            if addr == globalRegisters::r_endx as isize {
                state.regs[globalRegisters::re_endx as usize] = 0; 
            }
        }
    }

    // Runs DPS for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    fn run(clock_count: isize);

    // Sound control
    fn mute_voices(mask: isize);
}
