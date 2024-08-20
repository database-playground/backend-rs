fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["proto/dbrunner.proto"], &["proto"])?;
    Ok(())
}
