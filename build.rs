extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_comments(false)
        .log_verbose()
        .process_current_dir()
        .unwrap();
    //lalrpop::process_root().unwrap();
    println!("cargo:rerun-if-changed=src/parser/lrsql.lalrpop");
}