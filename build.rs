use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder};

fn main() -> anyhow::Result<()> {
    //-- 1. Generate rustc-env code into target folder
    // Use Vergen to generate 'cargo:rustc-env' instructions via 'build.rs' for
    // use in your code via the 'env!' macro
    let build = BuildBuilder::all_build()?;
    let gix = GixBuilder::all_git()?;
    let cargo = CargoBuilder::all_cargo()?;
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&gix)?
        .add_instructions(&cargo)?
        .emit()?;

    //-- 2. Generate proto files via Prost, service stubs and proto definitions for use with tonic
    // Proto files to compile
    let protos = [
        "./backend/proto/utilities.proto",
        "./backend/proto/sessions.proto",
        "./backend/proto/logins.proto",
        "./backend/proto/users.proto",
        "./backend/proto/authentication.proto",
    ];

    // Proto file path
    let protos_folder = ["./backend/proto"];
    tonic_build::configure()
        // Don't build server code
        .build_server(false)
        // .out_dir("src/rpc")
        // .include_file("mod.rs")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&protos, &protos_folder)?;


    Ok(())
}
