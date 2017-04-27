use sizes::Sizes;
use registers::envMode;

pub struct Voice {
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

