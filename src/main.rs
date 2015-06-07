#![allow(non_camel_case_types)]

use std::io::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::File;

type Elf64_Half = u16;

type Elf64_Word = u32;

type Elf64_Addr = u64;

type Elf64_Off = u64;

const EI_NIDENT : usize = 16;

#[repr(C)]
#[derive(Debug)]
struct ElfIdent {
    data: [u8; EI_NIDENT],
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiClass {
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
}

impl Display for ElfEiClass {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiClass::*;
        let s = match *self {
            ELFCLASSNONE => "None",
            ELFCLASS32 => "ELF32",
            ELFCLASS64 => "ELF64",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiData {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
}

impl Display for ElfEiData {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiData::*;
        let s = match *self {
            ELFDATANONE => "None",
            ELFDATA2LSB => "2's complement, little endian",
            ELFDATA2MSB => "2's complement, big endian",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiVersion {
    EV_NONE,
    EV_CURRENT,
}

impl Display for ElfEiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiVersion::*;
        let s = match *self {
            EV_NONE => "None",
            EV_CURRENT => "1 (current)",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum ElfEiOsAbi {
    ELFOSABI_NONE = 0,
    ELFOSABI_HPUX = 1,
    ELFOSABI_NETBSD = 2,
    ELFOSABI_GNU = 3,
    ELFOSABI_SOLARIS = 6,
    ELFOSABI_AIX = 7,
    ELFOSABI_IRIX = 8,
    ELFOSABI_FREEBSD = 9,
    ELFOSABI_TRU64 = 10,
    ELFOSABI_MODESTO = 11,
    ELFOSABI_OPENBSD = 12,
    ELFOSABI_ARM_AEABI = 64,
    ELFOSABI_ARM = 97,
    ELFOSABI_STANDALONE = 255
}

#[allow(dead_code)]
const ELFOSABI_SYSV: u8 = ElfEiOsAbi::ELFOSABI_NONE as u8;
#[allow(dead_code)]
const ELFOSABI_LINUX: u8 = ElfEiOsAbi::ELFOSABI_GNU as u8;

impl Display for ElfEiOsAbi {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEiOsAbi::*;
        let s = match *self {
            ELFOSABI_NONE => "UNIX - System V",
            ELFOSABI_HPUX => "HP-UX",
            ELFOSABI_NETBSD => "NetBSD",
            ELFOSABI_GNU => "GNU ELF",
            ELFOSABI_SOLARIS => "Sun Solaris",
            ELFOSABI_AIX => "IBM AIX",
            ELFOSABI_IRIX => "SGI Irix",
            ELFOSABI_FREEBSD => "FreeBSD",
            ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
            ELFOSABI_MODESTO => "Novell Modesto",
            ELFOSABI_OPENBSD => "OpenBSD",
            ELFOSABI_ARM_AEABI => "ARM EABI",
            ELFOSABI_ARM => "ARM",
            ELFOSABI_STANDALONE => "Standalone (embedded) application",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
struct ElfEiAbiVersion {
    data: u8,
}

impl Display for ElfEiAbiVersion {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.data)
    }
}



#[repr(C)]
#[derive(Debug)]
struct ElfIdentNamed {
    ei_magic: [u8; 4],
    ei_class: ElfEiClass,
    ei_data: ElfEiData,
    ei_version: ElfEiVersion,
    ei_osabi: ElfEiOsAbi,
    ei_osabiversion: ElfEiAbiVersion,
    padding2: [u8; 7],
}

impl Display for ElfIdent {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for b in self.data.iter() {
            try!(
                write!(
                    fmt, "{:02x} ", b));
        }
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug,PartialEq,PartialOrd,Eq,Ord)]
#[allow(dead_code)]
enum ElfEhdrType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_LOPROC = 0xff00,
    ET_HIPROC = 0xffff,
}

impl Display for ElfEhdrType {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use ElfEhdrType::*;
        let s = match *self {
            ET_NONE => "NONE (No file type)",
            ET_REL => "REL (Relocatable file)",
            ET_EXEC => "EXEC (Executable file)",
            ET_DYN => "DYN (Shared object file)",
            ET_CORE => "CORE (Core file)",
            ref x if *x >= ET_LOPROC && *x <= ET_HIPROC => "Processor-specific",
            _ => "Unknown file type",
        };
        write!(fmt, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug)]
struct Elf64_Ehdr {
    e_ident: ElfIdent,
    e_type: ElfEhdrType,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half
}

impl Display for Elf64_Ehdr {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let ehdr_ident: &ElfIdentNamed = unsafe {
            std::mem::transmute(&self.e_ident)
        };

        write!(
            fmt,
            concat!(
                "ELF Header:\n",
                "  Magic:   {}\n",
                "  Class:                             {}\n",
                "  Data:                              {}\n",
                "  Version:                           {}\n",
                "  OS/ABI:                            {}\n",
                "  ABI Version:                       {}\n",
                "  Type:                              {}\n",
                "  Machine:                           {:?}\n",
                "  Version:                           {:#x}\n",
                "  Entry point address:               {:#x}\n",
                "  Start of program headers:          {} (bytes into file)\n",
                "  Start of section headers:          {} (bytes into file)\n",
                "  Flags:                             {:#x}\n",
                "  Size of this header:               {} (bytes)\n",
                "  Size of program headers:           {} (bytes)\n",
                "  Number of program headers:         {}\n",
                "  Size of section headers:           {} (bytes)\n",
                "  Number of section headers:         {}\n",
                "  Section header string table index: {}",
                ),
            self.e_ident,
            ehdr_ident.ei_class,
            ehdr_ident.ei_data,
            ehdr_ident.ei_version,
            ehdr_ident.ei_osabi,
            ehdr_ident.ei_osabiversion,
            self.e_type,
            self.e_machine,
            self.e_version,
            self.e_entry,
            self.e_phoff,
            self.e_shoff,
            self.e_flags,
            self.e_ehsize,
            self.e_phentsize,
            self.e_phnum,
            self.e_shentsize,
            self.e_shnum,
            self.e_shstrndx)
    }
}

fn work() {
    let f = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut b = Vec::<u8>::with_capacity(std::mem::size_of::<Elf64_Ehdr>());
    f.take(std::mem::size_of::<Elf64_Ehdr>() as u64).read_to_end(&mut b).unwrap();

    let proper_magic = &[0x7f, b'E', b'L', b'F'];
    let magic_ptr: *const [u8; 4] = unsafe {
        std::mem::transmute(b.as_ptr())
    };
    let magic = unsafe { &*magic_ptr };
    if proper_magic != magic {
        panic!("Not an ELF file");
    }

    let ehdr_ptr: *const Elf64_Ehdr = unsafe {
        std::mem::transmute(b.as_ptr())
    };
    let ehdr: &Elf64_Ehdr = unsafe { &*ehdr_ptr };

    println!("{}", ehdr);
}

fn _static_asserts() {
    let ei_bytes: ElfIdent = unsafe {
        std::mem::uninitialized()
    };
    let _ei_named: ElfIdentNamed = unsafe {
        std::mem::transmute(ei_bytes)
    };

    let ehdr_type_bytes: Elf64_Half = unsafe {
        std::mem::uninitialized()
    };
    let _ehdr_type: ElfEhdrType = unsafe {
        std::mem::transmute(ehdr_type_bytes)
    };
}

fn main() {
    work();
}
