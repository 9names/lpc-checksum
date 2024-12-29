use goblin::elf::program_header;
use std::env;
use std::fs;
use std::path::Path;

/// From the user manual of LPC devices (except LPC55):
///
/// "The reserved exception vector location 7 (offset 0x001C in the vector table)
/// should contain the 2’s complement of the check-sum of table entries 0 through 6. This
/// causes the checksum of the first 8 table entries to be 0. The boot loader code checksums
/// the first 8 locations in sector 0 of the flash. If the result is 0, then execution control is
/// transferred to the user code."
fn write_signature(data: &mut [u8]) {
    let checksum2 = data[0..0x1c].chunks(4).fold(0u32, |acc, val| {
        assert!(val.len() == 4);
        acc.wrapping_add(u32::from_le_bytes(val.try_into().unwrap()))
    });
    
    let mut checksum: u32 = 0;
    for i in 0..7 {
        let i = i * 4;
        let val: u32 = u32::from_le_bytes(data[i..i + 4].try_into().unwrap());
        checksum = checksum.wrapping_add(val);
    }
    assert!(checksum == checksum2);
    let checksum = 0u32.wrapping_sub(checksum);
    // and store this into exception entry 6
    data[0x1c..0x20].copy_from_slice(&checksum.to_le_bytes());
}

fn lpc_sign_elf(src: &Path, dest: &Path) -> Result<(), ()> {
    let mut buffer = fs::read(src).expect("Unable to read file");
    let res = goblin::Object::parse(&buffer).expect("Unable to read file as elf");
    if let goblin::Object::Elf(elf) = res {
        for header in elf.program_headers {
            if header.p_type == program_header::PT_LOAD && header.p_paddr == 0 {
                let ofs = header.p_offset as usize;
                let end_ofs = ofs + 0x20;
                write_signature(&mut buffer.as_mut_slice()[ofs..end_ofs]);
                fs::write(dest, &buffer).unwrap();
                return Ok(());
            }
        }
    }
    Err(())
}

pub fn main() {
    let arg: Vec<String> = env::args().collect();
    match arg.len() {
        2 => {
            let path = Path::new(arg[1].as_str());
            let mut target_path = path.file_name().unwrap().to_owned();
            target_path.push("_boot.elf");
            let path2 = Path::new(&target_path);
            lpc_sign_elf(path, path2).expect("Couldn't find the expected flash region");
        }
        3 => {
            let path = Path::new(arg[1].as_str());
            let path2 = Path::new(arg[2].as_str());
            lpc_sign_elf(path, path2).expect("Couldn't find the expected flash region");
        }
        _ => {
            eprintln!("Usage: lpc-checksum source_elf_name [target_elf_name]");
        }
    }
}
