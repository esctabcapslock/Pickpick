use std::net::{TcpStream,IpAddr,SocketAddr, Ipv4Addr}; //UdpSocket,
use std::io::prelude::*;

#[derive(Debug)]
pub struct DNS{
    addr:IpAddr,

}

const DNS_BEFORE_HEADER:[u8;14] =  [0,31, //, 길이
    77,77,//id
    0x01,0x00,
    0,1,0,0,
    0,0,0,0,];

const DNS_AFTER_HEADER:[u8;4] =  [0,1,0,1,];

impl DNS {
    pub fn new(addr: IpAddr)->DNS{
        DNS{
            addr
        }
    }
    pub fn get(&self, host:&String) -> Result<Ipv4Addr, &str>{
        
        // println!("[DNS]");
        // let req_header = format!("aa{}",host);
        let host_buffer = host.as_bytes();
        let mut header:[u8;1024] = [0;1024];

        

        let mut cnt:usize = 0;
        for i in DNS_BEFORE_HEADER{
            header[cnt] = i;
            cnt += 1;
        }

        // length
        header[0] = ((14+4+host_buffer.len())/256) as u8;
        header[1] = (14+4+host_buffer.len()) as u8;
        
        
        let c:Vec<usize> = host.split(".").into_iter().map(|v| v.len() as usize).collect();
        // println!("{:?}",c);
        header[cnt] = c[0] as u8;
        cnt += 1;

        let mut c_index:usize = 1;
        for i in host_buffer{
            if *i == 46{
                header[cnt] = c[c_index] as u8;
                c_index += 1;
            }else{
                header[cnt] = *i
            }
            cnt += 1;
        }
        header[cnt] = 0;
        cnt += 1;

        for i in DNS_AFTER_HEADER{
            header[cnt] = i;
            cnt += 1;
        }


        
        // let mut reqbuffer = String::as_bytes(&req_header);
        // let len = reqbuffer.len() as u16;

        // let mut reqbuffer:[u8;15] = [0,13, 119, 119, 119, 46, 110, 97, 118, 101, 114, 46, 99, 111, 109];

        // let mut reqbuffer = [
        // 0,31, //, 길이
        // 77,77,//id
        // 0x01,0x00,
        // 0,1,0,0,
        // 0,0,0,0,
        // 3, 119, 119, 119, 5, 110, 97, 118, 101, 114, 3, 99, 111, 109,0,
        // 0,1,
        // 0,1,
        // ];
        // 73, 78


        // let mut reqbuftpvec:Vec<u8> = vec![(len/256) as u8, len as u8];
        // let mut len = [(len/256) as u8, len as u8];
        // reqbuftpvec.append(&mut reqbuffer.to_vec());
        // let req = demo::<>(reqbuftpvec);
        //reqbuffer[0] = (len/256) as u8;
        //reqbuffer[1] = len as u8;

// SocketAddr::new(*self.addr,53)
        // let socket = UdpSocket::bind("8.8.8.8:53").expect("111111111111");
        // let (amt, src) = socket.recv_from(&mut reqbuffer).expect("2222222222");
        
        // let mut buf:[u8;1024] = [0; 1024];
        // // let buf = &mut buf[..amt];
        // buf.reverse();
        // socket.send_to(&buf, &src);

        // println!("raw_buffer: {:?}",&header[..cnt]);



        let mut stream = match TcpStream::connect(SocketAddr::new(self.addr.clone(),53)){
            Ok(stream) => stream,
            Err(err) =>{
                println!("[dns error] {:?}",err);
                return Err("Couldn't connect to the DNS\nCheck your Internet connection");
            }
        };
        // stream.write(String::as_bytes(&req_header)).unwrap();
        // stream.write(&len);
        match stream.write(&header[..cnt]){
            Ok(_) => {},
            Err(_) => {return Err("DNS can't write"); }
        }

        let mut buffer = [0; 256]; //1024 byte. 
        // let f = ;
        match stream.read(&mut buffer) {
            Ok(_) => {},
            Err(_) => {return Err("DNS can't read"); }
        };
        // println!("raw_buffer: {:?}",buffer);
        // println!("res: {}", String::from_utf8_lossy(&buffer));

        let len = ((buffer[0] as u16)*256 + (buffer[1] as u16)) as usize;
        
        
        // println!("len: {}",len);
        // println!("id: {} {}",buffer[2], buffer[3]);
        // println!("QR: {}",buffer[4]>>7);
        // println!("OPCODE: {}",(buffer[4]&127)>>3);
        // println!("AA: {}",(buffer[4]&4)>>2);
        // println!("TC: {}",(buffer[4]&2)>>1);
        // println!("RD: {}",(buffer[4]&1));
        // println!("RA: {}",buffer[5]>>7);
        // println!("Z: {}",(buffer[5]&127)>>4);
        // println!("RCODE: {}",(buffer[5]&15));
        // println!("QDCOUNT: {} {}",buffer[6], buffer[7]);
        // println!("ANCOUNT: {} {}",buffer[8], buffer[9]);
        // println!("NSCOUNT: {} {}",buffer[10], buffer[11]);
        // println!("ARCOUNT: {} {}",buffer[12], buffer[13]);
        // println!("Q:{:?}",&buffer[14..cnt]);

        if len<cnt {
            println!("len too sort");
            return Err("not exist site (no dns answer)");
        }


        // println!("QDCO: {:?}", &buffer[cnt..len]);
        // println!("Name: {} {}", buffer[cnt], buffer[cnt+1]);
        // println!("Type: {} {}", buffer[cnt+2], buffer[cnt+3]);
        // println!("Class: {} {}", buffer[cnt+4], buffer[cnt+5]);
        // println!("TTL: {} {} {} {}", buffer[cnt+6], buffer[cnt+7], buffer[cnt+8] , buffer[cnt+9]);
        // println!("Len: {} {}", buffer[cnt+10], buffer[cnt+11]);

        
        
        if len < 13 {
            println!("no size");
            Err("not exist site (no dns answer)")
        }else if (buffer[cnt+2], buffer[cnt+3]) != (0, 1) && (buffer[cnt+2], buffer[cnt+3]) != (0, 5){
            println!("no size");
            Err("not exist site (no dns answer)")
        }else{
            Ok(Ipv4Addr::new(buffer[len-2], buffer[len-1], buffer[len], buffer[len+1]))
        }

    }
}