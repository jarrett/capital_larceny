#![macro_use]

macro_rules! tryln {
    ($expr:expr) => (try!(
        ($expr).map_err({ |e|
            (e, file!(), line!())
        })
    ))
}

// Example usage:
// 
// impl MyEnum {
//     pub fn my_func(&self, param1: i32, param2: i32) -> i32 {
//         dispatch_enum!(my_func, [Variant1, Variant2], [param1, param2], default_handler_func)
//     }
// }
// 
// Expands to:
// 
// impl MyEnum {
//     pub fn my_func(&self, param1: i32, param2: i32) -> i32 {
//         match *self {
//             variant1(v) => { v.my_func(param1, param2) },
//             variant2(v) => { v.my_func(param1, param2) },
//             _           => { default_handler_func(param1, param2) }
//         }
//     }
// }
macro_rules! dispatch_enum {
    (
        $f:ident,         // Function to dispatch.
        [$($v:ident),*],  // Variants that implement this function.
        [$($p:ident),*],  // Function parameters.
        $d:path           // Default handler function for all other variants.
    ) => {
        match *self {
            $(
                $v(inner) => { inner.$f($p) }
            ),*
            _             => { $d($($p),*) }
        }
    }
}