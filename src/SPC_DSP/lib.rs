

struct SPC_DSP 
{
    
}

//TODO: i have no idea what i'm doing, this probably needs fixing
trait SPC 
{
    init(&mut ram_64K:u32 );
    set_output(sample_t:i16);
    sample_count() -> isize;

    ///resets DSP to power-on state
    reset();

    ///Emulates pressing reset switch on SNES
    soft_reset();
    
    // Reads/writes DSP registers. For accuracy, you must first call spc_run_dsp()
    read(addr:isize) -> isize;
    write(addr:isize, data:isize);
    
    // Runs DPS for specified number of clocks (~1024000 per second). Every 32 clocks
    // a pair of samples is to be generated
    run(clock_count:isize);
    
    TODO: enums
// Sound control
    
    mute_voices(mask:isize);

    disable_surround(disable:bool);
    
// State TODO: enums
    
    load(regs: &mut [u8]);
    

}

impl SPC for SPC_DSP 
{

}
