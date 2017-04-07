extern crate lalrpop;

use std::env;

fn main() {
    env::set_var("LALRPOP_LANE_TABLE", "enabled");
    lalrpop::Configuration::new()
        .emit_comments(false)
        .log_verbose()
        .process_current_dir()
        .unwrap();
    //lalrpop::process_root().unwrap();
    println!("cargo:rerun-if-changed=src/parser/lrsql.lalrpop");
}
