use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["type.proto"],
        &["../firehose-fuel/proto/sf/fuel/type/v1/"],
    )?;

    fs::write("target/schema.sdl", fuel_core_client::SCHEMA_SDL)
        .expect("Unable to write schema file");

    Ok(())
}
