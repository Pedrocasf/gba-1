/// The CGA [Code Page 437][cp437] type face, with thick lines.
///
/// There's 256 tiles, packed down to 1bpp. To decompress this easily you can
/// call [`BitUnPack`] as follows:
/// ```no_run
/// # use gba::prelude::*;
/// # use gba::art::CGA_8X8_THICK;
/// # use core::convert::TryFrom;
/// # use core::mem::size_of_val;
/// let info = UnpackInfo {
///   source_data_len_bytes: size_of_val(&CGA_8X8_THICK) as usize,
///   source_unit_bit_width: 1,
///   // this assumes we want to unpack to 4bpp tiles
///   destination_unit_bit_width: 4,
///   data_offset: 0,
/// };
/// // our example unpacks directly to the start of VRAM.
/// unsafe { BitUnPack(CGA_8X8_THICK.as_ptr(), 0x0600_0000 as *mut u32, &info) };
/// ```
///
/// I am not a lawyer, but type faces are not protected by copyright in the USA.
/// The copyright status of font faces [varies by country][wp]. If it matters,
/// CGA was first released in 1981.
///
/// [cp437]: https://en.wikipedia.org/wiki/Code_page_437
///
/// [wp]:
/// https://en.wikipedia.org/wiki/Intellectual_property_protection_of_typefaces
pub const CGA_8X8_THICK: [u32; 512] = [
  // Note(Lokathor): I generated this by (1) converting the type face file from
  // an RGB PNG to Indexed Color PNG using GIMP, (2) running `grit
  // CGA8x8thick-indexed.png -gB1` to output an assembly file full of the
  // compressed data `.word` entries, (3) copying all those words into Rust.
  0x00000000, 0x00000000, 0x81A5817E, 0x7E8199BD, 0xFFDBFF7E, 0x7EFFE7C3, 0x7F7F7F36, 0x00081C3E,
  0x7F3E1C08, 0x00081C3E, 0x7F1C3E1C, 0x1C086B7F, 0x3E1C0808, 0x1C083E7F, 0x3C180000, 0x0000183C,
  0xC3E7FFFF, 0xFFFFE7C3, 0x42663C00, 0x003C6642, 0xBD99C3FF, 0xFFC399BD, 0xBEF0E0F0, 0x1E333333,
  0x6666663C, 0x187E183C, 0x0CFCCCFC, 0x070F0E0C, 0xC6FEC6FE, 0x0367E6C6, 0xE73CDB18, 0x18DB3CE7,
  0x7F1F0701, 0x0001071F, 0x7F7C7040, 0x0040707C, 0x187E3C18, 0x183C7E18, 0x66666666, 0x00660066,
  0xDEDBDBFE, 0x00D8D8D8, 0x361CC67C, 0x1E331C36, 0x00000000, 0x007E7E7E, 0x187E3C18, 0xFF183C7E,
  0x187E3C18, 0x00181818, 0x18181818, 0x00183C7E, 0x7F301800, 0x00001830, 0x7F060C00, 0x00000C06,
  0x03030000, 0x00007F03, 0xFF662400, 0x00002466, 0x7E3C1800, 0x0000FFFF, 0x7EFFFF00, 0x0000183C,
  0x00000000, 0x00000000, 0x0C1E1E0C, 0x000C000C, 0x00363636, 0x00000000, 0x367F3636, 0x0036367F,
  0x1E033E0C, 0x000C1F30, 0x18336300, 0x0063660C, 0x6E1C361C, 0x006E333B, 0x00030606, 0x00000000,
  0x06060C18, 0x00180C06, 0x18180C06, 0x00060C18, 0xFF3C6600, 0x0000663C, 0x3F0C0C00, 0x00000C0C,
  0x00000000, 0x060C0C00, 0x3F000000, 0x00000000, 0x00000000, 0x000C0C00, 0x0C183060, 0x00010306,
  0x7B73633E, 0x003E676F, 0x0C0C0E0C, 0x003F0C0C, 0x1C30331E, 0x003F3306, 0x1C30331E, 0x001E3330,
  0x33363C38, 0x0078307F, 0x301F033F, 0x001E3330, 0x1F03061C, 0x001E3333, 0x1830333F, 0x000C0C0C,
  0x1E33331E, 0x001E3333, 0x3E33331E, 0x000E1830, 0x000C0C00, 0x000C0C00, 0x000C0C00, 0x060C0C00,
  0x03060C18, 0x00180C06, 0x003F0000, 0x00003F00, 0x30180C06, 0x00060C18, 0x1830331E, 0x000C000C,
  0x7B7B633E, 0x001E037B, 0x33331E0C, 0x0033333F, 0x3E66663F, 0x003F6666, 0x0303663C, 0x003C6603,
  0x6666361F, 0x001F3666, 0x1E16467F, 0x007F4616, 0x1E16467F, 0x000F0616, 0x0303663C, 0x007C6673,
  0x3F333333, 0x00333333, 0x0C0C0C1E, 0x001E0C0C, 0x30303078, 0x001E3333, 0x1E366667, 0x00676636,
  0x0606060F, 0x007F6646, 0x7F7F7763, 0x0063636B, 0x7B6F6763, 0x00636373, 0x6363361C, 0x001C3663,
  0x3E66663F, 0x000F0606, 0x3333331E, 0x00381E3B, 0x3E66663F, 0x00676636, 0x0C06331E, 0x001E3318,
  0x0C0C2D3F, 0x001E0C0C, 0x33333333, 0x003F3333, 0x33333333, 0x000C1E33, 0x6B636363, 0x0063777F,
  0x1C366363, 0x0063361C, 0x1E333333, 0x001E0C0C, 0x1831637F, 0x007F664C, 0x0606061E, 0x001E0606,
  0x180C0603, 0x00406030, 0x1818181E, 0x001E1818, 0x63361C08, 0x00000000, 0x00000000, 0xFF000000,
  0x00180C0C, 0x00000000, 0x301E0000, 0x006E333E, 0x3E060607, 0x003B6666, 0x331E0000, 0x001E3303,
  0x3E303038, 0x006E3333, 0x331E0000, 0x001E033F, 0x0F06361C, 0x000F0606, 0x336E0000, 0x1F303E33,
  0x6E360607, 0x00676666, 0x0C0E000C, 0x001E0C0C, 0x30300030, 0x1E333330, 0x36660607, 0x0067361E,
  0x0C0C0C0E, 0x001E0C0C, 0x7F330000, 0x00636B7F, 0x331F0000, 0x00333333, 0x331E0000, 0x001E3333,
  0x663B0000, 0x0F063E66, 0x336E0000, 0x78303E33, 0x6E3B0000, 0x000F0666, 0x033E0000, 0x001F301E,
  0x0C3E0C08, 0x00182C0C, 0x33330000, 0x006E3333, 0x33330000, 0x000C1E33, 0x6B630000, 0x00367F7F,
  0x36630000, 0x0063361C, 0x33330000, 0x1F303E33, 0x193F0000, 0x003F260C, 0x070C0C38, 0x00380C0C,
  0x00181818, 0x00181818, 0x380C0C07, 0x00070C0C, 0x00003B6E, 0x00000000, 0x361C0800, 0x007F6363,
  0x3303331E, 0x1E30181E, 0x33003300, 0x007E3333, 0x331E0038, 0x001E033F, 0x603CC37E, 0x00FC667C,
  0x301E0033, 0x007E333E, 0x301E0007, 0x007E333E, 0x301E0C0C, 0x007E333E, 0x031E0000, 0x1C301E03,
  0x663CC37E, 0x003C067E, 0x331E0033, 0x001E033F, 0x331E0007, 0x001E033F, 0x0C0E0033, 0x001E0C0C,
  0x181C633E, 0x003C1818, 0x0C0E0007, 0x001E0C0C, 0x63361C63, 0x0063637F, 0x1E000C0C, 0x00333F33,
  0x063F0038, 0x003F061E, 0x30FE0000, 0x00FE33FE, 0x7F33367C, 0x00733333, 0x1E00331E, 0x001E3333,
  0x1E003300, 0x001E3333, 0x1E000700, 0x001E3333, 0x3300331E, 0x007E3333, 0x33000700, 0x007E3333,
  0x33003300, 0x1F303E33, 0x663C18C3, 0x00183C66, 0x33330033, 0x001E3333, 0x037E1818, 0x18187E03,
  0x0F26361C, 0x003F6706, 0x3F1E3333, 0x0C0C3F0C, 0x5F33331F, 0xE363F363, 0x3C18D870, 0x0E1B1818,
  0x301E0038, 0x007E333E, 0x0C0E001C, 0x001E0C0C, 0x1E003800, 0x001E3333, 0x33003800, 0x007E3333,
  0x1F001F00, 0x00333333, 0x3733003F, 0x00333B3F, 0x7C36363C, 0x00007E00, 0x1C36361C, 0x00003E00,
  0x060C000C, 0x001E3303, 0x3F000000, 0x00000303, 0x3F000000, 0x00003030, 0x7B3363C3, 0xF03366CC,
  0xDB3363C3, 0xC0F3F6EC, 0x18001818, 0x00181818, 0x3366CC00, 0x0000CC66, 0xCC663300, 0x00003366,
  0x11441144, 0x11441144, 0x55AA55AA, 0x55AA55AA, 0x77DBEEDB, 0x77DBEEDB, 0x18181818, 0x18181818,
  0x18181818, 0x1818181F, 0x181F1818, 0x1818181F, 0x6C6C6C6C, 0x6C6C6C6F, 0x00000000, 0x6C6C6C7F,
  0x181F0000, 0x1818181F, 0x606F6C6C, 0x6C6C6C6F, 0x6C6C6C6C, 0x6C6C6C6C, 0x607F0000, 0x6C6C6C6F,
  0x606F6C6C, 0x0000007F, 0x6C6C6C6C, 0x0000007F, 0x181F1818, 0x0000001F, 0x00000000, 0x1818181F,
  0x18181818, 0x000000F8, 0x18181818, 0x000000FF, 0x00000000, 0x181818FF, 0x18181818, 0x181818F8,
  0x00000000, 0x000000FF, 0x18181818, 0x181818FF, 0x18F81818, 0x181818F8, 0x6C6C6C6C, 0x6C6C6CEC,
  0x0CEC6C6C, 0x000000FC, 0x0CFC0000, 0x6C6C6CEC, 0x00EF6C6C, 0x000000FF, 0x00FF0000, 0x6C6C6CEF,
  0x0CEC6C6C, 0x6C6C6CEC, 0x00FF0000, 0x000000FF, 0x00EF6C6C, 0x6C6C6CEF, 0x00FF1818, 0x000000FF,
  0x6C6C6C6C, 0x000000FF, 0x00FF0000, 0x181818FF, 0x00000000, 0x6C6C6CFF, 0x6C6C6C6C, 0x000000FC,
  0x18F81818, 0x000000F8, 0x18F80000, 0x181818F8, 0x00000000, 0x6C6C6CFC, 0x6C6C6C6C, 0x6C6C6CFF,
  0x18FF1818, 0x181818FF, 0x18181818, 0x0000001F, 0x00000000, 0x181818F8, 0xFFFFFFFF, 0xFFFFFFFF,
  0x00000000, 0xFFFFFFFF, 0x0F0F0F0F, 0x0F0F0F0F, 0xF0F0F0F0, 0xF0F0F0F0, 0xFFFFFFFF, 0x00000000,
  0x3B6E0000, 0x006E3B13, 0x1F331E00, 0x03031F33, 0x03333F00, 0x00030303, 0x36367F00, 0x00363636,
  0x0C06333F, 0x003F3306, 0x1B7E0000, 0x000E1B1B, 0x66666600, 0x03063E66, 0x183B6E00, 0x00181818,
  0x331E0C3F, 0x3F0C1E33, 0x7F63361C, 0x001C3663, 0x6363361C, 0x00773636, 0x3E180C38, 0x001E3333,
  0xDB7E0000, 0x00007EDB, 0xDB7E3060, 0x03067EDB, 0x1F03061C, 0x001C0603, 0x3333331E, 0x00333333,
  0x3F003F00, 0x00003F00, 0x0C3F0C0C, 0x003F000C, 0x0C180C06, 0x003F0006, 0x0C060C18, 0x003F0018,
  0x18D8D870, 0x18181818, 0x18181818, 0x0E1B1B18, 0x3F000C0C, 0x000C0C00, 0x003B6E00, 0x00003B6E,
  0x1C36361C, 0x00000000, 0x18000000, 0x00000018, 0x00000000, 0x00000018, 0x303030F0, 0x383C3637,
  0x3636361E, 0x00000036, 0x060C180E, 0x0000001E, 0x3C3C0000, 0x00003C3C, 0x00000000, 0x00000000,
];
