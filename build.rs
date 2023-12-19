use std::{path::PathBuf, env};

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("descriptor.bin");
    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .build_server(true)
        .out_dir("src")
        .compile(
            &[
                "puan_core.proto"
            ],
            &["puan-proto/v1"], // Add the directory containing your proto files
        )?;
    Ok(())
}

