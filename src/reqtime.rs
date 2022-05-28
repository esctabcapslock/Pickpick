
use chrono::{DateTime,Utc,};


#[derive(Debug)]
pub struct Reqtime{
    pub sent:DateTime<Utc>,
    pub received :DateTime<Utc>,
    pub server:DateTime<Utc>,
}

// #[warn(dead_code)]
// #[allow(unused_results)]
#[allow(dead_code)]
fn printdate(date:&DateTime<Utc>){
    let m = date.timestamp_subsec_millis();
    let s = date.timestamp()&100;
    println!("{}s {}ms",s,m)
}

impl Reqtime {
    pub fn get_offset_range(&self) -> (i64, i64){
        let before_offset = (self.received - self.server).num_milliseconds() - 1000;
        let affer_offset  = (self.sent     - self.server).num_milliseconds();
        //   rev - sev - 1 < 서버시각 < sent-sev -->
        println!("{}ms < offet < {}ms dur:{}, for sent:{}, rev:{}", before_offset, affer_offset, before_offset - affer_offset + 1000, self.sent.timestamp_subsec_millis(), self.received.timestamp_subsec_millis());
        return (before_offset, affer_offset)

        // before_offset - affer_offset + 1000 소요시간
    }
}

