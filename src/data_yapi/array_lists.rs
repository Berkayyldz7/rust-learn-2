pub fn make_array_list()->ArrayList<'static>{
//
    return ArrayList{
        items: &MY_ARR,
        capacity: 20,
        length : MY_ARR.len()
    };
}

#[derive(Debug)]
pub struct ArrayList <'a>{
    items : &'a[i16],
    capacity : usize,
    length : usize
}


static MY_ARR:[i16; 2] = [210,1510];