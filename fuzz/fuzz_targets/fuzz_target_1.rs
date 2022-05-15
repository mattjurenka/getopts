#![no_main]
use libfuzzer_sys::fuzz_target;
use getopts::Options;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let mut opts = Options::new();
            opts.optopt("o", "", "set output file name", "NAME");
            opts.optflag("h", "help", "print this help menu");
            let _ = opts.parse(s.split_whitespace());
        },
        _ => {}
    }
    
});
