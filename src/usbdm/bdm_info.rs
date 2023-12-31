use crate::errors::{BdmError, USBDM_ErrorCode};
use std::fmt;

pub const MINIMAL_VERSION : u32 = 0x040C01;


///`BdmInfo`
///The idea is to group a huge number of USBDM structures, enumerations and settings into three abstractions.
/// 
/// One is BdmInfo - It includes all data information about USBDM, software and hardware versions, buffer sizes
#[derive(Debug, PartialEq)]
pub struct BdmInfo {
    pub bdm_software_version      : u32,           // Version of USBDM Firmware
    pub bdm_hardware_version      : u8,            // Version of USBDM Hardware
    pub icp_software_version      : u8,            // Version of ICP bootloader Firmware
    pub icp_hardware_version      : u8,            // Version of Hardware (reported by ICP code)
    pub capabilities              : Capabilities,  // BDM Capabilities
    pub command_buffer_size       : u16,           // Size of BDM Communication buffer
    pub jtag_buffer_size          : u16,           // Size of JTAG buffer (if supported)
    pub dsc_max_memory_read_size  : u16,
    pub dsc_max_memory_write_size : u16,
}

impl Default for BdmInfo {
    fn default() -> Self { 
        BdmInfo {
            bdm_software_version      : 0,
            bdm_hardware_version      : 0,
            icp_software_version      : 0,
            icp_hardware_version      : 0,
            capabilities              : Capabilities::default(),
            command_buffer_size       : 100,
            jtag_buffer_size          : 100,
            dsc_max_memory_read_size  : 0,
            dsc_max_memory_write_size : 0,

        }
    } 
}

#[derive(Debug, PartialEq)]
pub struct Capabilities {
    pub hcs12:       bool,  // Supports HCS12
    pub rs08:        bool,  // 12 V Flash programming supply available (RS08 support)
    pub vddcontrol:  bool,  // Control over target Vdd
    pub vddsense:    bool,  // Sensing of target Vdd
    pub cfv_x:       bool,  // Support for CFV 1,2 & 3
    pub hcs08:       bool,  // Supports HCS08 targets - inverted when queried
    pub cfv1:        bool,  // Supports CFV1 targets  - inverted when queried
    pub jtag:        bool,  // Supports JTAG targets
    pub dsc:         bool,  // Supports DSC targets
    pub arm_jtag:    bool,  // Supports ARM targets via JTAG
    pub rst:         bool,  // Control & sensing of RESET
    pub pst:         bool,  // Supports PST signal sensing
    pub cdc:         bool,  // Supports CDC Serial over USB interface
    pub arm_swd:     bool,  // Supports ARM targets via SWD
    pub s12z:        bool,  // Supports HCS12Z targets via SWD
}

impl Default for Capabilities {
    fn default() -> Self { 
        Capabilities {
            hcs12:       false,
            rs08:        false,
            vddcontrol:  false,
            vddsense:    false,
            cfv_x:       false,
            hcs08:       false,
            cfv1:        false,
            jtag:        false,
            dsc:         false,
            arm_jtag:    false,
            rst:         false,
            pst:         false,
            cdc:         false,
            arm_swd:     false,
            s12z:        false,
        }
    } 
}


impl fmt::Display for BdmInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl BdmInfo {
    pub fn print_version(&self) {
        println!("bdm_software_version: {:#02}",  &self.bdm_software_version);
        println!("bdm_hardware_version: {:#02}",  &self.bdm_hardware_version);
        println!("icp_software_version: {:#02}",  &self.icp_software_version);
        println!("icp_hardware_version: {:#02}",  &self.icp_hardware_version);
    }

    pub fn print_version2(&self) {
        let v1: u8 = ((&self.bdm_software_version & 0x00FF0000) >> 16) as u8;
        let v2: u8 = ((&self.bdm_software_version & 0x0000FF00) >> 8) as u8;
        let v3: u8 = ((&self.bdm_software_version & 0x000000FF)) as u8;
        println!("bdm_software_version: {}.{}.{}",  v1, v2 ,v3);
        println!("bdm_hardware_version: {:#02X}",  &self.bdm_hardware_version);
        println!("icp_software_version: {:#02X}",  &self.icp_software_version);
        println!("icp_hardware_version: {:#02X}",  &self.icp_hardware_version);
    }

    pub fn print_capabilities(&self) {
        println!("Capabilities: ");
        println!("hcs12: {}",  &self.capabilities.hcs12);
        println!("rs08: {}",  &self.capabilities.rs08);
        println!("vddcontrol: {}",  &self.capabilities.vddcontrol);
        println!("vddsense: {}",  &self.capabilities.vddsense);
        println!("cfv_x: {}",  &self.capabilities.cfv_x);
        println!("hcs08: {}",  &self.capabilities.hcs08);
        println!("cfv1: {}",  &self.capabilities.cfv1);
        println!("jtag: {}",  &self.capabilities.jtag);
        println!("dsc: {}",  &self.capabilities.dsc);
        println!("rst: {}",  &self.capabilities.rst);
        println!("pst: {}",  &self.capabilities.pst);
        println!("cdc: {}",  &self.capabilities.cdc);
        println!("arm_swd: {}",  &self.capabilities.arm_swd);
        println!("s12z: {}",  &self.capabilities.s12z);
    }

    pub fn check_version(&self) -> Result<(), BdmError> {
        if &self.bdm_hardware_version != &self.icp_hardware_version { 
            Err(BdmError::USBDM_Errors(USBDM_ErrorCode::BDM_RC_WRONG_BDM_REVISION))
        } else if &self.bdm_software_version < & MINIMAL_VERSION {
            Err(BdmError::UsbdmFWVersionUnsupported(self.version_in_string().clone(), self.version_in_string_from_u32(MINIMAL_VERSION)))
        } else {
            Ok(())
        }
    }

    pub fn version_in_string(&self) -> String {
        let v1: u8 = ((&self.bdm_software_version & 0x00FF0000) >> 16) as u8;
        let v2: u8 = ((&self.bdm_software_version & 0x0000FF00) >> 8) as u8;
        let v3: u8 = ((&self.bdm_software_version & 0x000000FF)) as u8;
        let str_ver = format!(" Firmware Ver: {}.{}.{}",  v1, v2 ,v3);
        str_ver
    }

    fn version_in_string_from_u32(&self, ver : u32) -> String {
        let v1: u8 = ((ver & 0x00FF0000) >> 16) as u8;
        let v2: u8 = ((ver & 0x0000FF00) >> 8) as u8;
        let v3: u8 = ((ver & 0x000000FF)) as u8;
        let str_ver = format!(" Firmware Ver: {}.{}.{}",  v1, v2 ,v3);
        str_ver
    } 


}



impl Capabilities {
    pub fn parse(&mut self, capabilities: u16 ){
        if (capabilities & (1<<0)) != 0 { self.hcs12 = true; }
        if (capabilities & (1<<1)) != 0 { self.rs08 = true; }
        if (capabilities & (1<<2)) != 0 { self.vddcontrol = true; }
        if (capabilities & (1<<3)) != 0 { self.vddsense = true; }
        if (capabilities & (1<<4)) != 0 { self.cfv_x = true; }
        if (capabilities & (1<<5)) != 0 { self.hcs08 = true; }
        if (capabilities & (1<<6)) != 0 { self.cfv1 = true; }
        if (capabilities & (1<<7)) != 0 { self.jtag = true; }
        if (capabilities & (1<<8)) != 0 { self.dsc = true; }
        if (capabilities & (1<<9)) != 0 { self.arm_jtag = true; }
        if (capabilities & (1<<10)) != 0 { self.rst = true; }
        if (capabilities & (1<<11)) != 0 { self.pst = true; }
        if (capabilities & (1<<12)) != 0 { self.cdc = true; }
        if (capabilities & (1<<13)) != 0 { self.arm_swd = true; }
        if (capabilities & (1<<14)) != 0 { self.s12z = true; }
    }

    pub fn check_dsc_supported(&self) -> Result<(), BdmError> {

        if self.dsc == true
        {
            Ok(())
        }
        else
        {
            return       Err(BdmError::UsbdmUnsuited)

        }


   }
}

