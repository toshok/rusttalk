use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use om::{OM, OOP};

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
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
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
        load_manchester(file, om, object_space_length as usize, object_table_length as usize)
    } else {
        panic!("unrecognized image type {}", image_type)
    }
}

fn load_manchester(mut file: File, om: &mut OM, object_space_length: usize, object_table_length: usize) {
    file.seek(SeekFrom::Start(512));

    print!("Loading object space... ");
    let mut object_space = vec![0u8; object_space_length];
    file.read_exact(&mut object_space);
    println!("done");

    let tell = file.seek(SeekFrom::Current(0)).unwrap();
    file.seek(SeekFrom::Start(tell & !511)).unwrap();

    print!("Loading object table... ");
    let mut offsets = vec![0i32; object_table_length];

    for o in 0..(object_table_length-1) {
        offsets[o] = file.read_i32::<BigEndian>().unwrap();
    }
    for o in 0..(object_table_length-1) {
        om.set_ot_count(o as OOP, file.read_u8().unwrap());
    }
    for o in 0..(object_table_length-1) {
        om.set_ot_rest(o as OOP, file.read_u8().unwrap());
    }
    for o in 0..(object_table_length-1) {
        om.set_ot_size(o as OOP, file.read_i32::<BigEndian>().unwrap());
    }

    for o in 0..(object_table_length-1) {
        om.set_ot_class(o as OOP, file.read_u32::<BigEndian>().unwrap() as OOP)
    }
    println!("done");

    // load bitmaps

    // link free entries in object table
    om.initialize_free_list();

    // initialize free chunk lists

    println!("snapshot loaded.");
}