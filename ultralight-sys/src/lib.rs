#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_os = "windows")]
pub fn check_for_resources() {
    let mut current_exe = std::env::current_exe().expect("Failed to get current exe path!");
    current_exe.pop();

    let dll_names = [
        "AppCore.dll",
        "Ultralight.dll",
        "UltralightCore.dll",
        "WebCore.dll",
    ];

    for dll_name in dll_names.iter() {
        if !current_exe.join(dll_name).exists() {
            panic!("Missing Ultralight resource: {}", dll_name);
        }
    }

    let resource_names = ["cacert.pem", "icudt67l.dat"];

    for resource_name in resource_names.iter() {
        if !current_exe.join("resources").join(resource_name).exists() {
            panic!("Missing Ultralight resource: {}", resource_name);
        }
    }
}

#[cfg(target_os = "linux")]
pub fn check_for_resources() {
    let mut current_elf = std::env::current_dir().expect("Failed to get current directory!");
    current_elf.pop();

    let so_names = [
        "libAppCore.so",
        "libUltralight.so",
        "libUltralightCore.so",
        "libWebCore.so",
    ];

    for so_name in so_names.iter() {
        if !current_elf.join(so_name).exists() {
            panic!("Missing Ultralight resource: {}", so_name);
        }
    }

    let resource_names = ["cacert.pem", "icudt67l.dat"];

    for resource_name in resource_names.iter() {
        if !current_elf.join("resources").join(resource_name).exists() {
            panic!("Missing Ultralight resource: {}", resource_name);
        }
    }
}
