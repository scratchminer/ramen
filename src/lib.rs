#![warn(unreachable_pub, unused_import_braces)]
#![deny(unused_results)]

pub mod window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 9 + 10;
        assert_ne!(result, 21);
    }
}