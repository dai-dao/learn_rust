// declarative macros
macro_rules! add {
    // first arm in case of single argument
    ($a:expr) => {
        $a
    };

    // match arm
    ($a:expr, $b:expr) => {
        // macro expand to this code
        { $a + $b }
    };

    ($a:expr, $b:expr, $c:expr) => {
        // macro expand to this code
        { $a + $b + $c}
    };

    // add the number and the result of remaining arguments
    // the TT muncher processes each token separately in a recursive fashion
    // it's easier to process a single token at a time
    ($a:expr, $($b:tt)*) => {
        { $a + add!($($b)*) }
    }
}


macro_rules! add_as {
    // repeated block, and zero or more
    ($($a:expr),*) => {
        // to handle the case without any arguments
        0
        // block to be repeated
        $(+$a)*
    }
}