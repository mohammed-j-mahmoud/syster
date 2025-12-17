mod collection;
mod parsing;

pub use collection::collect_file_paths;
pub use parsing::{load_and_parse, parse_content, parse_with_result};

#[cfg(test)]
mod tests;
