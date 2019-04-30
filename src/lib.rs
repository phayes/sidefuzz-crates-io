// This section contains utility functions used by WASM targets
// ------------------------------------------------------------


/// A function that is opaque to the optimizer, to allow fuzzed functions to
/// pretend to use outputs to assist in avoiding dead-code elimination.
///
/// NOTE: We don't have a proper black box in stable Rust. This is
/// a workaround implementation, that may have a too big performance overhead,
/// depending on operation, or it may fail to properly avoid having code
/// optimized out. It is good enough that it is used.
#[inline(never)]
pub fn black_box<D>(dummy: D) -> D {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}


// Assign a 1024 byte vector to hold inputs
lazy_static::lazy_static! {
  static ref INPUT: Vec<u8> = vec![0; 1024];
}

// The actual input length (generally less than 1024)
static mut INPUT_LEN: i32 = 0;

/// Get an input of the desired length.
/// This function should be called with a constant unchanging len argument.
/// Calling it with different lengths will result in invalid fuzzing.
///
/// Example:
/// ```ignore
/// let input = sidefuzz::fetch_input(32); // get 32 bytes of input
/// sidefuzz::black_box(my_contant_time_fn(input));
/// ```
///
// This is a VERY odd fuction that provides us with a really nice external API.
// 1. It is called once before fuzzing starts in order to set the size of INPUT.
// 2. After it is called once, we call input_pointer and input_len from the host to get a stable pointer to INPUT.
// 3. Fuzzing starts, we write data to INPUT from the host, then call the exported `fuzz` function.
pub fn fetch_input(len: i32) -> &'static [u8] {
    // This use of unsafe since wasm is single-threaded and nothing else is accessing INPUT_LEN.
    unsafe {
        if INPUT_LEN == 0 {
            INPUT_LEN = len;
            panic!("Input length successfully set. Panicking to unwind and stop execution.");
        }
    }


    &INPUT[0..len as usize]
}

/// Get a pointer to the input array
/// This needs to be public so we can call it across host/wasm boundary,
/// but it should be considered a "private" function to sidefuzz.
/// It's API is not stable and may be subject to change
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn input_pointer() -> i32 {
    INPUT.as_ptr() as i32
}

/// Get the length of the input array
/// This needs to be public so we can call it across host/wasm boundary,
/// but it should be considered a "private" function to sidefuzz.
/// It's API is not stable and may be subject to change
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn input_len() -> i32 {
    unsafe { INPUT_LEN }
}