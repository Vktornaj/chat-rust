use protobuf_codegen::Codegen;
use protoc_bin_vendored::protoc_bin_path;


pub fn build() {

    let protoc_bin_path = protoc_bin_path().unwrap();

    // Use this in build.rs
    Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_path)
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["common/src/adapter/protos"])
        // Inputs must reside in some of include paths.
        .input("common/src/adapter/protos/proto_message.proto")
        // .input("src/protos/banana.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("common/src/adapter/protos_schemas")
        .run_from_script();
}