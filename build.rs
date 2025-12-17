use protobuf_codegen::CodeGen;

fn main() {
    CodeGen::new()
        .inputs(["proto/testo.proto"])
        .generate_and_compile()
        .unwrap();
}
