
pub struct State {
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

