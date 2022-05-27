use std::net::{UdpSocket,TcpStream,IpAddr,SocketAddr, Ipv4Addr};
use std::io::prelude::*;

pub struct DNS<'a>{
    addr:&'a IpAddr,

}

//>>> a="www.naver.com"
// [*map(ord,a)]
// use std::convert::TryInto;
// fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
//     v.try_into()
//         .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

const DNS_BEFORE_HEADER:[u8;14] =  [0,31, //, 길이
    77,77,//id
    0x01,0x00,
    0,1,0,0,
    0,0,0,0,];

const DNS_AFTER_HEADER:[u8;4] =  [0,1,0,1,];

impl <'a> DNS <'a> {
    pub fn new(addr:&'a IpAddr)->DNS<'a>{
        DNS{
            addr
        }
    }
    pub fn get(&self, host:&String) -> Ipv4Addr{
        
        // println!("[DNS]");
        // let req_header = format!("aa{}",host);
        let mut host_buffer = host.as_bytes();
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



        let mut stream = TcpStream::connect(SocketAddr::new(*self.addr,53)).expect("Couldn't connect to the server...");
        // stream.write(String::as_bytes(&req_header)).unwrap();
        // stream.write(&len);
        stream.write(&header[..cnt]);

        let mut buffer = [0; 256]; //1024 byte. 
        stream.read(&mut buffer).unwrap();
        // println!("raw_buffer: {:?}",buffer);
        // println!("res: {}", String::from_utf8_lossy(&buffer));

        let len = ((buffer[0] as u16)*256 + (buffer[1] as u16)) as usize;
        Ipv4Addr::new(buffer[len-2], buffer[len-1], buffer[len], buffer[len+1])
    }
}