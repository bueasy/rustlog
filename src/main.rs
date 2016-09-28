extern crate encoding;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encoding::{Encoding, DecoderTrap};
use encoding::all::GBK;
struct MyLog {
    begin_time: String,
    end_time: String,
    cost: String,
    func_code: String,
    request: String,
    response: String,
}
fn print_log(log: MyLog) {
    println!("func_code is:{}\nrequest is:\n{}\nresponse is:\n{}\n#############\n",
             log.func_code,
             log.request,
             log.response);
}

fn load_file(path: &str) -> String {
    // println!("the file path is   {}", path);
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // let mut s = String::new();
    // let mut result = String::new();
    let mut buffer = Vec::new();
    // try!(file.read_to_end(&mut buffer));//TODO
    file.read_to_end(&mut buffer);
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
    //     Ok(_) => result = result + &s,
    // }
    // for str in buffer {
    //     println!("the str is   {}", str);


    // }
    let mut chars = String::new();
    GBK.decode_to(&buffer, DecoderTrap::Ignore, &mut chars);
    // println!("the str is   {}", chars.to_string());

    return chars.to_string();



}

fn main() {

    let file_content = load_file("/home/bueasy/rust/test1.log");

    let content_v: Vec<&str> = file_content.split("clientId=").collect();
    for con in &content_v {
        if (con.len() > 0) {
            let mut result = con.find("funcCode");
            let mut func_code_index: usize = 0;
            match result {
                Some(x) => func_code_index = x,
                None => func_code_index = 0,
            }
            if (func_code_index > 0) {
                let (first, last) = con.split_at(func_code_index);
                result = last.find("request");
                match result {
                    Some(x) => func_code_index = x,
                    None => func_code_index = 0,
                }
                if (func_code_index > 0) {
                    let (first1, last1) = last.split_at(func_code_index);

                    let mut func_code = first1.replace("funcCode=", "");
                    func_code = func_code.replace("\"", "");
                    // println!("func_code is:{}", func_code);
                    result = last1.find("response:");
                    match result {
                        Some(x) => func_code_index = x,
                        None => func_code_index = 0,
                    }
                    if (func_code_index > 0) {
                        let (first2, last2) = last1.split_at(func_code_index);
                        let request = first2.replace("request:\n", "");
                        let response = last2.replace("response:\n", "");
                        let log = MyLog {
                            func_code: func_code,
                            begin_time: "".to_string(),
                            end_time: "".to_string(),
                            cost: "".to_string(),
                            request: request.to_string(),
                            response: response.to_string(),
                        };
                        print_log(log)

                    }
                }
            }

        }

    }


}
