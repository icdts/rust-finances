extern crate csv;
extern crate chrono;
extern crate rustc_serialize;

use chrono::date::Date;

#[derive(Debug)]
struct Transaction {
    id : i64,
    date: Date<chrono::UTC>,
    desc: String,
    amnt: i32
}
fn main() {
    let mut reader = csv::Reader::from_file("/tmp/transactions.csv").unwrap();
    for record in reader.decode() {
        let (date,account,description,category,tags,amount):(String,String,String,String,String,String) = record.unwrap();


        println!("{}", date)
    }
}
