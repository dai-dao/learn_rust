/*
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


macro_rules! ok_or_return {
    // match someFunc(a, b, c, 1, 2, 3)
    // compiler extracts function name and arguments. it
    // injects the values in respective variables
    //  returns the function is an operation returns Err or the 
    // value of an operation returns Ok. it takes a function as argument
    // and executes it inside a match statement
    // internal rule, now the macro will never match for an internal rule
    // until explicitly specified as an argument
    (@error $a:ident,($($b:tt)*)) => {
        {
            match $a($($b)*) {
                Ok(value) => value,
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };
    // public rule can be called by the user
    ($a:ident($($b:tt)*)) => {
        ok_or_return!(@error $a,$($b)*)
    }

}


macro_rules! make_public {
    // vis type for visibility keyword and ident for struct name
    // meta data about struct
    ($(#[$meta:meta])*
    $vis:vis struct $struct_name:ident {
        $(
            // meta data about field
            $(#[$field_meta:meta])*
            // need to match the comma as well, + means zero or one
            $field_vis:vis $field_name:ident : $field_type:ty),*$(,)+
    }
    ) => {
        {
            $(#[$meta])*
            pub struct $struct_name {
                $(
                    $(#[$field_meta])*
                    pub $field_name : $field_type,)*
            }
        }
    }
}
*/


macro_rules! init_command {
    (
        // match meta data, zero or more
        $(#[$outer:meta])*
        use $cmd_name:ident 
        // match zero or one life time param
        for fn $function_name:ident $(<$($lt:lifetime)+>)? (
            // match function arguments and argument types 
            // i'll just make it super simple to start with
            $($arg_name:ident : $arg_type:ty ,)*
        ) 
        $(;)?
    ) => {
        $(#[$outer])*
        pub fn $function_name $(<$($lt)+>)? (&mut self, $($arg_name:$arg_type,)*) 
            -> $crate::result::Result<<$cmd_name as $crate::commands::StreamCommand>::Response>
             {
            // pass in arguments for the command function
            let command = $cmd_name { $($arg_name,)* };
            self.stream.run_command(command)
        }
    };
}