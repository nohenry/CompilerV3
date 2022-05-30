use std::env;
use std::path::Path;
use std::process::Command;

fn output(cmd: &mut Command) -> String {
    String::from(std::str::from_utf8(&cmd.output().unwrap().stdout).unwrap())
}

fn main() {
    let target = env::var("TARGET").expect("TARGET was not set");
    let host = env::var("HOST").expect("HOST was not set");
    let is_crossed = target != host;

    let llvm_config =
        "D:\\Developement\\Libraries\\llvm-project\\build\\Debug\\bin\\llvm-config.exe";

    let mut cmd = Command::new(&llvm_config);
    cmd.arg("--link-static").arg("--cxxflags");
    let flags = output(&mut cmd);
    println!("Falgs {}", flags);

    std::env::set_var("CXXFLAGS", flags);
    cc::Build::new()
        .file("wrappers/targets.cpp")
        .cpp(true)
        // .static_flag(true)
        .include("D:\\Developement\\Libraries\\llvm-project\\llvm\\include")
        .compile("wrappers");

    println!("cargo:config_path=D:\\Developement\\Libraries\\llvm-project\\llvm\\Debug\\bin\\llvm-config");
    println!("cargo:libdir=D:\\Developement\\Libraries\\llvm-project\\llvm\\Debug\\lib");

    let components = vec!["core"];

    let mut cmd = Command::new(&llvm_config);
    cmd.arg("--link-static").arg("--libs");

    cmd.arg("--system-libs");

    cmd.args(&components);

    let llvm_kind = "static";

    for lib in output(&mut cmd).split_whitespace() {
        let name = if let Some(stripped) = lib.strip_prefix("-l") {
            stripped
        } else if let Some(stripped) = lib.strip_prefix('-') {
            stripped
        } else if Path::new(lib).exists() {
            // On MSVC llvm-config will print the full name to libraries, but
            // we're only interested in the name part
            let name = Path::new(lib).file_name().unwrap().to_str().unwrap();
            name.trim_end_matches(".lib")
        } else if lib.ends_with(".lib") {
            // Some MSVC libraries just come up with `.lib` tacked on, so chop
            // that off
            lib.trim_end_matches(".lib")
        } else {
            continue;
        };

        // Don't need or want this library, but LLVM's CMake build system
        // doesn't provide a way to disable it, so filter it here even though we
        // may or may not have built it. We don't reference anything from this
        // library and it otherwise may just pull in extra dependencies on
        // libedit which we don't want
        if name == "LLVMLineEditor" {
            continue;
        }

        let kind = if name.starts_with("LLVM") {
            llvm_kind
        } else {
            "dylib"
        };
        println!("cargo:rustc-link-lib={}={}", kind, name);
    }

    let mut cmd = Command::new(&llvm_config);
    cmd.arg("--link-static").arg("--ldflags");

    for lib in output(&mut cmd).split_whitespace() {
        if is_crossed {
            if let Some(stripped) = lib.strip_prefix("-LIBPATH:") {
                println!(
                    "cargo:rustc-link-search=native={}",
                    stripped.replace(&host, &target)
                );
            } else if let Some(stripped) = lib.strip_prefix("-L") {
                println!(
                    "cargo:rustc-link-search=native={}",
                    stripped.replace(&host, &target)
                );
            }
        } else if let Some(stripped) = lib.strip_prefix("-LIBPATH:") {
            println!("cargo:rustc-link-search=native={}", stripped);
        } else if let Some(stripped) = lib.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={}", stripped);
        } else if let Some(stripped) = lib.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={}", stripped);
        }
    }

    let mut cmd = Command::new(&llvm_config);
    cmd.arg("--link-static").arg("--ldflags");
    for lib in output(&mut cmd).split_whitespace() {
        if is_crossed {
            if let Some(stripped) = lib.strip_prefix("-LIBPATH:") {
                println!(
                    "cargo:rustc-link-search=native={}",
                    stripped.replace(&host, &target)
                );
            } else if let Some(stripped) = lib.strip_prefix("-L") {
                println!(
                    "cargo:rustc-link-search=native={}",
                    stripped.replace(&host, &target)
                );
            }
        } else if let Some(stripped) = lib.strip_prefix("-LIBPATH:") {
            println!("cargo:rustc-link-search=native={}", stripped);
        } else if let Some(stripped) = lib.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={}", stripped);
        } else if let Some(stripped) = lib.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={}", stripped);
        }
    }

    // println!("cargo:rustc-link-search=D:/Developement/Libraries/llvm-project/build/Debug/lib");
    println!("cargo:rustc-link-lib=msvcrtd");
    // println!("cargo:rustc-link-lib=static=LLVMRemarks");
    // println!("cargo:rustc-link-lib=static=LLVMBitstreamReader");
    // println!("cargo:rustc-link-lib=static=LLVMBinaryFormat");
    // println!("cargo:rustc-link-lib=static=LLVMSupport");
    // println!("cargo:rustc-link-lib=static=LLVMDemangle");
    // // println!("cargo:rustc-link-lib=static=psapi");
    // //.lib shell32.lib ole32.lib uuid.lib advapi32
    // // println!("cargo:rustc-link-lib=static=shell32");
    // println!("cargo:rustc-link-lib=static=ole32");
    // println!("cargo:rustc-link-lib=static=uuid");
    // println!("cargo:rustc-link-lib=static=advapi32");
    // println!(
    //     "cargo:config_path={}",
    //     LLVM_CONFIG_PATH.clone().unwrap().display()
    // ); // will be DEP_LLVM_CONFIG_PATH
    // println!("cargo:libdir={}", libdir);
}
