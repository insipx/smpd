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

