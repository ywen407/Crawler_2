use crate::agent::agent::Agent;
use crate::util::urlparser::UrlParser;
use std::fs;
use chrono::Local;
use std::error::Error;
use queues::*;

impl Agent {
    /* implement */
    /*
        웹 페이지 내용의 128-bit checksum을 구하여 웹페이지의 중복 수집 여부 점검.
        0)중복 여부를 체크한다.
        1) 웹 페이지를 저장한다.
        2) 웹 페이지의 128-bit checksum 값을 구하고 Contents table에 추가한다.
        3) 웹 페이지의 URL을 Visited Queue에 추가한다.
    */

    pub fn replicate_check(&mut self,url: UrlParser,res_data: &Vec<u8>)->Result<(), Box<dyn Error>> {
            let res_data_byte =&res_data[..];
            let digest = md5::compute(res_data_byte);
            if !self.contents_table.contains(&digest){
                self.contents_table.insert(digest);
                
                let stamp =Local::now();  
                fs::create_dir_all("./Download/")?;
                fs::File::create("./Download/".to_owned()+&url.host() + &stamp.to_rfc3339()[..])?;
                fs::write("./Download/".to_owned()+&url.host()+ &stamp.to_rfc3339()[..],&res_data)?;

                self.visited_queue.add(self.to_visit_queue.peek()?)?;
                self.to_visit_queue.remove()?;
            }
            Ok(())
        }
}
