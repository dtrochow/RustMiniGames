use rumiga_core::{game_base, runner};

#[no_mangle]
pub extern "C" fn hello_test() {
    let result = rumiga_core::add(2, 2)
    println!("hello world! 2 + 2 = {}", result);
    println!("TEST TEST TEST!");
}