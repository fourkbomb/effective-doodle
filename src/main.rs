extern crate regex;

use regex::Regex;

use std::io::prelude::*;
use std::net::TcpStream;
use std::result::Result;

struct Message<'a> {
    sender: &'a str,
    dest:   &'a str,
    msg:    &'a str,
}

struct Config<'a> {
    nick:   &'a str,
    server: &'a str,
}

struct Bot<'a> {
    cfg: Config<'a>,
    socket: TcpStream,
}


impl<'a> Message<'a> {
    fn reply(&self, bot: Bot, msg: String) {
        let dest = if self.dest == bot.cfg.nick {
            self.sender
        } else {
            self.dest
        };
        println!("Send {} to {}", msg, dest);
    }
}

impl <'a> Bot<'a> {
    fn sendln(&mut self, line: String) {
        write!(self.socket, "{}\r\n", line);
    }
}

fn read_line(sock: &mut TcpStream) -> Result<String, std::io::Error> {
    // dankness
    let mut buffer: [u8; 1] = [0; 1];
    let mut cur : String = "".to_string();
    let mut done = false;
    while !done {
        match sock.read_exact(&mut buffer) {
            Ok(_) => {
                done = (buffer[0] as char) == '\r';
                if done {
                    println!("READ LINE");
                    continue;
                }
                if (buffer[0] as char) == '\n' {
                    continue;
                }
                cur.push(buffer[0] as char);
            },
            Err(e) => {
                println!("RIP");
                return Err(e)
            }
        }
    }
    Ok(cur)
}

fn handle_msg(bot: &mut Bot, line: &mut String) {
    // TODO make this regexp static
    let re = Regex::new(r"^(?::(.+?) )?(.+?) (.+?)(?: :(.+))?$").unwrap();
    //println!("{} matches regex: {}", line, re.is_match(line));
    println!("<= {}", line);
    for cap in re.captures_iter(line) {
        let sender = cap.at(1).unwrap_or("?");
        let cmd = cap.at(2).unwrap_or("?");
        let dest = cap.at(3).unwrap_or("?");
        let text = cap.at(4).unwrap_or("");
        //println!("sender: {}, cmd: {}, dest: {}, text: '{}'", sender, cmd, dest, text);
        if cmd == "PRIVMSG" && dest == bot.cfg.nick {
            bot.sendln(text.to_string());
            println!("=> {}", text);
        }
    }


}

fn main() {
    let cfg = Config { nick: "veryrustybot", server: "chat.au.freenode.net" };
    let mut bot = Bot { cfg: cfg , socket: TcpStream::connect("chat.au.freenode.net:6667").unwrap()};

    // setup connection
    bot.socket.write(format!("NICK {}\r\nUSER bot 8 * :human\r\n", bot.cfg.nick).into_bytes().as_slice());
    bot.socket.flush();
    let mut finished = false;
    while !finished {
        match read_line(&mut bot.socket) {
            Ok(s) => handle_msg(&mut bot, &mut s.clone()),
            Err(_) => {
                println!("Failed to read line");
                finished = true;
            },
        };
    }
//    println!("{}", line);
}
