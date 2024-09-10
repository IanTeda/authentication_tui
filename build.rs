use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder};

fn main() -> anyhow::Result<()> {
    // Use Vergen to generate 'cargo:rustc-env' instructions via 'build.rs' for
    // use in your code via the 'env!' macro
    let build = BuildBuilder::all_build()?;
    let gix = GixBuilder::all_git()?;
    let cargo = CargoBuilder::all_cargo()?;
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&gix)?
        .add_instructions(&cargo)?
        .emit()
}
