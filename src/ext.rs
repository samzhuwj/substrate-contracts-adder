use alloc::vec::Vec;

mod cabi {
    extern "C" {
        pub fn ext_set_storage(key_ptr: u32, value_non_null: u32, value_ptr: u32, value_len: u32);
        pub fn ext_get_storage(key_ptr: u32) -> u32;
        pub fn ext_input_size() -> u32;
        pub fn ext_input_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_scratch_size() -> u32;
        pub fn ext_scratch_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_return(data_ptr: u32, data_len: u32) -> !;
    }
}

pub struct Key(pub [u8; 32]);

pub fn set_storage(key: &Key, value: Option<&[u8]>) {
    unsafe {
        let mut value_ptr = 0;
        let mut value_len = 0;
        let value_non_null = if let Some(v) = value {
            value_ptr = v.as_ptr() as u32;
            value_len = v.len() as u32;
            1
        } else {
            0
        };

        cabi::ext_set_storage(key.0.as_ptr() as u32, value_non_null, value_ptr, value_len);
    }
}

pub fn get_storage(key: &Key) -> Option<Vec<u8>> {
    const ERR_OK: u32 = 0;
    unsafe {
        let result = cabi::ext_get_storage(key.0.as_ptr() as u32);
        if result == ERR_OK {
            let size = cabi::ext_scratch_size();
            let mut value = Vec::new();
            if size > 0 {
                value.resize(size as usize, 0);
                cabi::ext_scratch_copy(value.as_mut_ptr() as u32, 0, size);
            }
            Some(value)
        } else {
            None
        }
    }
}

pub fn input() -> Vec<u8> {
    unsafe {
        let size = cabi::ext_input_size();
        if size == 0 {
            return Vec::new();
        }

        let mut value = Vec::new();
        if size > 0 {
            value.resize(size as usize, 0);
            cabi::ext_input_copy(value.as_mut_ptr() as u32, 0, size);
        }
        value
    }
}

pub fn return_(data: &[u8]) -> ! {
    unsafe {
        cabi::ext_return(data.as_ptr() as u32, data.len() as u32);
    }
}
