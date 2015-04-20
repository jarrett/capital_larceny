#![macro_use]

macro_rules! tryln {
    ($expr:expr) => (try!(
        ($expr).map_err({ |e|
            (e, file!(), line!())
        })
    ))
}