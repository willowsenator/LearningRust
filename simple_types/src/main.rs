use simple_types::print_difference;
use simple_types::print_array;
use simple_types::ding;
use simple_types::on_off;
use simple_types::print_distance;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference( coords.0, coords.1);


    let coords_arr:[f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);


    print_distance(coords);
}


