// use std::borrow::Borrow;
use std::io::prelude::*;
// use chrono::prelude::*;
use std::net::{TcpStream,SocketAddr,SocketAddrV4,Ipv4Addr,IpAddr}; //

// mod reqtime;
pub mod dns;
pub mod reqtime;
use reqtime::Reqtime;
use dns::DNS;
// use std::time::{Instant};

use chrono::{DateTime,Utc};
// use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
// use chrono::format::ParseError;

// use iced::futures::future::ok;

use std::thread;

const DATE1:[u8;7] = [13,10,68, 65,84, 69, 58];
const DATE2:[u8;7] = [13,10,100,97,116,101,58];
const CRLF :[u8;2] = [13,10];
// }

fn find<'a>(buffer:&'a[u8;1024], word:&'a Vec<&Vec<u8>>, s:usize) -> Result<usize, &'a str>{
    let mut cnt:usize = s;
    let mut f = 0;
    let len = word[0].len();

    while cnt < 1024 {
        let value = buffer[cnt];
        if f >= len {
            return Ok(cnt);
        }
        f = if word.iter().any(|&x| x[f] == value) {
            f + 1
        }else{
            0
        };

        cnt += 1;
    }
    return Err("no word in buffer");
}


type Offset = (i64,i64);

pub struct Servertime{
    pub offset:Option<Offset>,
    pub addr:SocketAddr,
    pub host:String,
    delay:Vec<u64>
}

impl Servertime {
    pub fn new(addr:SocketAddr, host:String) -> Servertime{
        Servertime{
            addr,
            host,
            offset:Option::None,
            delay:Vec::new()
        }
    }

    pub fn gettime(&mut self) -> std::io::Result<Reqtime> {
        // https://wiki.wireshark.org/Hyper_Text_Transfer_Protocol
        
        let req_header = format!("GET /404 HTTP/1.1\r
    Host: {}\r
    User-Agent: Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.7)\r
    Accept: text/xml,application/xml,application/xhtml+xml,text/html;q=0.9,text/plain;q=0.8,*/*;q=0.5\r
    Accept-Language: en-us,en;q=0.5\r\n\r\n", self.host);
    
        let sent = Utc::now();
        let mut stream = TcpStream::connect(self.addr).expect("Couldn't connect to the server...");
        stream.write(String::as_bytes(&req_header))?;
        let mut buffer = [0; 1024]; //1024 byte. 
        stream.read(&mut buffer).unwrap();
        let received = Utc::now();
        let date_start = find(&buffer, &vec![&DATE1.to_vec(),&DATE2.to_vec(),], 0).expect("no Date header in server response");
        let date_end = find(&buffer, &vec![&CRLF.to_vec()], date_start).expect("date header not ended or buffer is too short") - 2;
        let date = &buffer[date_start..date_end];
        let date_string = &*String::from_utf8_lossy(&date);
        let server = DateTime::parse_from_rfc2822(date_string).unwrap().with_timezone(&Utc);
        let myreqtime = Reqtime{sent,received,server};
        Ok(myreqtime)
    } // the stream is closed here

    fn update_offset(&mut self, myreqtimerange:Offset){
        let (s,e) = myreqtimerange;
        let delay = (s-e + 1000) as u64;
        self.delay.push(delay);
        
        match self.offset {
            Some((s,e)) => {
                // println!("{} {}-> {}", s, myreqtimerange.0, s.max(myreqtimerange.0));
                self.offset = Some((
                    s.max(myreqtimerange.0),
                    e.min(myreqtimerange.1)
                ));
            },
            None =>{
                self.offset = Some(myreqtimerange);
            }
        }
    }

    pub fn calculate(&mut self) -> Result<Offset,&str>{

        
        let range = self.gettime().expect("can not make").get_offset_range();
        self.update_offset(range);
        if range.1 - range.0 > 2000{
            return Err("server response is too delayed.")
        }

        for _ in 0..10{
            let range = self.gettime().expect("can not make").get_offset_range();
            self.update_offset(range);
            thread::sleep(std::time::Duration::from_millis(100));
        }

        // do_at_millisec(300,|| {} );

        // 1/3으로 범위 좁히기.
        self.calculate_1_3().unwrap();
        self.calculate_1_3().unwrap();

        if let Some(offset) = self.offset{
            return Ok(offset);
        }else {
            return Err("error");
        }
    }

    fn calculate_1_3(&mut self) -> Result<(),&str>{
        if let Some(offset) = self.offset{
            println!("first {:?}",offset);
            let delay = self.get_delay_median() as i64;
            println!("delay {}",delay);

            let ( s, e) = offset;
            let s = s - delay;

            let q1 = ((s*2 + e)/3) as u64;
            do_at_millisec(q1);
            let range = self.gettime().expect("can not make").get_offset_range();
            self.update_offset(range);
            let q2 = ((s + e*2)/3) as u64;
            do_at_millisec(q2);
            let range = self.gettime().expect("can not make").get_offset_range();
            self.update_offset(range);
            return Ok(())
        }else {
            return Err("error");
        }
    }

    pub fn get_delay_median(&mut self) -> u64{
        self.delay.sort();
        let mid = self.delay.len() / 2;
        return self.delay[mid];
    }

    
}

fn do_at_millisec(mut milli:u64)
// fn do_at_millisec<F>(milli:u64, f:F)
// where F: FnOnce() + 'static
{
    milli %= 1000;
    let thismilli:u64 = Utc::now().timestamp_subsec_millis() as u64;
    let millis:u64 = (milli+1000-thismilli)%1000; //지연시각
    thread::sleep(std::time::Duration::from_millis(millis));
    println!("success??? {}, {}",milli, Utc::now().timestamp_subsec_millis());
    // f();
}



// 기다리기

pub struct ServertimeWait{
    servertime:Option<Servertime>,
    addr:Option<(Box<SocketAddr>, String)>,
    re_url:Regex,
    dns:DNS,
}

use regex::Regex;
// const Re:Regex = Regex::new("/^https?:(\\/\\/)?([^/]+)/").unwrap();

impl ServertimeWait{
    pub fn new() -> ServertimeWait {
        ServertimeWait{
            servertime: None,
            addr: None,
            re_url:Regex::new("^(https?:)?(//)?([^/]+)").unwrap(),
            dns:DNS::new(IpAddr::V4(Ipv4Addr::new(1,1,1,1))),
        }
    }
    

    pub fn add_address(&mut self,address:String) -> Result<(),String>{
        // let re_ip = Regex::new("^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(\\:(\\d{1,5}))$").unwrap();
        // if re_ip.is_match(address){
        //     self.addr = Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(),)))
        // }
        match address.parse::<SocketAddrV4>(){
            Ok(addr) => {
                let k  = (Box::new(SocketAddr::V4(addr)), "localhost".to_string());
                self.addr = Some(k);
                // let s = Servertime::new(SocketAddr::V4(addr), "localhost".to_string());
                return Ok(());
            }
            Err(_) =>{
                // println!("this is not ip address");
            }
        }

        println!("eded");

        let capture = match self.re_url.captures(&address){
            Some(cp) => {cp},
            None => {
                println!("Not in url format");
                return Err(String::from("Not in url format"));
            }
        };

        // println!("{:?}",capture);

        let host = match capture.get(3){
            Some(cp) => {cp.as_str()},
            None => {
                println!("regexp error");
                return Err(String::from("regexp error"));
            }
        };
        

        let ip = match self.dns.get(&host.to_string()){
            Ok(ip) => {ip},
            Err(_) =>{
                println!("not exist site (no dns answer)");
                return Err(String::from("not exist site (no dns answer)"));
            } 
        };

        println!("all, dns answer: {:?}",ip);
        
        self.addr = Some((Box::new(SocketAddr::V4(SocketAddrV4::new(ip,80))), host.to_string()));
        // let sv = Servertime::new(addr, host);
        Ok(())

    }


    pub fn set_server(&mut self)  -> Result<(), ()>{
        if let Some((addr, host)) = self.addr.take() {
            let mut s = Servertime::new(*addr, host);
            if let Err(msg) = s.calculate(){
                println!("[error] msg:{}",msg);
                return Err(())
            }
            self.servertime = Some(s);
            return Ok(());
        }
        return Err(())
    }

    pub fn get_offset_mean(&self) -> (i64,f32){
        // unsafe{
            if let Some(Servertime { offset, ..}) = &self.servertime{
                match offset {
                    Some(offset) => {
                        println!("offset:{:?}",offset);
                        return ((offset.0+offset.1)/2, ((offset.1 - offset.0)as f32)/2.) //.abs()
                    },
                    None => return (0,-1.)
                }
                
            }else{
                return (0,-1.);
            }
        // }
    }
    pub fn get_host(&self) -> String{
        if let Some((_,b)) = &self.addr {
            return b.clone();
        }

        else if let Some(sev) = &self.servertime{
            return sev.host.clone();
        }

        else{
            return String::from("_");
        } 
    }

    // pub fn cal(&mut self) {
    //     match &self.servertime {
    //         Some(mut sv) => {
    //             sv.calculate();
    //         }
    //         _ => {}
    //     }
    // }

    // pub fn reset(&mut self){
    //     // self.addr = None;
    //     self.get_server()
    // }

}


// pub enum ServertimeWrap {
//     Wait(ServertimeWait),
//     Do(Servertime),
// }