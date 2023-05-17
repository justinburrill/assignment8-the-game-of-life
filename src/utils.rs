// use read_input::prelude::*;


pub fn clear_screen()
{
    clearscreen::clear().unwrap();
}



pub fn find_number_of_digits(size:usize) -> u32
{
    let mut amt:u32 = 1;
    while size as i32 > (u32::pow(10,amt) as i32) - 1
    {
        amt += 1;
    }

    amt
}