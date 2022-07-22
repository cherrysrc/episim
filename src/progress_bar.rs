
pub fn print_progress(progress: f32) {
    print!("[");
    for _ in 0..((progress / 10.0) as u32) {
        print!("=");
    }
    
    for _ in ((progress / 10.0) as u32)..10 {
        print!(" ");
    }
    print!("] ");
    
    print!("{:.2} %", progress);
}