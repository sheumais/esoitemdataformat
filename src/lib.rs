use eso_common::{
    always_skip_serialization, is_empty_array, is_empty_vec, is_zero_u32, is_zero_u8,
    serialize_array,
};
use serde::Serialize;

pub const DATA1_SIZE: usize = 17;
pub const DATA2_SIZE: usize = 17;
pub const DATA3_SIZE: usize = 7;
pub const DATA4_SIZE: usize = 3;
pub const ENTRY_FIXED_SIZE: usize = 92;
pub const DATA5_SIZE_OFFSET: u32 = 60;

#[derive(Debug, Serialize, PartialEq, Clone, Copy, Default)]
pub struct Header {
    #[serde(skip_serializing_if = "always_skip_serialization")]
    pub magic: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub idk: u32,
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
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub size2: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub idk: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub item_id: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub idk3: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub size3: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub item_id2: u32,

    #[serde(
        serialize_with = "serialize_array",
        skip_serializing_if = "is_empty_array"
    )]
    pub data1: [u8; DATA1_SIZE],

    #[serde(skip_serializing_if = "is_zero_u32")]
    pub date_time: u32,

    #[serde(
        serialize_with = "serialize_array",
        skip_serializing_if = "is_empty_array"
    )]
    pub data2: [u8; DATA2_SIZE],

    #[serde(skip_serializing_if = "is_zero_u8")]
    pub style: u8,

    #[serde(
        serialize_with = "serialize_array",
        skip_serializing_if = "is_empty_array"
    )]
    pub data3: [u8; DATA3_SIZE],

    #[serde(skip_serializing_if = "is_zero_u8")]
    pub default_enchant_id: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub r#trait: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]
    pub bind_type: u8,

    #[serde(
        serialize_with = "serialize_array",
        skip_serializing_if = "is_empty_array"
    )]
    pub data4: [u8; DATA4_SIZE],

    #[serde(skip_serializing_if = "is_zero_u32")]
    pub set_id: u32,

    #[serde(skip_serializing_if = "is_empty_vec")]
    pub data5: Vec<u8>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            start_offset: 0,
            end_offset: 0,
            magic: 0,
            index: 0,
            size1: 0,
            size2: 0,
            idk: 0,
            item_id: 0,
            idk3: 0,
            size3: 0,
            item_id2: 0,
            data1: [0u8; DATA1_SIZE],
            date_time: 0,
            data2: [0u8; DATA2_SIZE],
            style: 0,
            data3: [0u8; DATA3_SIZE],
            default_enchant_id: 0,
            r#trait: 0,
            bind_type: 0,
            data4: [0u8; DATA4_SIZE],
            set_id: 0,
            data5: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ItemFileData {
    pub size_of_file: u64,
    pub header: Header,
    pub entries: Vec<Entry>,
}
