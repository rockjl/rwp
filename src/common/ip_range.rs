/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[derive(Debug, Clone)]
pub(crate) struct IpRange {
    pub(crate) start_ip: (u8, u8, u8, u8),
    pub(crate) ip_s: u32,
    pub(crate) end_ip: (u8, u8, u8, u8),
    pub(crate) ip_e: u32,
}
impl IpRange {
    pub(crate) fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> Self {
        let mut ip_s: u32 = 0;
        ip_s = ip_s << 8 | a as u32;
        ip_s = ip_s << 8 | b as u32;
        ip_s = ip_s << 8 | c as u32;
        ip_s = ip_s << 8 | d as u32;
        let mut ip_e: u32 = 0;
        ip_e = ip_e << 8 | e as u32;
        ip_e = ip_e << 8 | f as u32;
        ip_e = ip_e << 8 | g as u32;
        ip_e = ip_e << 8 | h as u32;
        Self { 
            start_ip: (a, b, c, d, ),
            ip_s,
            end_ip: (e, f, g, h, ),
            ip_e,
        }
    }
    pub(crate) fn check(&self) -> bool {
        if self.start_ip.0 > self.end_ip.0 {
            return false;
        }
        if self.start_ip.1 > self.end_ip.1 {
            return false;
        }
        if self.start_ip.2 > self.end_ip.2 {
            return false;
        }
        if self.start_ip.3 > self.end_ip.3 {
            return false;
        }
        true
    }
    pub(crate) fn is_in_the_range(&self, ip_addr: &str) -> bool {
        if self.ip_s == 0 && self.ip_e == 0 {
            return true;
        }
        let ips: Vec<u8> = ip_addr.split(".").map(|item| {
            item.parse::<u8>().unwrap()
        }).collect();
        if ips.len() != 4 {
            return false;
        }
        let mut ipt: u32 = 0;
        for i in ips {
            ipt = ipt << 8 | i as u32;
        }
        self.ip_s <= ipt && ipt <= self.ip_e
    }
}