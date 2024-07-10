use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

use get_size::GetSize;
use uuid::Uuid;


fn main() -> anyhow::Result<()> {
    let output_dir = String::from("tmp/dump.txt");
    generate_sample_data(10_000_000, Path::new(&output_dir))?;


    Ok(())
}

fn generate_sample_data(datapoint_count: usize, output_dir: &Path) -> anyhow::Result<()> {
    let mut in_mem_hash: HashMap<String, String> = HashMap::new();

    let mut file_buffer = File::create(output_dir)?;
    for _i in 0..datapoint_count {
        let user_id = Uuid::new_v4();
        write!(&mut file_buffer, "{}\n", user_id)?;
        in_mem_hash.insert(user_id.to_string(), user_id.to_string());
   }

    file_buffer.flush()?;

    println!("In memory size: {:?}", in_mem_hash.get_heap_size());
    Ok(())
}
