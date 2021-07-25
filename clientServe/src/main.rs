use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();//监听
    for stream in listener.incoming(){//循环读取流
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];//读取缓存512字节
    stream.read(&mut buffer).unwrap();//读取
    println!("request:{}",String::from_utf8_lossy(&buffer[..]) );
    let mut file = File::open("hello.html").expect("cant open html");//打开html文件
    let mut contents = String::new();//内容变量
    file.read_to_string(&mut contents).unwrap();//读取字符串到content变量
    let response = format!("HTTP/1.1 200 ok\r\n\r\n{}",contents);//格式化字符串
    stream.write(response.as_bytes()).unwrap();//向流中写入html文件
    stream.flush().unwrap();//刷新流
}