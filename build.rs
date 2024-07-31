//-- ./build.rs

//! Compile proto files via Prost and generates service stubs and proto definitions for use with tonic

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Proto files to compile
    let protos = [
        "./backend/proto/utilities.proto",
        "./backend/proto/sessions.proto",
        "./backend/proto/logins.proto",
        "./backend/proto/users.proto",
        "./backend/proto/authentication.proto"
    ];
    // Proto file path
    let includes = ["./backend/proto"];
    tonic_build::configure()
        // Don't build server code
        .build_server(false)
        // .out_dir("src/rpc")
        // .include_file("mod.rs")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&protos, &includes)?;

    Ok(())
}
