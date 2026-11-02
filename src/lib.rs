use std::io::{self, Read, Seek};

use eso_common::*;
use serde::Serialize;

pub mod enums {
    pub mod bind_type;
    pub mod item_type;
}

pub const DATA1_SIZE: usize = 17;
pub const DATA2_SIZE: usize = 17;
pub const DATA3_SIZE: usize = 3;
pub const DATA4_SIZE: usize = 3;
pub const DATA5B_SIZE: usize = 7;
pub const MID_A_SIZE: usize = 35;
pub const TAIL_D_FLAGS_SIZE: usize = 4;
pub const ENTRY_END_BIAS: u64 = 32;

fn is_false(v: &bool) -> bool {
    !*v
}
fn is_default_scale(v: &f32) -> bool {
    *v == 1.0
}
fn list_is_empty(v: &Vec<u32>) -> bool {
    v.is_empty()
}

#[derive(Debug, Serialize, PartialEq, Clone, Copy, Default)]
pub struct Header {
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub magic: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub version: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub total_entries: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub idk2: u32,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Entry {
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub start_offset: u64,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub end_offset: u64,

    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub magic: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub index: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub size1: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub size2: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub blob_len: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub item_id: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub blob_offset: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub size3: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub item_id2: u32,

    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub data1: [u8; DATA1_SIZE],
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub date_time: u32,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub data2: [u8; DATA2_SIZE],
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub style: u8,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub data3: [u8; DATA3_SIZE],
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub default_enchant_id: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub r#trait: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub bind_type: u8,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub data4: [u8; DATA4_SIZE],
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub set_id: u32,

    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_88: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub tooltip_lang_index: u32,
    #[serde(skip_serializing_if = "is_default_scale")]
    pub scale: f32,
    #[serde(skip_serializing_if = "is_zero_u16")]
    pub max_stack: u16,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub data5b: [u8; DATA5B_SIZE],
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_109: u8,

    #[serde(skip_serializing_if = "is_false")]
    pub has_link_block: bool,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub link_pad: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub ref_id: u32,
    #[serde(skip_serializing_if = "list_is_empty")]
    pub link_list_a: Vec<u32>,
    #[serde(skip_serializing_if = "list_is_empty")]
    pub link_list_b: Vec<u32>,

    #[serde(skip_serializing_if = "list_is_empty")]
    pub b_list: Vec<u32>,

    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub mid_a: [u8; MID_A_SIZE],
    #[serde(skip_serializing_if = "list_is_empty")]
    pub mid_list: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_148: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_149: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub icon_file_index: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_157: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub name_id: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_165: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_166: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_170: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_174: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_175: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_179: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_183: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_187: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_191: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_192: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_193: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub linked_item_id: u32,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub mid_b_pad: [u8; 2],

    #[serde(skip_serializing_if = "is_zero_u16")]
    pub item_type_id: u16,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub link_kind: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub link_id: u32,

    #[serde(skip_serializing_if = "is_zero_u8")]
    pub unk_207: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_208: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_212: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub tail_item_ref: u32,
    #[serde(skip_serializing_if = "list_is_empty")]
    pub tail_list_a: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub model_lut_index: u32,
    #[serde(skip_serializing_if = "list_is_empty")]
    pub tail_list_b: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_226: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_230: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_234: u32,
    #[serde(skip_serializing_if = "list_is_empty")]
    pub tail_list_c: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_239: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_243: u32,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")]
    pub tail_d_flags: [u8; TAIL_D_FLAGS_SIZE],
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub unk_251: u32,

    #[serde(skip_serializing_if = "is_empty_vec")]
    pub trailing: Vec<u8>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            start_offset: 0, end_offset: 0, magic: 0, index: 0, size1: 0, size2: 0,
            blob_len: 0, item_id: 0, blob_offset: 0, size3: 0, item_id2: 0,
            data1: [0; DATA1_SIZE], date_time: 0, data2: [0; DATA2_SIZE], style: 0,
            data3: [0; DATA3_SIZE], default_enchant_id: 0, r#trait: 0, bind_type: 0,
            data4: [0; DATA4_SIZE], set_id: 0, unk_88: 0, tooltip_lang_index: 0, scale: 1.0,
            max_stack: 0, data5b: [0; DATA5B_SIZE], unk_109: 0, has_link_block: false,
            link_pad: 0, ref_id: 0, link_list_a: Vec::new(), link_list_b: Vec::new(),
            b_list: Vec::new(), mid_a: [0; MID_A_SIZE], mid_list: Vec::new(),
            unk_148: 0, unk_149: 0, icon_file_index: 0, unk_157: 0, name_id: 0, unk_165: 0,
            unk_166: 0, unk_170: 0, unk_174: 0, unk_175: 0, unk_179: 0, unk_183: 0,
            unk_187: 0, unk_191: 0, unk_192: 0, unk_193: 0, linked_item_id: 0,
            mid_b_pad: [0; 2], item_type_id: 0, link_kind: 0, link_id: 0,
            unk_207: 0, unk_208: 0, unk_212: 0, tail_item_ref: 0, tail_list_a: Vec::new(),
            model_lut_index: 0, tail_list_b: Vec::new(), unk_226: 0, unk_230: 0, unk_234: 0,
            tail_list_c: Vec::new(), unk_239: 0, unk_243: 0,
            tail_d_flags: [0; TAIL_D_FLAGS_SIZE], unk_251: 0, trailing: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ItemFileData {
    pub size_of_file: u64,
    pub header: Header,
    pub entries: Vec<Entry>,
}

pub fn read_header<R: Read + Seek>(r: &mut ByteReader<R>) -> io::Result<Header> {
    Ok(Header {
        magic: r.read_dword_be()?,
        version: r.read_dword_be()?,
        total_entries: r.read_dword_be()?,
        idk2: r.read_dword_be()?,
    })
}

fn fill<const N: usize, R: Read + Seek>(r: &mut ByteReader<R>, out: &mut [u8; N]) -> io::Result<()> {
    out.copy_from_slice(&r.read_bytes(N)?);
    Ok(())
}

fn read_u32_list<R: Read + Seek>(r: &mut ByteReader<R>) -> io::Result<Vec<u32>> {
    let count = r.read_byte()? as usize;
    let mut v = Vec::with_capacity(count);
    for _ in 0..count {
        v.push(r.read_dword_be()?);
    }
    Ok(v)
}

pub fn read_entry<R: Read + Seek>(r: &mut ByteReader<R>, expected_index: u32) -> io::Result<Entry> {
    let mut e = Entry::default();

    e.start_offset = r.tell()?;
    e.magic = r.read_dword_be()?;
    e.index = r.read_dword_be()?;
    e.size1 = r.read_dword_be()?;
    e.size2 = r.read_dword_be()?;
    e.blob_len = r.read_dword_be()?;
    e.item_id = r.read_dword_be()?;
    e.blob_offset = r.read_dword_be()?;
    e.size3 = r.read_dword_be()?;
    e.item_id2 = r.read_dword_be()?;

    if expected_index != 0 && e.index != expected_index {
        eprintln!("Entry Index Mismatch at 0x{:X}: {} != {}", e.start_offset, e.index, expected_index);
    }
    if e.item_id != e.item_id2 {
        eprintln!("item_id / item_id2 Mismatch at 0x{:X}: {} != {}", e.start_offset, e.item_id, e.item_id2);
    }

    fill(r, &mut e.data1)?;
    e.date_time = r.read_dword_be()?;
    fill(r, &mut e.data2)?;
    e.style = r.read_byte()?;
    fill(r, &mut e.data3)?;
    e.default_enchant_id = r.read_byte()?;
    e.r#trait = r.read_byte()?;
    e.bind_type = r.read_byte()?;
    fill(r, &mut e.data4)?;
    e.set_id = r.read_dword_be()?;
    e.unk_88 = r.read_dword_be()?;
    e.tooltip_lang_index = r.read_dword_be()?;

    let f = r.read_bytes(4)?;
    e.scale = f32::from_le_bytes([f[0], f[1], f[2], f[3]]);
    e.max_stack = r.read_word_be()?;
    fill(r, &mut e.data5b)?;
    e.unk_109 = r.read_byte()?;

    e.has_link_block = r.read_byte()? != 0;
    if e.has_link_block {
        e.link_pad = r.read_dword_be()?;
        e.ref_id = r.read_dword_be()?;
        e.link_list_a = read_u32_list(r)?;
        e.link_list_b = read_u32_list(r)?;
    }
    e.b_list = read_u32_list(r)?;

    fill(r, &mut e.mid_a)?;
    e.mid_list = read_u32_list(r)?;
    e.unk_148 = r.read_byte()?;
    e.unk_149 = r.read_dword_be()?;
    e.icon_file_index = r.read_dword_be()?;
    e.unk_157 = r.read_dword_be()?;
    e.name_id = r.read_dword_be()?;
    e.unk_165 = r.read_byte()?;
    e.unk_166 = r.read_dword_be()?;
    e.unk_170 = r.read_dword_be()?;
    e.unk_174 = r.read_byte()?;
    e.unk_175 = r.read_dword_be()?;
    e.unk_179 = r.read_dword_be()?;
    e.unk_183 = r.read_dword_be()?;
    e.unk_187 = r.read_dword_be()?;
    e.unk_191 = r.read_byte()?;
    e.unk_192 = r.read_byte()?;
    e.unk_193 = r.read_byte()?;
    e.linked_item_id = r.read_dword_be()?;
    fill(r, &mut e.mid_b_pad)?;

    e.item_type_id = r.read_word_be()?;
    e.link_kind = r.read_byte()?;
    e.link_id = r.read_dword_be()?;

    e.unk_207 = r.read_byte()?;
    e.unk_208 = r.read_dword_be()?;
    e.unk_212 = r.read_dword_be()?;
    e.tail_item_ref = r.read_dword_be()?;
    e.tail_list_a = read_u32_list(r)?;
    e.model_lut_index = r.read_dword_be()?;
    e.tail_list_b = read_u32_list(r)?;
    e.unk_226 = r.read_dword_be()?;
    e.unk_230 = r.read_dword_be()?;
    e.unk_234 = r.read_dword_be()?;
    e.tail_list_c = read_u32_list(r)?;
    e.unk_239 = r.read_dword_be()?;
    e.unk_243 = r.read_dword_be()?;
    fill(r, &mut e.tail_d_flags)?;
    e.unk_251 = r.read_dword_be()?;

    let expected_end = e.start_offset + e.size1 as u64 + ENTRY_END_BIAS;
    let pos = r.tell()?;
    if pos < expected_end {
        eprintln!(
            "Warning: entry {} at 0x{:X}: {} unparsed bytes before entry end",
            e.index, e.start_offset, expected_end - pos
        );
        e.trailing = r.read_bytes((expected_end - pos) as usize)?;
    } else if pos > expected_end {
        eprintln!(
            "Warning: entry {} at 0x{:X}: parsed {} bytes past entry end",
            e.index, e.start_offset, pos - expected_end
        );
    }

    e.end_offset = r.tell()?;
    Ok(e)
}