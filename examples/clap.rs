use clap::Parser;
use optional_struct::*;

#[optional_struct(OptionalFoo, true, true)]
pub struct Foo {
    bar: char,
    baz: bool,
}

#[derive(Parser, Debug, serde::Deserialize, serde::Serialize)]
pub struct OptionalFoo {
    #[clap(long)]
    #[serde(default)]
    pub bar: Option<char>,

    #[clap(long)]
    #[serde(default)]
    pub baz: Option<bool>,
}

fn main() {
    let opt_f = OptionalFoo::parse();
    println!("{opt_f:?}");
}
