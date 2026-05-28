fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate("v1", "proto/rewire/v1/rewire.proto", "rewire.v1.rs")?;
    generate("v2", "proto/rewire/v2/rewire.proto", "rewire.v2.rs")?;
    Ok(())
}

fn generate(
    version: &str,
    proto: &str,
    generated_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(format!("src/proto/{version}"));
    std::fs::create_dir_all(&out_dir)?;

    tonic_prost_build::configure()
        .out_dir(&out_dir)
        .compile_protos(&[proto], &["proto"])?;

    std::fs::rename(out_dir.join(generated_name), out_dir.join("rewire.rs"))?;

    println!("Generated proto code at src/proto/{version}/rewire.rs");
    Ok(())
}
