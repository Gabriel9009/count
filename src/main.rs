use std::io::Read;
fn main(){
     let mut count : i32 = 0;
    let mut file = std::fs::File::open("national.txt").unwrap();
    let  mut content = String::new();
file.read_to_string(&mut content).unwrap();
for _i in content.split("|"){
    count = count + 1;
}
println!("There are {} words in this text",count);
}