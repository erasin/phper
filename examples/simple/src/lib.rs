#![feature(allocator_api)]

use phper::{c_str_ptr, php_fn, ebox};
use phper::sys::{ZEND_RESULT_CODE_SUCCESS, zend_parse_parameters, zend_internal_arg_info, zend_function_entry, PHP_INI_SYSTEM};
use phper::sys::{zend_ini_entry_def, zend_module_entry, zend_register_ini_entries, zend_unregister_ini_entries};
use phper::zend::api::FunctionEntries;
use phper::zend::compile::InternalArgInfos;
use phper::zend::ini::IniEntryDefs;
use phper::zend::modules::ModuleEntry;
use phper::zend::types::{ExecuteData, Val, SetVal, Value};
use phper::{
    php_function, php_minit, php_minit_function, php_mshutdown, php_mshutdown_function,
    php_rinit_function, php_rshutdown_function,
};
use phper::{php_minfo, php_minfo_function, php_rinit, php_rshutdown, zend_get_module};
use std::ffi::CStr;
use std::mem;
use std::mem::{size_of, transmute};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort};
use std::ptr::{null, null_mut};

static INI_ENTRIES: IniEntryDefs<2> = IniEntryDefs::new([
    zend_ini_entry_def {
        name: c_str_ptr!("simple.enable"),
        on_modify: None,
        mh_arg1: null_mut(),
        mh_arg2: null_mut(),
        mh_arg3: null_mut(),
        value: c_str_ptr!("1"),
        displayer: None,
        modifiable: PHP_INI_SYSTEM as c_int,
        name_length: 0,
        value_length: 0,
    },
    unsafe { transmute([0u8; size_of::<zend_ini_entry_def>()]) },
]);

#[php_minit_function]
fn m_init_simple(type_: c_int, module_number: c_int) -> bool {
    unsafe {
        zend_register_ini_entries(INI_ENTRIES.get(), module_number);
    }
    true
}

#[php_mshutdown_function]
fn m_shutdown_simple(type_: c_int, module_number: c_int) -> bool {
    unsafe {
        zend_unregister_ini_entries(module_number);
    }
    true
}

#[php_rinit_function]
fn r_init_simple(type_: c_int, module_number: c_int) -> bool {
    true
}

#[php_rshutdown_function]
fn r_shutdown_simple(type_: c_int, module_number: c_int) -> bool {
    true
}

#[php_minfo_function]
fn m_info_simple(zend_module: *mut ::phper::sys::zend_module_entry) {
}

#[php_function]
pub fn test_simple(execute_data: ExecuteData) -> impl SetVal {
    let mut a: *const c_char = null_mut();
    let mut a_len = 0;
    let mut b: *const c_char = null_mut();
    let mut b_len = 0;

    unsafe {
        if zend_parse_parameters(
            execute_data.num_args() as c_int,
            c_str_ptr!("ss"),
            &mut a,
            &mut a_len,
            &mut b,
            &mut b_len,
        ) != ZEND_RESULT_CODE_SUCCESS
        {
            return Value::Null;
        }

        Value::String(format!(
            "(a . b) = {}{}",
            CStr::from_ptr(a).to_str().unwrap(),
            CStr::from_ptr(b).to_str().unwrap(),
        ))
    }
}

#[zend_get_module]
pub fn get_module() -> &'static ModuleEntry {
    static ARG_INFO_TEST_SIMPLE: InternalArgInfos<3> = InternalArgInfos::new([
        zend_internal_arg_info {
            name: 2 as *const _,
            type_: 0,
            pass_by_reference: 0,
            is_variadic: 0,
        },
        zend_internal_arg_info {
            name: c_str_ptr!("a"),
            type_: 0,
            pass_by_reference: 0,
            is_variadic: 0,
        },
        zend_internal_arg_info {
            name: c_str_ptr!("b"),
            type_: 0,
            pass_by_reference: 0,
            is_variadic: 0,
        },
    ]);

    static FUNCTION_ENTRIES: FunctionEntries<2> = FunctionEntries::new([
        zend_function_entry {
            fname: c_str_ptr!("test_simple"),
            handler: Some(php_fn!(test_simple)),
            arg_info: ARG_INFO_TEST_SIMPLE.get(),
            num_args: 2,
            flags: 0,
        },
        unsafe { transmute([0u8; size_of::<zend_function_entry>()]) },
    ]);

    static MODULE_ENTRY: ModuleEntry = ModuleEntry::new(zend_module_entry {
        size: size_of::<zend_module_entry>() as c_ushort,
        zend_api: phper::sys::ZEND_MODULE_API_NO as c_uint,
        zend_debug: phper::sys::ZEND_DEBUG as c_uchar,
        zts: phper::sys::USING_ZTS as c_uchar,
        ini_entry: std::ptr::null(),
        deps: std::ptr::null(),
        name: c_str_ptr!(env!("CARGO_PKG_NAME")),
        functions: FUNCTION_ENTRIES.get(),
        module_startup_func: Some(php_minit!(m_init_simple)),
        module_shutdown_func: Some(php_mshutdown!(m_shutdown_simple)),
        request_startup_func: Some(php_rinit!(r_init_simple)),
        request_shutdown_func: Some(php_rshutdown!(r_shutdown_simple)),
        info_func: Some(php_minfo!(m_info_simple)),
        version: c_str_ptr!(env!("CARGO_PKG_VERSION")),
        globals_size: 0usize,
        #[cfg(phper_zts)]
        globals_id_ptr: std::ptr::null_mut(),
        #[cfg(not(phper_zts))]
        globals_ptr: std::ptr::null_mut(),
        globals_ctor: None,
        globals_dtor: None,
        post_deactivate_func: None,
        module_started: 0,
        type_: 0,
        handle: null_mut(),
        module_number: 0,
        build_id: phper::sys::PHP_MODULE_BUILD_ID,
    });

    &MODULE_ENTRY
}
