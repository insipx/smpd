pub mod SPC_DSP {
    
    pub const SPC_NO_COPY_STATE_FUNCS = 1;
    pub const SPC_LESS_ACCURATE = 1;

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

    enum envMode {
        env_release,
        env_attack,
        env_decay,
        env_sustain,
    }

    enum Sizes {
        voice_count = 8,
        register_count = 128,
        extra_size = 16,
        echo_hist_size = 8,
        brr_buf_size = 12,
        extra_size = 16,
    }

    struct Voice {
        // decoded samples. should be twice the size to simplify wrap handling
        but: [isize; Sizes::brr_buf_size * 2],
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

    struct State {
        //TODO
        regs: [&mut u8; Sizes::register_count],
        echo_hist: [[&mut isize: Sizes::echo_hist_size*2]; 2],
        // *echo_hist_pos[2]
        every_other_sample: isize,
        kon: isize,
        noise: isize,
        echo_offset: isize,
        echo_length: isize,
        phase: isize,
        counters: [&mut usize; 4],
        new_kon: isize,
        t_koff: isize,
        voices: [Voice: Sizes::voice_count],
        counter_select: [&mut usize; 32],
        ram: &mut u8, // 64K shared RAM between DSP and SMP
        mute_mask: isize,
        surround_threshold: isize,
        out: &mut i8,
        out_end: &mut i8,
        out_begin: &mut i8,
        extra: [i8: Sizes::extra_size],
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
            assert!(addr < Sizes::register_count); 
            return State::regs[addr];
        }
        
        //won't work either. Need an init/create func to create the 
        //structs we are going to modify
        //i'm just going straight from C++ 
        fn write(addr: isize, data: isize) {
            assert!(addr < Sizes::register_count);
            State::regs[addr] = data as u8;
            let low:isize = addr & 0x0F;
            
            // voice volumes
            if  low < 0x2 {
                update_voice_vol( low ^ addr);
            } else if ( low == 0xC) {
                if addr == globalRegisters::r_kon {
                    State::new_kon = data as u8;
                } 
                
                // always cleared, regardless of data written
                if addr == globalRegisters::r_endx {
                    State::regs[globalRegisters::r_endx] = 0; 
                }
            }
        }

        // Runs DPS for specified number of clocks (~1024000 per second). Every 32 clocks
        // a pair of samples is to be generated
        fn run(clock_count: isize);

    // Sound control
        fn mute_voices(mask: isize);
        fn disable_surround(disable: bool) {
            if disable {
                State::surround_threshold = 0; 
            } else {
                State::surround_threshold = -0x4000; 
            }
        }

    // State
        fn load(regs: &mut [u8]);
        fn extra() -> u8;
        fn out_pos() -> u8;
        fn extra() -> u8;

        fn init_counter();
        fn run_count();
        fn soft_reset_common();
        fn write_outline(addr: isize, data:isize);

        //TODO: no way will this work, using it as a basis
        fn update_voice_vol(addr: isize) {
            let mut l = State::regs[addr + voiceRegisters::v_voll];
            let mut r = State::regs[addr + voiceRegisters::v_volr];
            if l*r < State::surround_threshold {
                //signs differ, so negate those that are negative
                l ^= l >> 7;
                r ^= r >> 7;
            }

            let &mut v:State::voices[addr >> 4];
            let enabled:isize = v.enabled;
            v.volume[0] = l & enabled;
            v.volume[1] = r & enabled;
        }
    }
}
