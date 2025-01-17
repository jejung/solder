extern crate libc;
extern crate solder;

use libc::*;
use solder::*;
use solder::zend::*;
use solder::info::*;

#[no_mangle]
pub extern fn php_module_info() {
    print_table_start();
    print_table_row("A demo PHP extension written in Rust", "enabled");
    print_table_end();
}

#[no_mangle]
pub extern fn get_module() -> *mut zend::Module {
    let mut entry = Box::new(zend::Module::new(
        c_str!("hello_world"),
        c_str!("0.1.0-dev"),
    ));

    entry.set_info_func(php_module_info);

    let args = Box::new([ArgInfo::new(c_str!("name"), 0, 0, 0)]);
    let funcs = Box::new([Function::new_with_args(c_str!("hello_world"), hello_world, args), Function::end(), ]);
    entry.set_functions(funcs);

    Box::into_raw(entry)
}


#[no_mangle]
pub extern fn hello_world(_data: &ExecuteData, retval: &mut Zval) {
    let mut name_zval = Zval::new_as_null();
    php_parse_parameters!(&mut name_zval);
    let name = String::try_from(name_zval).ok().unwrap();
    let hello = format!("Hello {}", name);
    php_return!(retval, hello);
}
