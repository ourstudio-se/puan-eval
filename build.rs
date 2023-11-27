use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src")
        .compile(
            &[
                "puan-proto/puan_core.proto",
                "puan-proto/puan_eval.proto"
            ], // Add sdic.proto here
            &["proto/"], // Add the directory containing your proto files
        )?;
    Ok(())
}

