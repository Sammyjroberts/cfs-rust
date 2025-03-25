// src/object_table.rs
use lazy_static::lazy_static;
use std::fs::File;
use std::sync::Mutex;

use crate::error::*;
use crate::types::*;

// Object types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Stream,
    // Other object types would go here
}

// Stream record structure (similar to OS_stream_internal_record_t)
pub struct StreamRecord {
    pub name: String,
    pub in_use: bool,
    pub file: Option<File>,
    // Other fields as needed
}

impl StreamRecord {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            in_use: false,
            file: None,
        }
    }
}

// Object token (similar to OS_object_token_t)
pub struct ObjectToken {
    pub obj_type: ObjectType,
    pub obj_idx: usize,
    // Other fields as needed
}

impl ObjectToken {
    pub fn to_osal_id(&self) -> OsalId {
        // Convert token to OSAL ID using similar logic to C OSAL
        // This is a simplified version
        ((self.obj_type as u32) << 24) | (self.obj_idx as u32)
    }
}

// Object table
pub struct ObjectTable {
    streams: Vec<StreamRecord>,
    // Other tables would go here
}

impl ObjectTable {
    pub fn new(max_streams: usize) -> Self {
        let mut streams = Vec::with_capacity(max_streams);
        for _ in 0..max_streams {
            streams.push(StreamRecord::new());
        }

        Self { streams }
    }

    pub fn allocate_id(
        &mut self,
        obj_type: ObjectType,
        _name: Option<&str>,
    ) -> Result<ObjectToken, i32> {
        // Find a free slot
        for (idx, stream) in self.streams.iter().enumerate() {
            if !stream.in_use {
                return Ok(ObjectToken {
                    obj_type,
                    obj_idx: idx,
                });
            }
        }

        // No free slots
        Err(OS_ERR_NO_FREE_IDS)
    }

    pub fn free_id(&mut self, token: &ObjectToken) {
        if token.obj_type == ObjectType::Stream && token.obj_idx < self.streams.len() {
            self.streams[token.obj_idx].in_use = false;
            self.streams[token.obj_idx].name.clear();
            self.streams[token.obj_idx].file = None;
        }
    }

    pub fn get_stream_mut(&mut self, token: &ObjectToken) -> &mut StreamRecord {
        &mut self.streams[token.obj_idx]
    }

    pub fn id_to_token(&self, id: OsalId) -> Result<ObjectToken, i32> {
        // Extract type and index from ID
        let obj_type_num = (id >> 24) & 0xFF;
        let obj_idx = (id & 0xFFFFFF) as usize;

        // Convert to ObjectType
        let obj_type = match obj_type_num {
            0 => ObjectType::Stream,
            _ => return Err(OS_ERR_INVALID_ID),
        };

        // Validate index
        match obj_type {
            ObjectType::Stream => {
                if obj_idx >= self.streams.len() || !self.streams[obj_idx].in_use {
                    return Err(OS_ERR_INVALID_ID);
                }
            }
        }

        Ok(ObjectToken { obj_type, obj_idx })
    }
}

// Global object table
lazy_static! {
    pub static ref OBJECT_TABLE: Mutex<ObjectTable> = Mutex::new(ObjectTable::new(32)); // 32 is arbitrary
}
