use core::prelude::*;
use core::mem::{transmute, size_of};
use vec::Vec;

static GDT_SIZE: usize = 3;

extern "C" {
  
  fn lgdt(ptr: *mut GDTReal);
  
}

// TODO made pub to get around error
#[allow(dead_code)]
#[repr(packed)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct GDTEntry {
  bytes: [u8; 8]
}

impl GDTEntry {

  fn new(mut limit: u32, base: u32, typ: u8) -> GDTEntry {

    let mut target: [u8; 8] = [0; 8];

    // adapted from http://wiki.osdev.org/GDT_Tutorial
    // Check the limit to make sure that it can be encoded
    //let mut target: u32 = transmute(targ);
    if (limit > 65536) && (limit & 0xFFF) != 0xFFF {
        //kerror("You can't do that!");
    }
    if limit > 65536 {
        // Adjust granularity if required
        limit = limit >> 12;
        target[6] = 0xC0;
    } else {
        target[6] = 0x40;
    }
 
    // Encode the limit
    target[0] = (limit & 0xFF) as u8;
    target[1] = ((limit >> 8) & 0xFF) as u8;
    target[6] |= ((limit >> 16) & 0xF) as u8;
 
    // Encode the base 
    target[2] = (base & 0xFF) as u8;
    target[3] = ((base >> 8) & 0xFF) as u8;
    target[4] = ((base >> 16) & 0xFF) as u8;
    target[7] = ((base >> 24) & 0xFF) as u8;
 
    // And... Type
    target[5] = typ;
    return GDTEntry { bytes: target } 
  }
  
  fn null() -> GDTEntry {
    GDTEntry { bytes: [0; 8] } 
  }

}

#[repr(packed)]
pub struct GDT {
  table: Vec<GDTEntry>
}

#[allow(dead_code)]
#[repr(packed)]
struct GDTReal {
  limit: u16,
  base: u32
}

impl GDT {
  
  pub fn new() -> GDT {
    let table = Vec::with_capacity(GDT_SIZE);
    GDT {table: table} 
  }
  
  pub fn add_entry(&mut self, base: u32, limit: u32, typ: u8) {
    self.add(GDTEntry::new(limit, base, typ));
  }
  
  pub fn add(&mut self, e: GDTEntry) {
    self.table.push(e);
  }
  
  pub fn enable(&mut self) {
    unsafe {
      let limit: u16 = (GDT_SIZE*size_of::<GDTEntry>()) as u16;
      let (base, _): (u32, u32) = transmute(&self.table[..]);
      let mut real = GDTReal { limit: limit, base: base };
      debug!("limit: {}", limit);
      debug!("base: {}", base);
      
      lgdt(&mut real);
    }
  }
  
  pub fn identity_map(&mut self) {
    self.add(GDTEntry::null());                     // Selector 0x00 cannot be used
    self.add_entry(0, 0xffffffff, 0x9A);         // Selector 0x08 will be our code
    self.add_entry(0, 0xffffffff, 0x92);         // Selector 0x10 will be our data
    //gdt.add_entry( = {.base=&myTss, .limit=sizeof(myTss), .type=0x89}; // You can use LTR(0x18)
  }

}
