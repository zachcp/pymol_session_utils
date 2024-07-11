use pymol_session_utils::molviewspec::nodes::{ComponentExpression, KindT, State};
use pymol_session_utils::psedata::PSEData;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_load_pse_data_molecule_only() {
    //https://users.rust-lang.org/t/serde-untagged-enum-ruins-precise-errors/54128/2
    // https://www.gustavwengel.dk/serde-untagged-enum-errors-are-bad
    let deserialized: PSEData = PSEData::load("tests/data/example_molecule_only.pse").unwrap();
    // deserialized.to_json("tests/data/example_molecule_only.json");
    // Check fields of PSEData
    assert!(deserialized.version == 3000000);
}

#[test]
fn test_load_pse_data_molecule_selection() {
    let deserialized: PSEData = PSEData::load("tests/data/example.pse").unwrap();
    assert!(deserialized.version == 3000000);
}

#[test]
fn test_molspecview_json_1cbs() {
    let json_files_component_list = vec![
        "tests/mol-spec-data/1cbs/auth_residue.json",
        "tests/mol-spec-data/1cbs/chain_label.json",
        "tests/mol-spec-data/1cbs/rainbow.json",
        "tests/mol-spec-data/1cbs/validation.json",
    ];

    for json_file in json_files_component_list {
        let file = File::open(json_file).expect(&format!("Failed to open file: {}", json_file));
        let reader = BufReader::new(file);
        let _: Vec<ComponentExpression> = from_reader(reader).expect(&format!(
            "Failed to parse JSON as a Vector of ComponentExpressions: {}",
            json_file
        ));
    }

    // // Todo: Fix the Resideu_range_component/////
    // let json_files_component = vec!["tests/mol-spec-data/1cbs/auth_residue_range.json"];
    // for json_file in json_files_component {
    //     let file = File::open(json_file).expect(&format!("Failed to open file: {}", json_file));
    //     let reader = BufReader::new(file);
    //     let _: ComponentExpression = from_reader(reader).expect(&format!(
    //         "Failed to parse JSON as ComponentExpression: {}",
    //         json_file
    //     ));
    // }
}

#[test]
fn test_molspecview_json_1h9t() {
    let file = File::open("tests/mol-spec-data/1h9t/domains.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let testvec: Vec<ComponentExpression> =
        from_reader(reader).expect("Failed to parse JSON as ComponentExpression");

    assert_eq!(testvec[0].label_asym_id, Some("A".to_string()));
    assert_eq!(testvec[0].beg_label_seq_id, Some(9));
    assert_eq!(testvec[0].end_label_seq_id, Some(83));

    // todo: these shouw work!
    // assert_eq!(testvec[0].color, "#dd6600");
    // assert_eq!(testvec[0].tooltip, "DNA-binding");

    // let file = File::open("tests/mol-spec-data/1h9t/domains.json").expect("Failed to open file");
    // let reader = BufReader::new(file);
    // let testvec: Vec<ComponentExpression> =
    //     from_reader(reader).expect("Failed to parse JSON as ComponentExpression");
}

#[test]
fn test_molspecview_json_2bvk() {
    let file = File::open("tests/mol-spec-data/2bvk/atoms.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let testvec: Vec<ComponentExpression> =
        from_reader(reader).expect("Failed to parse JSON as ComponentExpression");

    assert_eq!(testvec[0].atom_index, Some(0));

    // // Todo : Fix color type on Component Expression
    // assert_eq!(testvec[0].color, "#ffdd88");
    // // Todo : Fix tooltip type on Component Expression
    // assert_eq!(testvec[0].tooltip, "First cycle (by atom_index)");
}

#[test]
fn test_molspecview_json_full_examples_annotations() {
    let file =
        File::open("tests/mol-spec-examples/annotations/state.mvsj").expect("Failed to open file");
    let reader = BufReader::new(file);
    let msvj: State = from_reader(reader).expect("Failed to parse JSON as ComponentExpression");

    assert_eq!(msvj.root.kind, KindT::Root);
    // // Todo : Fix color type on Component Expression
    // assert_eq!(testvec[0].color, "#ffdd88");
    // // Todo : Fix tooltip type on Component Expression
    // assert_eq!(testvec[0].tooltip, "First cycle (by atom_index)");
}
