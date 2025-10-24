use tracel_xtask::prelude::*;

#[macros::base_commands(Bump, Compile, Publish)]
pub enum Command {}

fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>(parse_args::<Command>()?)?;
    dispatch_base_commands(args)
}
