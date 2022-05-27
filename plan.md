## 문자열 합치는데 1ms 걸림

```rust


// "GET /404 HTTP/1.1\r\nHost: "
// let k = ;
    // let l = host.len();
    // // let k:[u32;0] = [0; 25+l+207];
    // let mut v:Vec<u8> = Vec::with_capacity(25+l+207);

    // let mut a1:Vec<u8>=vec![71, 69, 84, 32, 47, 52, 48, 52, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32];
    // let mut a2 =  Vec::from(host.as_bytes());
    // let mut a3:Vec<u8>=vec![13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 77, 111, 122, 105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 87, 105, 110, 100, 111, 119, 115, 59, 32, 85, 59, 32, 87, 105, 110, 100, 111, 119, 115, 32, 78, 84, 32, 53, 46, 49, 59, 32, 101, 110, 45, 85, 83, 59, 32, 114, 118, 58, 49, 46, 55, 46, 55, 41, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 116, 101, 120, 116, 47, 120, 109, 108, 44, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 120, 109, 108, 44, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 120, 104, 116, 109, 108, 43, 120, 109, 108, 44, 116, 101, 120, 116, 47, 104, 116, 109, 108, 59, 113, 61, 48, 46, 57, 44, 116, 101, 120, 116, 47, 112, 108, 97, 105, 110, 59, 113, 61, 48, 46, 56, 44, 42, 47, 42, 59, 113, 61, 48, 46, 53, 13, 10, 65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101, 58, 32, 101, 110, 45, 117, 115, 44, 101, 110, 59, 113, 61, 48, 46, 53, 13, 10, 13, 10];
    // // // let a4 = [a1,a2];
    // a1.append(&mut a2);
    // a1.append(&mut a3);


    // for i in 0..25 {
    //     let j:usize = i;
    //     v[j] = a1[j]
    // }

// "\r\nUser-Agent: Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US; rv:1.7.7)\r\nAccept: text/xml,application/xml,application/xhtml+xml,text/html;q=0.9,text/plain;q=0.8,*/*;q=0.5\r\nAccept-Language: en-us,en;q=0.5\r\n\r\n"


```

## 시간계산

```rust
use std::time::{Instant};
let start = Instant::now();
let duration = start.elapsed();
println!("Time elapsed in 후처리 is: {:?}", duration);
```

## 차이

- 서버는 2s라고 응답.
- 전송시각 1.2s
- 응답시각 1.8s
- 서버응답 2s
- 서버 처리 시점, 2s ~ 3s 사이

- 시간차는 (3s-1.2s) == 1.8s ~ (2s-1.8s)=0.2s 사이!
- ser + 1 - rev > 서버시각 > ser - sent
- 서버 시계는 내 시계보다. 1.8 ~ 0.2초 사이로 느리게 간다. (offer 음수값) 
- 내 시계가 서서 시계보다 빠르게 간다.
- 서버 시계가 내림인지, 반올림인지 모르지 -> node.js 기준 내림!


## 참조
- https://wiki.wireshark.org/Hyper_Text_Transfer_Protocol
- 여기에, 표준이 나와있음
- https://darksoulstory.tistory.com/62
- https://github.com/EmilHernvall/dnsguide/blob/master/chapter1.md

## dns
- https://www.ietf.org/rfc/rfc1035.txt

### 4.1.1. Header section format

The header contains the following fields:

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

#### ID              
A 16 bit identifier assigned by the program that generates any kind of query.  This identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.

#### QR              
A one bit field that specifies whether this message is a query (0), or a response (1).

#### OPCODE          
A four bit field that specifies kind of query in this message.  This value is set by the originator of a query and copied into the response.  The values are:

##### 0    
- a standard query (QUERY)
##### 1    
- an inverse query (IQUERY)
##### 2    
- a server status request (STATUS)
##### 3-15 
- reserved for future use

#### AA              
Authoritative Answer - this bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question sect Note that the contents of the answer section may have multiple owner names because of aliases.  The AA bit corresponds to the name which matches the query name, the first owner name in the answer section.

#### TC        
TrunCation - specifies that this message was truncated due to length greater than that permitted on the transmission channel.

#### RD          
Recursion Desired - this bit may be set in a query and is copied into the response.  If RD is set, it directs the name server to pursue the query recursively. Recursive query support is optional.

#### RA          
Recursion Available - this be is set or cleared in a response, and denotes whether recursive query support is available in the name server.

#### Z               
Reserved for future use.  Must be zero in all queries and responses.

#### RCODE       
Response code - this 4 bit field is set as part of responses.  The values have the following interpretation:

##### 0               
- No error condition
##### 1               
- Format error - The name server was unable to interpret the query.
##### 2               
- Server failure - The name server was unable to process this query due to a problem with the name server. 
##### 3               
- Name Error - Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist. 
##### 4               
- Not Implemented - The name server does not support the requested kind of query. 
##### 5               
- Refused - The name server refuses to perform the specified operation for policy reasons.  For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (e.g., zone transfer) for particular data.
##### 6-15            
- Reserved for future use.

#### QDCOUNT        
an unsigned 16 bit integer specifying the number of entries in the question section.

#### ANCOUNT     
an unsigned 16 bit integer specifying the number of resource records in the answer section.

#### NSCOUNT     
an unsigned 16 bit integer specifying the number of name server resource records in the authority records section.

#### ARCOUNT     
an unsigned 16 bit integer specifying the number of resource records in the additional records section.

### 4.1.2. Question section format

The question section is used to carry the "question" in most queries, i.e., the parameters that define what is being asked.  The section contains QDCOUNT (usually 1) entries, each of the following format:

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                     QNAME                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QTYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QCLASS                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

#### QNAME       
a domain name represented as a sequence of labels, where each label consists of a length octet followed by that number of octets.  The domain name terminates with the zero length octet for the null label of the root.  Note that this field may be an odd number of octets; no padding is used.

#### QTYPE       
a two octet code which specifies the type of the query. The values for this field include all codes valid for a TYPE field, together with some more general codes which can match more than one type of RR.



#### QCLASS      
a two octet code that specifies the class of the query. For example, the QCLASS field is IN for the Internet.