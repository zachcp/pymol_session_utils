use pymol_session_utils::pymolparsing::parsing::{CustomValue, Settings, SettingsEnum};
use pymol_session_utils::PSEData;
const TEST_OUTPUT_DIR: &str = "./test_temporary";

#[test]
fn test_load_pse_data_molecule_only() {
    //https://users.rust-lang.org/t/serde-untagged-enum-ruins-precise-errors/54128/2
    // https://www.gustavwengel.dk/serde-untagged-enum-errors-are-bad
    let deserialized: PSEData = PSEData::load("tests/data/example_molecule_only.pse").unwrap();
    // deserialized.to_json("tests/data/example_molecule_only.json");
    assert!(deserialized.version == 3000000);
    assert!(deserialized.get(SettingsEnum::Orthoscopic).unwrap().value == CustomValue::Integer(0));
    // println!("{:?}", deserialized.settings);
}

#[test]
fn test_load_pse_data_molecule_selection() {
    let deserialized: PSEData = PSEData::load("tests/data/example.pse").unwrap();
    // let _ = deserialized.to_json("tests/data/example.json");
    assert!(deserialized.version == 3000000);
}

#[test]
fn test_pdb_00() {
    let psedata: PSEData = PSEData::load("tests/data/example.pse").unwrap();
    let names = psedata.get_session_names();
    print!("{:?}", names);
    // this has a Molecule and a selection
    assert_eq!(names.len(), 2);

    let mols = psedata.get_molecule_data();
    assert_eq!(mols.len(), 1);
    let atom01 = mols[0].get_atom(0);
    assert!(atom01.x() == 50.87300109863281);
    assert!(atom01.y() == 32.97800064086914);
    assert!(atom01.z() == 2.38700008392334);

    let chains = mols[0].get_chains();
    println!("Chains: {:?}", chains);
    let residues = mols[0].get_residues_by_chain(chains[0].clone());
    println!("Residues: {:?}", residues);

    let residue = mols[0].create_residue(chains[0].clone(), residues[0]);
    println!("Residue: {:?}", residue);

    let chain = mols[0].create_chain(chains[0].clone());
    println!("Chain: {:?}", chain);

    // Check symmetry code
    let (unit, sym) = mols[0].get_unit_cell_symmetry();
    println!("{:?},{:?}", unit, sym);

    // Move on to PDB, baby!
    let pdb = mols[0].to_pdb();

    let _ = pdbtbx::save_pdb(
        &pdb,
        format!("{}/test_01.pdb", TEST_OUTPUT_DIR),
        pdbtbx::StrictnessLevel::Strict,
    )
    .expect("PDB output");
}

#[test]
fn test_pdb_01() {
    let psedata: PSEData = PSEData::load("tests/data/example.pse").unwrap();
    // let _ = psedata.to_disk(TEST_OUTPUT_DIR);
    let _ = psedata.to_disk_full(TEST_OUTPUT_DIR);
    let url = psedata.to_mvsj_url();
    println!("{}", url);
}
