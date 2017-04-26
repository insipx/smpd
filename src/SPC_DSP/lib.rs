
struct Voice {
    but: [isize; SPC_DSP::brrBufSize*2],      // decoded samples. should be twice the size to simplify wrap handling 
    buf_pos: isize,         // place in buffer where next samples will be decoded
    interp_pos: isize,      // relative fractional positoin in sample (0x1000 = 1.0)
    brr_addr: isize,        // address of current BRR block
    brr_offset: isize,      // current decoding offset in BRR block
    kon_delay: isize,       // KON delay/current setup phase
    env_mode: envMode,
    env: isize,             // current envelope level
    hidden_env: isize,      // used by GAIN mode 7, obscure quirk
    volume: [isize; 2],   // copy of volume from DSP registers, with surround disabled
    enabled: isize,         // -1 if enabled, 0 if muted
    //TODO: Consider changing enabled to bool
}

struct State { //TODO
    regs: [&mut u8],
}

enum globalRegisters 
{
    r_mvoll = 0x0C, r_mvolr = 0x1C,
    r_evoll = 0x2C, r_evolr = 0x3C,
    r_kon   = 0x4C, r_koff  = 0x5C,
    r_flg   = 0x6C, r_endx  = 0x7C,
    r_efb   = 0x0D, r_pmon  = 0x2D,
    r_non   = 0x3D, r_eon   = 0x4D,
    r_dir   = 0x5D, r_esa   = 0x6D,
    r_edl   = 0x7D,
    r_fir   = 0x0F // 8 coefficients at 0x0F, 0x1F ... 0x7F
}

enum voiceRegisters
{
    v_voll   = 0x00, v_volr   = 0x01,
    v_pitchl = 0x02, v_pitchh = 0x03,
    v_srcn   = 0x04, v_adsr0  = 0x05,
    v_adsr1  = 0x06, v_gain   = 0x07,
    v_envx   = 0x08, v_outx   = 0x09
}

enum envMode
{
    envRelease,
    envAttack,
    envDecay,
    envSustain
}

enum SPC_DSP 
{
   voiceCount = 8,
   registerCount = 128,
   extraSize = 16,
   echoHistSize = 8,
   brrBufSize = 12,
   extraSize = 16,
}

//TODO: This probably won't work, but it's a start
trait SPC 
{
    pub fn init(&mut ram_64K:u32 );
    pub fn set_output(sample_t:i16);
    pub fn sample_count() -> isize;

    //resets DSP to power-on state
    pub fn reset();

    //Emulates pressing reset switch on SNES
    pub fn soft_reset();
    
    // Reads/writes DSP registers. For accuracy, you must first call spc_run_dsp()
    pub fn read(addr:isize) -> isize;
    pub fn write(addr:isize, data:isize);
    
    // Runs DPS for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    pub fn run(clock_count:isize);
    
// Sound control
    pub fn mute_voices(mask:isize);
    pub fn disable_surround(disable:bool);
    
// State
    pub fn load(regs: &mut [u8]);
    pub fn extra() -> u8;
    pub fn out_pos() -> u8;
    pub fn extra() -> u8;

//private
    fn init_counter();
    fn run_counter();
    fn soft_reset_common();
    fn write_outline(addr:isize, data:isize);
    fn update_voice_vol(addr:isize);
}

impl SPC for SPC_DSP 
{

}
