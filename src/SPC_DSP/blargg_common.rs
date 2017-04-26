//just going to forgo blargg_vec

//a macro to return four character integer constant
//i think we can get away without this
macro_rules! BLARGG_4CHAR {
    ( $( $a:tt, $b:tt, $c:tt, $d:tt) => {
        {a, b, c, d}
    });
}
