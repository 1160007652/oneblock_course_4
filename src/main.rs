mod question;

fn main() {
    println!("\n[第一题]");
    question::one::enum_traffic_light("Green".to_string());

    println!("\n[第二题]");
    question::two::sum_num(&[1, 2, 3]);

    println!("\n[第三题]");
    question::three::calculate_graphics_area();

    println!("\n");
}
