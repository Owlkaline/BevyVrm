use std::borrow::BorrowMut;
use std::hash::Hash;
use std::io::{self, BufReader, Read};
use std::net::UdpSocket;

use std::collections::HashMap;

pub const EYE_TRACKING_ADDR: &str = "/VMC/Ext/Set/Eye";
pub const CAMERA_ADDR: &str = "/VMC/Ext/Cam";
pub const ROOT_ADDR: &str = "Ext/Root/Pos";
pub const BONE_TRACKING_ADDR: &str = "/VMC/Ext/Bone/Pos";
pub const BLEND_TRACKING_ADDR: &str = "/VMC/Ext/Blend/Val";
pub const BLEND_APPLY_ADDR: &str = "/VMC/Ext/Blend/Apply";

#[cfg(feature = "bevy")]
use bevy::prelude::*;
#[cfg(feature = "bevy")]
use bevy_vrm::*;

#[cfg(feature = "bevy")]
#[derive(Resource)]
pub struct VMCListener {
    port: u32,
    pub blend_shape_translations: HashMap<String, String>,
    listener: Option<UdpSocket>,
}

#[cfg(not(feature = "bevy"))]
pub struct VMCListener {
    port: u32,
    pub blend_shape_translations: HashMap<String, String>,
    listener: Option<UdpSocket>,
}

impl VMCListener {
    pub fn new() -> VMCListener {
        let mut blend_shape_translations = HashMap::new();
        blend_shape_translations.insert("Joy".to_owned(), "happy".to_owned());
        blend_shape_translations.insert("Sorrow".to_owned(), "sad".to_owned());
        blend_shape_translations.insert("Fun".to_owned(), "relaxed".to_owned());
        blend_shape_translations.insert("A".to_owned(), "aa".to_owned());
        blend_shape_translations.insert("I".to_owned(), "ih".to_owned());
        blend_shape_translations.insert("U".to_owned(), "ou".to_owned());
        blend_shape_translations.insert("E".to_owned(), "ee".to_owned());
        blend_shape_translations.insert("O".to_owned(), "oh".to_owned());
        blend_shape_translations.insert("Neutral".to_owned(), "neutral".to_owned());
        blend_shape_translations.insert("Blink".to_owned(), "blink".to_owned());
        blend_shape_translations.insert("Blink_L".to_owned(), "blinkLeft".to_owned());
        blend_shape_translations.insert("Blink_R".to_owned(), "blinkRight".to_owned());
        blend_shape_translations.insert("LookUp".to_owned(), "lookUp".to_owned());
        blend_shape_translations.insert("LookDown".to_owned(), "lookDown".to_owned());
        blend_shape_translations.insert("LookLeft".to_owned(), "lookLeft".to_owned());
        blend_shape_translations.insert("LookRight".to_owned(), "lookRight".to_owned());
        blend_shape_translations.insert("Angry".to_owned(), "angry".to_owned());

        VMCListener {
            port: 3333,
            blend_shape_translations,
            listener: None, // UdpSocket::bind("127.0.0.1:3333").unwrap(),
        }
    }

    pub fn ready(&mut self) {
        let listener = UdpSocket::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        let _ = listener.set_nonblocking(true);
        self.listener = Some(listener);
    }

    pub fn process(&mut self) -> Vec<Message> {
        let mut messages = Vec::new();
        if let Some(listener) = &mut self.listener {
            let mut buf = [0; 10000];

            match listener.recv_from(&mut buf) {
                Ok(bytes) => {
                    if bytes.0 > buf.len() - 1 {
                        println!("Data from tracking excessed buffer size!");
                    }
                    let mut position = 0;
                    messages = parse(&buf, &mut position, bytes.0);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                _ => {}
            }
        }

        return messages;
    }
}

#[derive(Default, Debug)]
pub struct Message {
    pub address: String,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum Value {
    Float(f32),
    Int(i32),
    Bool(bool),
    Blob(String),
    String(String),
}

impl Message {
    pub fn set_address<S: Into<String>>(&mut self, location: S) {
        self.address = location.into();
    }

    pub fn get_float(&self, idx: usize) -> Option<f32> {
        match self.values[idx] {
            Value::Float(f) => return Some(f),
            _ => None,
        }
    }

    pub fn get_int(&self, idx: usize) -> Option<i32> {
        match self.values[idx] {
            Value::Int(f) => return Some(f),
            _ => None,
        }
    }

    pub fn get_bool(&self, idx: usize) -> Option<bool> {
        match self.values[idx] {
            Value::Bool(f) => return Some(f),
            _ => None,
        }
    }

    pub fn get_blob(&self, idx: usize) -> Option<String> {
        match &self.values[idx] {
            Value::Blob(f) => return Some(f.to_owned()),
            _ => None,
        }
    }

    pub fn get_string(&self, idx: usize) -> Option<String> {
        match &self.values[idx] {
            Value::String(f) => return Some(f.to_owned()),
            _ => None,
        }
    }
}

pub const BUNDLE: &str = "#bundle";

pub fn parse(buf: &[u8], pos: &mut usize, end_pos: usize) -> Vec<Message> {
    //  println!("{:?}", String::from_utf8_lossy(&buf));

    let mut messages = Vec::new();

    //println!("{:?}", String::from_utf8_lossy(&buf[0..64]));

    let goto_zero: Box<dyn Fn(&u8) -> bool> = Box::new(|a| *a == 0);
    let goto_the_d: Box<dyn Fn(&u8) -> bool> = Box::new(|a| *a as char == 'D');
    let skip_zero: Box<dyn Fn(&u8) -> bool> = Box::new(|a| *a != 0);

    let mut data = buf.clone().to_vec();
    let mut buffer_separated = buf[*pos..].splitn(2, &goto_zero);
    let identifier = buffer_separated.next().unwrap();
    data = buffer_separated.next().unwrap().to_vec();

    while data[0] == 0 {
        data.remove(0);
    }
    let mut new_u32 = Vec::new();
    for i in 0..4 {
        new_u32.push(data.remove(0));
    }
    let value = u32::from_le_bytes(new_u32[..].try_into().unwrap());
    //  println!("Value: {}", value);
    // Skip D, might indicate directory
    'parser: while data.len() > 0 {
        // let mut buffer_separated = data.splitn(2, &goto_the_d);
        //let _ = buffer_separated.next();
        //data = buffer_separated.next().unwrap().to_vec();
        for i in 0..3 {
            data.remove(0);
        }

        let fake_data = data.clone();
        let mut buffer_separated = fake_data.splitn(2, &goto_zero);
        let location = String::from_utf8_lossy(buffer_separated.next().unwrap());
        data = buffer_separated.next().unwrap().to_vec();
        if location == "/VMC/Ext/Blend/Apply" {
            break;
        }
        //println!("New identifer: {}", location);
        while data[0] == 0 || data[0] as char == ',' {
            data.remove(0);
            if data.len() == 0 {
                break 'parser;
            }
        }
        let fake_data = data.clone();
        let mut buffer_separated = fake_data.splitn(2, &goto_zero);
        let types = buffer_separated
            .next()
            .unwrap()
            .iter()
            .map(|a| *a as char)
            .collect::<Vec<_>>();
        data = buffer_separated.next().unwrap().to_vec();
        //println!("types: {:?}", types);
        // small buffer alignment
        while data[0] == 0 {
            data.remove(0);
        }

        messages.push(fill_message(location.to_owned(), types, &mut data));
    }

    messages
}

fn fill_message<S: Into<String>>(location: S, types: Vec<char>, data: &mut Vec<u8>) -> Message {
    let goto_zero: Box<dyn Fn(&u8) -> bool> = Box::new(|a| *a == 0);
    let mut message = Message::default();
    message.set_address(location);

    for value in types.iter() {
        match value {
            'i' => {
                let mut new_i32 = Vec::new();
                for i in 0..4 {
                    new_i32.push(data.remove(0));
                }
                let value = i32::from_be_bytes(new_i32.try_into().unwrap());
                message.values.push(Value::Int(value));
            }
            'f' => {
                let mut new_float = Vec::new();
                for i in 0..4 {
                    new_float.push(data.remove(0));
                }
                let value = f32::from_be_bytes(new_float.clone().try_into().unwrap());
                message.values.push(Value::Float(value));
            }
            's' => {
                while data[0] == 0 {
                    data.remove(0);
                }

                let fake_data = data.clone();
                let mut buffer_separated = fake_data.splitn(2, &goto_zero);
                let string = String::from_utf8_lossy(buffer_separated.next().unwrap());
                *data = buffer_separated.next().unwrap().to_vec();
                let size = string.len();
                let aligned_size = (size + 4) & !0x3;
                for _ in 0..(aligned_size - size - 1) {
                    data.remove(0);
                }
                //println!("Size: {} Aligned Size: {}", size, aligned_size);
                //println!("String: {}", string);
                message.values.push(Value::String(string.into()));
            }
            'b' => {
                //println!("This is blob");
                //let (bytes, new_data) = data.split_at(4);
                //data = new_data;
                //*pos += 4;
                //let value = usize::from_be_bytes(bytes.try_into().unwrap());

                //let (bytes, new_data) = data.split_at(value);
                //data = new_data;
                //*pos += bytes.len();
                //message
                //  .values
                //  .push(Value::Blob(String::from_utf8_lossy(&bytes).into()));
            }
            'T' => {
                //*pos += 1;
                data.remove(0);
                message.values.push(Value::Bool(true));
            }
            'F' => {
                //*pos += 1;
                data.remove(0);
                message.values.push(Value::Bool(false));
            }
            _ => {
                //println!("Unknown type");
            }
        }
    }

    message
}
