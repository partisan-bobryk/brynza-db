use std::{collections::HashMap, fs::File, io::prelude::*, path::Path, u128};

use get_size::GetSize;
use uuid::Uuid;


fn main() -> anyhow::Result<()> {
    let output_dir = String::from("tmp/dump.txt");
    /*
    * Observations:
    * Prior runs of 10 million ids resulted memory consumption
    * of 1,424,643,120 bytes or 1.42464312 GB if stored as a String.
    *
    * When converted to be represented as a u128, the size went down
    * to 469,762,096 bytes or 469.762096 Mb. 
    *
    * A single uuid, converted to a string takes up 60 bytes as 
    * a String or 16bytes as a u128.
    *
    * When written to a file, total size 706Mb as String or 757Mb as u128.
    */
    generate_sample_data(10_000_000, Path::new(&output_dir))?;

    // Cost of one uuid 
    let sample_id = Uuid::new_v4().as_u128();
    println!("size of one uuid: {:?}", sample_id.get_size());

    Ok(())
}

fn generate_sample_data(datapoint_count: usize, output_dir: &Path) -> anyhow::Result<()> {
    let mut in_mem_hash: HashMap<u128, u128> = HashMap::new();

    let mut file_buffer = File::create(output_dir)?;
    for _i in 0..datapoint_count {
        let user_id = Uuid::new_v4().as_u128();
        write!(&mut file_buffer, "{}:{}\n", user_id, user_id)?;
        in_mem_hash.insert(user_id, user_id);
   }

    file_buffer.flush()?;

    println!("In memory size: {:?}", in_mem_hash.get_size());
    Ok(())
}
