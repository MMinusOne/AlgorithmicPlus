use core::slice;
use memmap2::{Mmap, MmapOptions};
use std::error::Error;
use std::{fs::File, mem, path::Path};

pub struct MmapManager<T: 'static> {
    _mmap: Mmap,
    data: &'static [T],
}

impl<T: 'static> MmapManager<T> {
    pub fn data(&self) -> &[T] {
        return self.data;
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        return self.data.iter();
    }

    pub fn index(&self, i: usize) -> &T { 
        return &self.data[i]
    }
}

pub fn load_mmap<T: 'static>(path: impl AsRef<Path>) -> Result<MmapManager<T>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let record_size = mem::size_of::<T>();
    let records_count = mmap.len() / record_size;

    if mmap.len() % record_size != 0 {
        return Err("File size not a multiple of record size".into());
    }

    let data = unsafe { slice::from_raw_parts(mmap.as_ptr() as *const T, records_count) };

    let data: &'static [T] = unsafe { std::mem::transmute(data) };

    Ok(MmapManager::<T> { _mmap: mmap, data })
}
