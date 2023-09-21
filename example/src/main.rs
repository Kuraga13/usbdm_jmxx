use usbdm_jmxx::usbdm::usb_interface::{UsbInterface, find_usbdm,};
use usbdm_jmxx::usbdm::programmer::Programmer;
use usbdm_jmxx::usbdm::settings::TargetVddSelect;

fn connect_usbdm() -> Option<UsbInterface> {
    
    if let Ok(finded) = find_usbdm() {

        println!("Found usbdm_jmxx");
        println!("Try claim usb & configure descriptors");
        let int = UsbInterface::new(finded).expect("failed to cfg int");
        Some(int)
        
    } else {

        None
    }

}



fn main() {

    if let Some(connected_bdm) = connect_usbdm() {

        let mut prog = Programmer::new(connected_bdm).expect("Programmer::new");
        let msg = "usbdm_mc56f_rs ".to_string() + &"connected ".to_string() + &prog.name.clone() +  &prog.get_string_version().clone();     
        println!(" { } ", msg); 
        prog.init_usbdm_for_mc56f().expect("Failed to cfg usbdm");
        prog.set_vdd(TargetVddSelect::Vdd3V3).expect("Failed to set power 3,3v");
    
        prog.refresh_feedback().expect("Failed to get feedback");
        prog.feedback.print_feedback();
    
        prog.set_vdd(TargetVddSelect::VddOff).expect("Failed to set power OFF");

    } else {
        print!(" BDM not found ");
    }
        
 
}