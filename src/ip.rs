const BAD_FORMAT: &str = "bad format must be formated like 255.255.255.255:8989";
const BAD_VALUE: &str = "all values must be formated has u8.u8.u8.u8.u32";

#[derive(Debug,Clone, Copy,PartialEq,Eq)]
pub struct IPv4 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,

    port: u32,
}

impl IPv4 {
    pub fn from(s: &str) -> Result<Self,&str> {
        let splited_full = s.split(":").collect::<Vec<&str>>();
        let (addr,port) = if splited_full.len() == 2 {
            (splited_full[0],splited_full[1])
        } else {
            return Err(BAD_FORMAT);
        };

        let splited_addr = addr.split(".").collect::<Vec<&str>>();
        let (a,b,c,d) = if splited_addr.len() == 4 {
            (splited_addr[0],splited_addr[1],splited_addr[2],splited_addr[3])
        } else {
            return Err(BAD_FORMAT);
        };

        let a = match a.parse::<u8>() {
            Ok(r) => r,
            Err(_) => return Err(BAD_FORMAT),
        };

        let b = match b.parse::<u8>() {
            Ok(r) => r,
            Err(_) => return Err(BAD_VALUE),
        };

        let c = match c.parse::<u8>() {
            Ok(r) => r,
            Err(_) => return Err(BAD_VALUE),
        };

        let d = match d.parse::<u8>() {
            Ok(r) => r,
            Err(_) => return Err(BAD_VALUE),
        };

        let port = match port.parse::<u32>() {
            Ok(r) => r,
            Err(_) => return Err(BAD_VALUE),
        };
        
        Ok(Self { a: a, b: b, c: c, d: d, port: port })
    }

    pub fn addr(&self) -> String {
        format!("{}.{}.{}.{}",self.a,self.b,self.c,self.d)
    }

    pub fn port(&self) -> String {
        format!("{}",self.port)
    }

    pub fn full(&self) -> String {
        format!("{}:{}",self.addr(),self.port())
    }
}