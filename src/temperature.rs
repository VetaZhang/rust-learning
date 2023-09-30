use std::io;

fn main() {
    println!("输入温度类型 => C 表示摄氏，F 表示华氏");

    let mut t = String::new();

    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read t");
        
    let b = t.trim();//.to_string();
    match b {
        "C" => {
            println!("输入摄氏度：");
            let mut c = String::new();

            io::stdin()
                .read_line(&mut c)
                .expect("Failed to read c");
              
            let c: f64 = c.trim().parse().expect("必须输入数字");
            let f = 32.0 + c * 1.8;
            println!("{c} 摄氏度等于 {f} 华氏度");
        }
        "F" => {
            println!("输入华氏度：");
            let mut f = String::new();

            io::stdin()
                .read_line(&mut f)
                .expect("Failed to read f");
              
            let f: f64 = f.trim().parse().expect("必须输入数字");
            let c = (f - 32.0) / 1.8;
            println!("{f} 华氏度等于 {c} 摄氏度");
        }
        &_ => {
            println!("必须输入 C 或者 F");
        }
    }
}