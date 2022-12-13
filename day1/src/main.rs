use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let mut input_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_file.push("input/input.txt");

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut current_elf: i32 = 0;
    let mut elf_map: HashMap<i32, i32> = HashMap::new();

    for line_raw in reader.lines() {
        let line: String = match line_raw {
            Err(_) => panic!("Could not read line"),
            Ok(line) => line,
        };

        if !line.is_empty() {
            let cals: i32 = match line.as_str().parse() {
                Ok(num) => num,
                Err(_) => panic!("Could not parse num"),
            };

            println!("{} - {}", current_elf, cals);
            *elf_map.entry(current_elf).or_insert(0) += cals;
        } else {
            println!("{}", elf_map[&current_elf]);
            current_elf += 1;
            println!("");
        }
    }

    let mut count_vec: Vec<_> = elf_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut sum: i32 = 0;
    for (depth, value) in count_vec.iter().enumerate() {
        if depth > 2 {
            break;
        }
        sum += value.1;
        //sum += count_vec[i][1];
        println!("Elf: {} Count: {}", value.0, value.1);
    }
    println!("Sum {}", sum);

    Ok(())
}
