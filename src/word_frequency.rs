use csv::{ByteRecord, ReaderBuilder};
// use std::error::Error;

// struct Rank(u32);

// fn search_rank(word: &str) -> Result<Rank, Box<dyn Error>> {
//     let mut rdr = ReaderBuilder::new().from_path("./wordFrequency - 5000.csv")?;

//     for result in rdr.records() {
//         let record = result?;
//         if record.get(1) == Some(word) {
//             let rank = record.get(0).expect("No rank for this word");
//             let rank = rank.parse()?;
//             return Ok(Rank(rank));
//         }
//     }
//     Err(String::from("Not found").into())
// }

pub fn contain(word: &str) -> bool {
    let bytes = word.as_bytes();
    let mut rdr = ReaderBuilder::new()
        .from_path("./src/wordFrequency-5000.csv")
        .expect("word frequency file should exist");
    // https://docs.rs/csv/1.1.6/csv/tutorial/index.html#performance
    let mut record = ByteRecord::new();
    while let Ok(true) = rdr.read_byte_record(&mut record) {
        if record.get(1) == Some(bytes) {
            return true;
        }
    }
    false
}
