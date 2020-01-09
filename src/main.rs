extern crate csv_import;
use csv_import::csv_parse;


fn main() {
    let mut datas: Vec<csv_parse::SCANDATA> = Vec::new();
    datas = csv_parse::read_sensor_data("sensor_data_600.csv".to_string()).unwrap();
    
    for item in datas{
        let record: csv_parse::SCANDATA = item;
        println!("{:?},{:?}",record.ir,record.lidar);
    }
    
}
