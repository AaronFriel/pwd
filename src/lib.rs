#[cfg(not(windows))]
#[macro_use] extern crate error_chain;
#[cfg(not(windows))]
extern crate libc;

#[cfg(not(windows))]
pub use unix::*;

#[cfg(not(windows))]
mod errors;
#[cfg(not(windows))]
mod unix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
