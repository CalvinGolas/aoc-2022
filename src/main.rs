use std::fs;

fn main() {
    println!("Hello, world!");
    let file_path = "input.txt";
    read_input(file_path);
}

// Read in input file and convert to list of combined values
fn read_input( file_path: &str) {
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut cal_vec = Vec::new();
    let mut current_elf_cal = 0;
    for mut line in contents.lines() {
        line = line.trim();
        // See if larger than previous, then continue
        if line.len() == 0 {
            cal_vec.push(current_elf_cal);
            current_elf_cal = 0;
        } else {
            let input: u32 = line
                .trim()
                .parse()
                .expect("Wanted a number");
            current_elf_cal += input;
        }
    }
    cal_vec.sort();
    let mut combined_max =cal_vec.pop().unwrap_or(0);
    combined_max += cal_vec.pop().unwrap_or(0);
    combined_max+= cal_vec.pop().unwrap_or(0);
    println!("Found max: {combined_max}");
}