//-- ./build.rs

//! Compile proto files via prost and generates service stubs and proto definitions for use with tonic

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Proto files to compile
    let protos = [
        "/home/ian/Workspaces/authentication_microservice/proto/utilities.proto",
        "/home/ian/Workspaces/authentication_microservice/proto/sessions.proto",
        "/home/ian/Workspaces/authentication_microservice/proto/logins.proto",
        "/home/ian/Workspaces/authentication_microservice/proto/users.proto",
        "/home/ian/Workspaces/authentication_microservice/proto/authentication.proto"
    ];
    // Proto file path
    let includes = ["/home/ian/Workspaces/authentication_microservice/proto"];
    tonic_build::configure()
        // Dont build server code
        .build_server(false)
        .out_dir("src/rpc")
        .include_file("mod.rs")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&protos, &includes)?;

    Ok(())
}
