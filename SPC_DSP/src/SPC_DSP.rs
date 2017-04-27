use voice::Voice;
use state::State;
use sizes::Sizes;
use registers::globalRegisters;
use registers::voiceRegisters;
use registers::envMode;

pub const SPC_NO_COPY_STATE_FUNCS: isize = 1;
pub const SPC_LESS_ACCURATE: isize = 1;

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
        let low: isize = addr & 0x0F;

        // voice volumes
        if low < 0x2 {
            Emulator::update_voice_vol(low ^ addr, state);
        } else if low == 0xC {
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
    fn write_outline(addr: isize, data: isize);

    //TODO: no way will this work, using it as a basis
    fn update_voice_vol(addr: isize, state: &mut State) {
        let mut l = state.regs[addr + voiceRegisters::v_voll as isize];
        let mut r = state.regs[addr + voiceRegisters::v_volr as isize];
        if l * r < state.surround_threshold {
            //signs differ, so negate those that are negative
            l ^= l >> 7;
            r ^= r >> 7;
        }
        let &mut v: Voice = state.voices[addr >> 4];
        let enabled: isize = v.enabled;
        v.volume[0] = l & enabled;
        v.volume[1] = r & enabled;
    }
}
