extern crate csv;
extern crate serde;

use std::error::Error;
use std::process;

use serde::{Deserialize};

use csv::Reader; 

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct SCANDATA {
    pub date: i32,
    pub time: i32,
    pub ir: i32,
    pub lidar: i32,
}

pub fn read_sensor_data(path : String)  -> Result<Vec<SCANDATA>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut iter = rdr.deserialize();
    let mut datas: Vec<SCANDATA> = Vec::new();
    for (num,item) in iter.enumerate(){
        let record: SCANDATA = item?;
        datas.push(record);
    }
    Ok(datas)

}
