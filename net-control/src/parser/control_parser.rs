use std::{sync::{Mutex, Once}, mem::MaybeUninit};


struct CLIParser{}

impl CLIParser {
    pub fn get_cli_parser() -> &'static Mutex<Self> {
        static mut SINGLETON: MaybeUninit<Mutex<CLIParser>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        unsafe {
            ONCE.call_once(|| {
                SINGLETON.as_mut_ptr().write(Mutex::new(CLIParser::new()));
            });
        
            &*SINGLETON.as_ptr()
        }
    }

    fn new() -> Self {
        CLIParser{}
    }
}