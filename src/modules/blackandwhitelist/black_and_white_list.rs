/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use std::io::Read;
use crate::{
    common::ip_range::IpRange, context::{tcp_context, ContextType}, error::{ConfigError, ConfigErrorKind, GatewayError, RResult}, 
    modules::{ModuleType, PipeData, PipeModule,}
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BlackAndWhiteList {}

impl PipeModule for BlackAndWhiteList {
    fn name(&self) -> ModuleType {
        ModuleType::BlackAndWhiteList
    }
    
    async fn execute(&self, mut ctx: crate::context::GatewayContext, pipe_data: &crate::modules::PipeData) -> RResult<crate::context::GatewayContext>  {
        if let PipeData::BlackAndWhiteListData { profile } = pipe_data {
            match &mut ctx.context_type {
                ContextType::HttpContext(http_context) => {
                    let ip_str = http_context.request_context.remote_addr.ip().to_string();
                    let profile_read_lock = profile.read().await;
                    if profile_read_lock.has_white_list() {
                        if !profile_read_lock.check_whitelist(&ip_str) {
                            return Err(gateway_err!(BlackAndWhiteListError, "not in whitelist", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    }
                    if profile_read_lock.has_black_list() {
                        if profile_read_lock.check_blacklist(&ip_str) {
                            return Err(gateway_err!(BlackAndWhiteListError, "in blacklist", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    }
                    drop(profile_read_lock);
                },
                ContextType::TcpContext(tcp_context) => {
                    let ip_str = ctx.remote_addr.ip().to_string();
                    let profile_read_lock = profile.read().await;
                    if profile_read_lock.has_white_list() {
                        if !profile_read_lock.check_whitelist(&ip_str) {
                            return Err(gateway_err!(BlackAndWhiteListError, "not in whitelist", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    }
                    if profile_read_lock.has_black_list() {
                        if profile_read_lock.check_blacklist(&ip_str) {
                            return Err(gateway_err!(BlackAndWhiteListError, "in blacklist", ConfigError::new(ConfigErrorKind::BLACKANDWHITE)));
                        }
                    }
                    drop(profile_read_lock);
                }
            }
            return Ok(ctx);
        }
        unreachable!()
    }
}

//-------------------------------------profile-----------------------------------
#[derive(Debug, Clone, Default)]
pub(crate) struct BlackAndWhiteListProfile {
    inner: BlackAndWhiteListInitData,
    black_list: Vec<BaW>,
    white_list: Vec<BaW>,
}
#[derive(Debug, Clone)]
pub(crate) enum BaW {
    Single((u8,u8,u8,u8)),
    Range(IpRange)
}
impl BaW {
    pub(crate) fn validate(&self) -> bool {
        match self {
            BaW::Range(ip_range) => {
                if ip_range.start_ip.0 > ip_range.end_ip.0 {
                    return false;
                }
                if ip_range.start_ip.1 > ip_range.end_ip.1 {
                    return false;
                }
                if ip_range.start_ip.2 > ip_range.end_ip.2 {
                    return false;
                }
                if ip_range.start_ip.3 > ip_range.end_ip.3 {
                    return false;
                }
            }
            BaW::Single(ip) => {

            }
        }
        true
    }
    pub(crate) fn check_ip(&self, ip_addr: &str) -> bool {
        match self {
            BaW::Range(ip_range) => {
                if ip_range.ip_s == 0 && ip_range.ip_e == 0 {
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
                ip_range.ip_s <= ipt && ipt <= ip_range.ip_e
            }
            BaW::Single((a, b, c, d)) => {
                if a==&0 && b==&0 && c==&0 && d==&0 {
                    return true;
                }
                let v: Vec<&str> = ip_addr.split(".").collect();
                if a.clone() == v.get(0).unwrap().parse::<u8>().unwrap()
                    && b.clone() == v.get(1).unwrap().parse::<u8>().unwrap()
                    && c.clone() == v.get(2).unwrap().parse::<u8>().unwrap()
                    && d.clone() == v.get(3).unwrap().parse::<u8>().unwrap() {
                    true
                } else {
                    false
                }
            }
        }
        
    }
}
impl Default for BaW {
    fn default() -> Self {
        Self::Single((0,0,0,0))
    }
}
#[derive(Debug, Clone)]
pub(crate) struct BlackAndWhiteListInitData {
    pub(crate) blacklist: BawFileOrMemory,
    pub(crate) whitelist: BawFileOrMemory,
}
#[derive(Debug, Clone)]
pub(crate) enum BawFileOrMemory {
    File(String),
    Memory(Vec<String>),
    None,
}
impl Default for BlackAndWhiteListInitData {
    fn default() -> Self {
        Self { blacklist: BawFileOrMemory::File(String::new()), whitelist: BawFileOrMemory::File(String::new()) }
    }
}
impl BlackAndWhiteListProfile {
    pub(crate) fn new(init_data: BlackAndWhiteListInitData) -> RResult<Self> {
        let black_list = match &init_data.blacklist {
            BawFileOrMemory::File(file_name) => {
                BlackAndWhiteListProfile::parse_list_from_file(&file_name)?
            }
            BawFileOrMemory::Memory(list) => {
                let mut bl = Vec::new();
                for line in list {
                    let baw = BlackAndWhiteListProfile::parse_baw(line)?;
                    bl.push(baw);
                }
                bl
            }
            BawFileOrMemory::None => { Vec::new() }
        };
        let white_list = match &init_data.whitelist {
            BawFileOrMemory::File(file_name) => {
                BlackAndWhiteListProfile::parse_list_from_file(&file_name)?
            }
            BawFileOrMemory::Memory(list) => {
                let mut wl = Vec::new();
                for line in list {
                    let baw = BlackAndWhiteListProfile::parse_baw(line)?;
                    wl.push(baw);
                }
                wl
            }
            BawFileOrMemory::None => { Vec::new() }
        };
        Ok(Self {
            inner: init_data,
            black_list,
            white_list,
        })
    }
    pub(crate) fn check_whitelist(&self, ip: &str) -> bool {
        for wl in &self.white_list {
            if wl.check_ip(ip) {
                return true;
            }
        }
        false
    }
    pub(crate) fn check_blacklist(&self, ip: &str) -> bool {
        for bl in &self.black_list {
            if bl.check_ip(ip) {
                return true;
            }
        }
        false
    }
    pub(crate) fn has_black_list(&self) -> bool {
        if let BawFileOrMemory::None = self.inner.blacklist {
            false
        } else {
            true
        }
    }
    pub(crate) fn has_white_list(&self) -> bool {
        if let BawFileOrMemory::None = self.inner.whitelist {
            false
        } else {
            true
        }
    }
    pub(crate) fn parse_list_from_file(file_path: &str) -> RResult<Vec<BaW>> {
        if file_path.is_empty() {
            return Ok(Vec::new());
        }
        let mut ret = Vec::new();
        let file_content = BlackAndWhiteListProfile::read_file(file_path)?;
        let file_content_lines: Vec<&str> = file_content.lines().filter(|s| !s.is_empty() ).collect();
        for line in file_content_lines {
            let baw = BlackAndWhiteListProfile::parse_baw(line)?;
            ret.push(baw);
        }
        Ok(ret)
    }
    fn parse_baw(line: &str) -> RResult<BaW> {
        let v: Vec<&str> = line.split('-').filter(|c| !c.is_empty()).collect();
        match v.len() {
            1 => {
                let ip = v.get(0).unwrap();
                let ips: Vec<&str> = ip.split('.').filter(|c| !c.is_empty() ).collect();
                if ips.len() != 4 {
                    return Err(gateway_err!(ConfigurationFailed, "Config black or white list failed Single IP len failed.", ConfigError::new(ConfigErrorKind::BAWTCPIP)));
                }
                return Ok(BaW::Single((
                    ips.get(0).unwrap().parse()?,
                    ips.get(1).unwrap().parse()?,
                    ips.get(2).unwrap().parse()?,
                    ips.get(3).unwrap().parse()?,
                )));
            }
            2 => {
                let start_ip = v.get(0).unwrap();
                let end_ip = v.get(1).unwrap();
                let start_ips: Vec<&str> = start_ip.split('.').filter(|c| !c.is_empty() ).collect();
                let end_ips: Vec<&str> = end_ip.split('.').filter(|c| !c.is_empty() ).collect();
                let a = start_ips.get(0).unwrap().parse()?;
                let b = start_ips.get(1).unwrap().parse()?;
                let c = start_ips.get(2).unwrap().parse()?;
                let d = start_ips.get(3).unwrap().parse()?;
                let e = end_ips.get(0).unwrap().parse()?;
                let f = end_ips.get(1).unwrap().parse()?;
                let g = end_ips.get(2).unwrap().parse()?;
                let h = end_ips.get(3).unwrap().parse()?;
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
                let baw = BaW::Range ( IpRange { 
                    start_ip: (a, b, c, d, ),
                    ip_s,
                    end_ip: (e, f, g, h, ),
                    ip_e,
                } );
                if !baw.validate() {
                    return Err(gateway_err!(ConfigurationFailed, "Config black or white list failed Ip Range failed", ConfigError::new(ConfigErrorKind::BAWTCPIP)));    
                }
                return Ok(baw)
            }
            _ => {
                return Err(gateway_err!(ConfigurationFailed, "Config black or white list failed", ConfigError::new(ConfigErrorKind::BAWTCPIP)));
            }
        }
    }
    fn read_file(file_path: &str) -> RResult<String> {
        let mut file = std::fs::OpenOptions::new().read(true).open("examples/test.txt")?;
        let mut buffer = String::new();
        let read_buffer = file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}