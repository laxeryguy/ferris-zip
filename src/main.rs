use std::{collections::HashMap, fs};
mod smasher;
/* 
for i in str::from_utf8(&data) {
        println!("{}", i);
}
байты в строку
*/
/*let contents = fs::read_to_string("test.txt").unwrap();
    println!("{}", contents);
*/



fn main() {
    let test: smasher::SFile = smasher::SFile::new(String::from("test.txt"), fs::read("test.txt").expect("gg"));
    let mut file: HashMap<u128, u8> = HashMap::new();
    let mut global_id: u32 = 1;
    for i in test.get_data() {
        if file.contains_key(&(i as u128)) {
            continue;
        } else {
            file.insert(i as u128, global_id as u8);
            global_id += 1;
        }
    }
    

    let mut compressed_f: Vec<u128> = Vec::new();
    for i in test.get_data() {
        compressed_f.push(*file.get(&(i as u128)).unwrap() as u128);
        compressed_f.push(0);
    }

    println!("Size before compress -> {}", test.get_original_size());
    println!("{:?}", test.get_data());
    println!("--------------------------");
    println!("Size after compress -> {}", compressed_f.len());
    println!("{:?}", compressed_f);

}