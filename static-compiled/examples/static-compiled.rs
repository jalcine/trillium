use myco_static_compiled::{include_dir, StaticCompiled};
pub fn main() {
    let handler = StaticCompiled::new(include_dir!("../docs/book")).with_index_file("index.html");
    myco_smol_server::run("localhost:8000", (), handler);
}