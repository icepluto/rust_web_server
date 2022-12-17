# 平平无奇的一个多线程rust web server

对rust语言基础的练习demo


#### 1：首先实现一个返回"hello rust"的web server
    怎样构建的计划
    1: 需要tcp 以及http知识
    2: 在socket监听tcp请求
    3: 分析http的请求
    4: 处理HTTP的请求并且创建http的响应
    5: 通过线程改善server的吞吐量

#### 2：首先先构建一个基础的单线程webserver

需要http以及tcp
http：超文本传输协议，在tcp协议之上，定义请求和响应的内容
tcp： 传输控制协议，也是由请求与响应组成，但是不关注定义内容，是一个底层的协议
*http是通过tcp传输的，所以我们要处理的就是TCP和HTTP的请求和响应的原始字节数据。*

#### 3：先监听TCP的连接

rust 标准库里提供了std::net模块处理这些功能

src/main.rs
```
use std::net::TcpListener;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("connecting...")
    }
}
```
tcplistener的bind方法。我们绑定了一个7878端口。然后我们会接收到从7878端口浏览器发送的http的内容。
然后我们将它。放在一个listener上有一个incoming方法，这个方法它可以一直等待浏览器发来的内容。而且它返回一个迭代器。
然后我们打印一个connecting，也就是说，如果浏览器发送了。数据到7878端口，也就是说它访问了7878端口。我们就在命令行里打印消息


当然，我们需要处理短信发来的HTTP数据的，所以我们在新建一个函数，就叫做handle_collection。

```
use std::{net::{TcpListener, TcpStream}, io::Read};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request:{}",String::from_utf8_lossy(&buffer[..]));
}

```

Handle_connection的参数是可变的。这是因为TCPStrream，它会接收我们。发送来的数据可能会保存多于我们发送来的数据，并保存它们已准在下一次使用数据。
首先，我们在内存里面开辟出一个。缓冲区Buffer。这里给它设置为1024字节。
接着将缓冲区传递给stream.read它会从 TcpStream 中读取字节并放入缓冲区中。
接下来将缓冲区的有吧自己转换为字符串并打印出来。这里使用了，String::from_utf8_lossy(&buffer[..])

然后运行一下。浏览器访问7878端口。