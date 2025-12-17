use protobuf_codegen::CodeGen;
use std::process::Command;

fn main() {
    // Get protoc path from mise
    let output = Command::new("mise")
        .args(["which", "protoc"])
        .output()
        .expect("Failed to run mise which protoc");

    let protoc_path = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 from mise")
        .trim()
        .to_string();

    CodeGen::new()
        .protoc_path(protoc_path)
        .inputs(["testo.proto"])
        .include("proto")
        .dependency(protobuf_well_known_types::get_dependency("protobuf_well_known_types"))
        .generate_and_compile()
        .unwrap();
}
