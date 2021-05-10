use std::{fs::File, io::Read, path::PathBuf};

fn get_test_program_path(name: &str) -> PathBuf {
    let mut pathbuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pathbuf.push("tests/program_test/programs");
    pathbuf.push(name);
    pathbuf.set_extension("_so");
    pathbuf
}

pub fn read_test_program_elf(name: &str) -> Vec<u8> {
    let path = get_test_program_path(name);
    let mut file = File::open(&path).unwrap_or_else(|err| {
        panic!("Failed to open {}: {}", path.display(), err);
    });
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();

    elf
}
