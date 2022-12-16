use optee_utee::{DataFlag, ObjectStorageConstants, PersistentObject};
use optee_utee::{Error, ErrorKind, Result};

pub fn create_raw_object(uri: &str, data: Vec<u8>) -> Result<()> {
    let mut obj_id = uri.as_bytes().to_vec();

    let obj_data_flag = DataFlag::ACCESS_READ
        | DataFlag::ACCESS_WRITE
        | DataFlag::ACCESS_WRITE_META
        | DataFlag::OVERWRITE;

    let mut init_data: [u8; 0] = [0; 0];

    match PersistentObject::create(
        ObjectStorageConstants::Private,
        &mut obj_id,
        obj_data_flag,
        None,
        &mut init_data,
    ) {
        Err(e) => {
            return Err(e);
        }

        Ok(mut object) => match object.write(&data) {
            Ok(()) => {
                return Ok(());
            }
            Err(e_write) => {
                object.close_and_delete()?;
                std::mem::forget(object);
                return Err(e_write);
            }
        },
    }
}

pub fn read_raw_object(uri: &str, data: &mut Vec<u8>) -> Result<u32> {
    let mut obj_id = uri.as_bytes().to_vec();

    match PersistentObject::open(
        ObjectStorageConstants::Private,
        &mut obj_id,
        DataFlag::ACCESS_READ | DataFlag::SHARE_READ,
    ) {
        Err(e) => return Err(e),

        Ok(object) => {
            let obj_info = object.info()?;
            let obj_size = obj_info.data_size(); 
            data.resize(obj_size, 0);
            let read_bytes = object.read(data).unwrap();

            if read_bytes != obj_info.data_size() as u32 {
                return Err(Error::new(ErrorKind::ExcessData));
            }

            Ok(read_bytes)
        }
    }
}

pub fn delete_object(uri: &str) -> Result<()> {
    let mut obj_id = uri.as_bytes().to_vec();

    match PersistentObject::open(
        ObjectStorageConstants::Private,
        &mut obj_id,
        DataFlag::ACCESS_READ | DataFlag::ACCESS_WRITE_META,
    ) {
        Err(e) => {
            return Err(e);
        }

        Ok(mut object) => {
            object.close_and_delete()?;
            std::mem::forget(object);
            return Ok(());
        }
    }
}