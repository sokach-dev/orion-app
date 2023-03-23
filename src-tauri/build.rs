// use std::process::Command;

fn main() {
    // tonic_build::configure()
    //     .out_dir("src/abi/pb")
    //     .type_attribute(
    //         "orion.VocabularyQuery",
    //         "#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]",
    //     )
    //     .type_attribute("orion.VocabularyQuery", "#[builder(setter(into), default)]")
    //     .compile(&["protos/orion.proto"], &["protos"])
    //     .unwrap();

    // Command::new("cargo").args(["fmt"]).output().unwrap();

    // println!("cargo:rerun-if-changed=protos/orion.proto");
    tauri_build::build()
}
