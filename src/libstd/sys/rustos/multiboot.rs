#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::prelude::*;
use core::mem::transmute;

// adapted from multiboot.h

/* The magic field should contain this. */
static MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;

/* This should be in %eax. */
pub static MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BADB002;

/* The bits in the required part of flags field we don't support. */
static MULTIBOOT_UNSUPPORTED: u32 = 0x0000fffc;

/* Alignment of multiboot modules. */
static MULTIBOOT_MOD_ALIGN: u32 = 0x00001000;

/* Alignment of the multiboot info structure. */
static MULTIBOOT_INFO_ALIGN: u32 = 0x00000004;

/* Flags set in the 'flags' member of the multiboot header. */

/* Align all boot modules on i386 page (4KB) boundaries. */
static MULTIBOOT_PAGE_ALIGN: u32 = 0x00000001;

/* Must pass memory information to OS. */
static MULTIBOOT_MEMORY_INFO: u32 = 0x00000002;

/* Must pass video information to OS. */
static MULTIBOOT_VIDEO_MODE: u32 = 0x00000004;

/* This flag indicates the use of the address fields in the header. */
static MULTIBOOT_AOUT_KLUDGE: u32 = 0x00010000;

/* Flags to be set in the 'flags' member of the multiboot info structure. */

/* is there basic lower/upper memory information? */
static MULTIBOOT_INFO_MEMORY: u32 = 0x00000001;
/* is there a boot device set? */
static MULTIBOOT_INFO_BOOTDEV: u32 = 0x00000002;
/* is the command-line defined? */
static MULTIBOOT_INFO_CMDLINE: u32 = 0x00000004;
/* are there modules to do something with? */
static MULTIBOOT_INFO_MODS: u32 = 0x00000008;

/* These next two are mutually exclusive */

/* is there a symbol table loaded? */
static MULTIBOOT_INFO_AOUT_SYMS: u32 = 0x00000010;
/* is there an ELF section header table? */
static MULTIBOOT_INFO_ELF_SHDR: u32 = 0x00000020;

/* is there a full memory map? */
static MULTIBOOT_INFO_MEM_MAP: u32 = 0x00000040;

/* Is there drive info? */
static MULTIBOOT_INFO_DRIVE_INFO: u32 = 0x00000080;

/* Is there a config table? */
static MULTIBOOT_INFO_CONFIG_TABLE: u32 = 0x00000100;

/* Is there a boot loader name? */
static MULTIBOOT_INFO_BOOT_LOADER_NAME: u32 = 0x00000200;

/* Is there a APM table? */
static MULTIBOOT_INFO_APM_TABLE: u32 = 0x00000400;

/* Is there video information? */
static MULTIBOOT_INFO_VIDEO_INFO: u32 = 0x00000800;

struct multiboot_header {
  /* Must be MULTIBOOT_MAGIC - see above. */
  magic: u32,
  /* Feature flags. */
  flags: u32,

  /* The above fields plus this one must equal 0 mod 2^32. */
  checksum: u32,

  /* These are only valid if MULTIBOOT_AOUT_KLUDGE is set. */
  header_addr: u32,
  load_addr: u32,
  load_end_addr: u32,
  bss_end_addr: u32,
  entry_addr: u32,

  /* These are only valid if MULTIBOOT_VIDEO_MODE is set. */
  mode_type: u32,
  width: u32,
  height: u32,
  depth: u32
}

/* The symbol table for a.out. */
struct multiboot_aout_symbol_table {
  tabsize: u32,
  strsize: u32,
  addr: u32,
  reserved: u32
}

/* The section header table for ELF. */
struct multiboot_elf_section_header_table {
  num: u32,
  size: u32,
  addr: u32,
  shndx: u32
}

pub struct multiboot_info {
  /* Multiboot info version number */
  flags: u32,

  /* Available memory from BIOS */
  mem_lower: u32,
  mem_upper: u32,

  /* "root" partition */
  boot_device: u32,

  /* Kernel command line */
  cmdline: u32,

  /* Boot-Module list */
  mods_count: u32,
  mods_addr: u32,

  //union
  //{
    aout_sym: multiboot_aout_symbol_table,
    //multiboot_elf_section_header_table_t elf_sec;
  //} u;

  /* Memory Mapping buffer */
  mmap_length: u32,
  mmap_addr: u32,

  /* Drive Info buffer */
  drives_length: u32,
  drives_addr: u32,

  /* ROM configuration table */
  config_table: u32,

  /* Boot Loader Name */
  boot_loader_name: u32,

  /* APM table */
  apm_table: u32,

  /* Video */
  vbe_control_info: u32,
  vbe_mode_info: u32,
  vbe_mode: u16,
  vbe_interface_seg: u16,
  vbe_interface_off: u16,
  vbe_interface_len: u16,
}

impl multiboot_info {

  fn has_flag(&self, flag_number: u8) -> bool {
    return (self.flags >> flag_number as usize) & 0x1 == 0x1;
  }

  pub unsafe fn multiboot_stuff(&self) {

    /* Print out the flags. */
    debug!("flags = 0x{:x}", self.flags);

    if self.has_flag(6) {
      debug!("mmap_addr = 0x{:x}", self.mmap_addr);
      debug!("mmap_length = 0x{:x}", self.mmap_length);

      let mut current: u32 = self.mmap_addr;
      while current < self.mmap_addr + self.mmap_length {
        let e: *mut multiboot_mmap_entry = transmute(current);
        if (*e).typ == 1 {
          debug!("at 0x{:x}", current);
          debug!("  size: 0x{:x}", (*e).size);
          debug!("  addr: 0x{:x}", (*e).addr as u32); // TODO(ryan): if 64-bit arg, then crashes !
          debug!("  length: 0x{:x}", (*e).len as u32);
          debug!("  type: 0x{:x}", (*e).typ);
        }
        current += (*e).size + 4;
      }
    } else {
      debug!("no memmap :(");
    }
  }
}


#[repr(packed)]
struct multiboot_mmap_entry {
  size: u32,
  addr: u64,
  len: u64,
  typ: u32
}

static MULTIBOOT_MEMORY_AVAILABLE: u32 = 1;
static MULTIBOOT_MEMORY_RESERVED: u32 = 2;

struct multiboot_mod_list {
  /* the memory used goes from bytes 'mod_start' to 'mod_end-1' inclusive */
  mod_start: u32,
  mod_end: u32,

  /* Module command line */
  cmdline: u32,

  /* padding to take it to 16 bytes (must be zero) */
  pad: u32,
}
