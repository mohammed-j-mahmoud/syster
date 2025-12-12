use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kerml.pest"]
pub struct KerMLParser;
