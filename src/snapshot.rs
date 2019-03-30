use std::error::Error;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::mem;
use std::path::Path;

use om::{OM, OM_SIZE, OOP};

extern crate byteorder;
use snapshot::byteorder::{BigEndian, ReadBytesExt};

static MANCHESTER2: i16 = 43;

pub fn load(filename: &str, om: &mut OM) {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    println!("opened image file");

    let object_space_length = file.read_i32::<BigEndian>().unwrap();
    let object_table_length = file.read_i32::<BigEndian>().unwrap();
    let image_type = file.read_i16::<BigEndian>().unwrap();

    println!("object space len = {}", object_space_length);
    println!("object table len = {}", object_table_length);
    println!("image type = {}", image_type);

    if image_type == MANCHESTER2 {
        load_manchester(
            file,
            om,
            object_space_length as usize,
            object_table_length as usize,
        )
    } else {
        panic!("unrecognized image type {}", image_type)
    }
}

fn load_manchester(
    mut file: File,
    om: &mut OM,
    object_space_length: usize,
    object_table_length: usize,
) {
    file.seek(SeekFrom::Start(512));

    print!("Loading object space... ");

    let mut object_space = vec![0u32; OM_SIZE];
    for i in 0..object_space_length {
        object_space[i] = file.read_u32::<BigEndian>().unwrap();
    }
    println!("done");

    let mut tell = file.seek(SeekFrom::Current(0)).unwrap();
    file.seek(SeekFrom::Start((tell + 511) & !511)).unwrap();
    tell = file.seek(SeekFrom::Current(0)).unwrap();

    println!("Loading object table from offset {}... ", tell);
    let mut offsets = vec![0i32; object_table_length];

    for o in 0..object_table_length {
        offsets[o] = file.read_i32::<BigEndian>().unwrap();
    }
    for o in 0..object_table_length {
        om.set_count(o as OOP, file.read_u8().unwrap());
    }
    for o in 0..object_table_length {
        om.set_rest(o as OOP, file.read_u8().unwrap());
    }
    for o in 0..object_table_length {
        om.set_size(o as OOP, file.read_i32::<BigEndian>().unwrap() as usize);
    }

    om.initialize_object_space(&mut object_space, offsets);
    mem::forget(object_space);

    for o in 0..object_table_length {
        om.set_class(o as OOP, file.read_u32::<BigEndian>().unwrap() as OOP)
    }
    println!("done");

    // load bitmaps

    // link free entries in object table
    om.initialize_free_list();

    // initialize free chunk lists
    om.initialize_free_chunks(object_space_length);

    println!("snapshot loaded.");
}
