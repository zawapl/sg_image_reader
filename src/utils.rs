use crate::{Result, SgImageError};
use std::io::Read;
use std::str;

pub trait ReadHelper {
    fn read_u8(&mut self) -> Result<u8>;

    fn read_u16_le(&mut self) -> Result<u16>;

    fn read_u32_le(&mut self) -> Result<u32>;

    fn read_i32_le(&mut self) -> Result<i32>;

    fn read_string(&mut self, length: usize) -> Result<String>;

    fn read_bytes<const LENGTH: usize>(&mut self) -> Result<[u8; LENGTH]>
    where
        [u8; LENGTH]: Default;
}

impl<R: Read> ReadHelper for R {
    fn read_u8(&mut self) -> Result<u8> {
        let mut tmp = [0; 1];
        self.read_exact(&mut tmp)?;
        Ok(tmp[0])
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        let mut tmp = [0; 2];
        self.read_exact(&mut tmp)?;
        Ok(u16::from_le_bytes(tmp))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let mut tmp = [0; 4];
        self.read_exact(&mut tmp)?;
        Ok(u32::from_le_bytes(tmp))
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        let mut tmp = [0; 4];
        self.read_exact(&mut tmp)?;
        Ok(i32::from_le_bytes(tmp))
    }

    fn read_string(&mut self, max_length: usize) -> Result<String> {
        let mut tmp = vec![0; max_length];

        self.read_exact(&mut tmp)?;

        return match str::from_utf8(&tmp) {
            Ok(str) => Ok(String::from(str.split(char::from(0)).nth(0).unwrap())),
            Err(err) => Err(SgImageError::Utf8Error(err)),
        };
    }

    fn read_bytes<const LENGTH: usize>(&mut self) -> Result<[u8; LENGTH]>
    where
        [u8; LENGTH]: Default,
    {
        let mut result: [u8; LENGTH] = Default::default();
        self.read_exact(&mut result)?;
        Ok(result)
    }
}
