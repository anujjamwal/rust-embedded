use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Setup the linker script
    let mut linker_script = File::create(out_dir.join("link.x"))?;

    if target.starts_with("thumbv7") {
        linker_script.write_all(include_bytes!("ld/arch/arm-cortex-m4.x"))?;
    }

    Ok(())
}
