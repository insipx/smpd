macro_rules! clamp16 {
    ( $io:expr ) => {
        {
            if ($io as i16) != $io {
                $io = ($io >> 31) ^ 0x7FFF; 
            }
        }
    };
}

