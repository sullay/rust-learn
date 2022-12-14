use std::fs;

fn main() {
    let url: &str = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("请求url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("html转markdown");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("保存到{}", output);
}
