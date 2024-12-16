use enum_expand::enum_expand;
use snafu::prelude::*;

#[enum_expand]
#[derive(Debug, Snafu)]
pub enum Error {
    #[enum_expand]
    Common {
        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display("Bad thing 1"))]
    Alfa,

    #[snafu(display("Bad thing 2"))]
    Beta,
}

fn main() -> Result<(), Error> {
    AlfaSnafu.fail()?;
    Ok(())
}
