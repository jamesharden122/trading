pub mod vectype;
use fnv::FnvHashMap;
use std::io::Write;
use std::path::Path;
use vectype::*;

pub async fn write_hashmap_to_bin(map: FnvHashMap<String, VecType>) {
    _ = iterate_and_match(map).await;
}

// Function to write columnar data to a binary file
fn write_columnar_data_to_file(
    filename: std::path::PathBuf,
    columns: &[Vec<u8>],
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(filename)?;
    let mut writer = std::io::BufWriter::new(file);
    for column in columns {
        // Write the length of the column first (optional, for reading later)
        let length = column.len() as u64;
        writer.write_all(&length.to_be_bytes())?;
        // Write the column data (u8 values are already single-byte values)
        writer.write_all(column)?;
    }
    writer.flush()?;
    Ok(())
}

// Assuming your existing write_columnar_data_to_file function
pub async fn iterate_and_match(map: FnvHashMap<String, VecType>) {
    let mut bytes_map: FnvHashMap<String, Vec<u8>> = FnvHashMap::default();
    let begin_path = "/home/yakaman/trading/bento_queries/";

    // Iterate over the FnvHashMap
    println!("{:?}", map.keys());
    for (key, value) in map.iter() {
        bytes_map.insert(key.to_string(), Vec::<u8>::new());
        println!("{:?}", key);
        // Match on the VecType variant
        match value {
            VecType::I8Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_i8_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("I8Vec: {:?}", temp);
                }
            }
            VecType::U8Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_u8_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("U8Vec: {:?}", temp);
                }
            }
            VecType::I32Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_i32_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("I32Vec: {:?}", temp);
                }
            }
            VecType::U32Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_u32_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("U32Vec: {:?}", temp);
                }
            }
            VecType::I64Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_i64_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("I64Vec: {:?}", temp);
                }
            }
            VecType::U64Vec(vec) => {
                for val in value.iter() {
                    let temp = extract_u64_from_vec_type_item(val).unwrap().to_be_bytes();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("U64Vec: {:?}", temp);
                }
            }
            VecType::StringVec(vec) => {
                for val in value.iter() {
                    let temp = extract_string_from_vec_type_item(val)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    bytes_map
                        .entry(key.to_string())
                        .and_modify(|v| v.extend_from_slice(&temp));
                    print!("{:?} ", val);
                    println!("StringVec: {:?}", temp);
                }
            }
        }

        // Write the byte data to file
        let path = Path::new(begin_path);
        let new_path = path.join(format!("{}.bin", key));
        write_columnar_data_to_file(new_path, &[bytes_map.get(key).unwrap().to_vec()]).unwrap();
    }
}
