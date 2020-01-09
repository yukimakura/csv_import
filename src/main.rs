extern crate csv;
extern crate serde;

use std::error::Error;
use std::process;

use serde::{Deserialize};

use csv::Reader;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct SCANDATA {
    date: i32,
    time: i32,
    ir: i32,
    lidar: i32,
}

fn read_sensor_data(path : String)  -> Result<Vec<SCANDATA>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut iter = rdr.deserialize();
    let mut datas: Vec<SCANDATA> = Vec::new();
    for (num,item) in iter.enumerate(){
        let record: SCANDATA = item?;
        datas.push(record);
    }
    Ok(datas)

}

fn main() {
    let mut datas: Vec<SCANDATA> = Vec::new();
    datas = read_sensor_data("sensor_data_600.csv".to_string()).unwrap();
    
    for item in datas{
        let record: SCANDATA = item;
        println!("{:?},{:?}",record.ir,record.lidar);
    }
    
}
