extern crate core;

use std::thread;

mod http_request;

#[tokio::main]
async fn main() {
    // let func = rust_store::StateData {
    //     api_key: "your_api_key".to_string(),
    //     // make sure to change this to some key!
    //     // change this if your want but otherwise don't
    //     null: "null_nil_value_key:345n,234lj52".to_string(),
    //     // make sure this file exists or the code will not work!
    //     data_storage_location: "database/".to_string(),
    // };
    // func.write_data("this is going very well", "test/worked/local", "data")
    //     .expect("failed when writing data");
    // func.delete_data("test/worked/local/data").expect("");
    // func.null_write("test/worked/local/data").expect("");
    // let error = func.read_data("test/worked/local/data");
    // match error {
    //     Ok(_) => {
    //         panic!("bad")
    //     }
    //     Err(_) => {
    //         func.delete_data("test/worked/local/data").expect("");
    //         func.start_database_online();
    //         return;
    //     }
    // }
    let cool = "hi I will crash your computer".to_owned();
    let mut num: i128 = 0;
    loop {
        num = num + 1;
        println!("{}", num);
        cool.clone();
        num.clone();
    }
}
//http://10.50.33.48:8081/topic.php?t=Chronology
