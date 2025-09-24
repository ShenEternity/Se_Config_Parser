use std::{fs::File, io::{self, BufRead, BufReader}};

pub fn read_all_lines(file:&str) -> io::Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

fn get_config_qujian(config:&Result<Vec<String>,io::Error>, path:&str) -> (usize, usize){
    let mut  config_qujian:(usize, usize) = (0, 0);
    let path_vec:Vec<&str> = path.split(".").collect();
    let mut start: usize = 0;
    let mut started = false;
    match config {
        Ok(lines) => {
            for (i, line) in lines.iter().enumerate() {
                if line.contains("[") && !started {
                    let a = line.find("[");
                    let b = line.find("]");
                    if let (Some(start), Some(end)) = (a, b) {
                        if &&line[start+1..end] == path_vec.last().unwrap() {
                            started = true;
                        }
                    }
                }
                if started {
                    if line.contains("{") {
                        start = line.find("{").unwrap();
                        config_qujian.0 = i + 1;
                        started = false;
                    }
                   
                }
                if line.contains("}") && !started{
                    if start < line.len() && &line[start..start+1] == "}" {
                            config_qujian.1 = i;
                    }
                }
            }
        }
        Err(err) => {
            println!("{}",err);
        }
    }
    config_qujian
} 

fn get_config_lines(config:&Result<Vec<String>,io::Error>, qujian:(usize,usize)) -> Vec<String> {
    let mut line_vec:Vec<String> = Vec::new();
    let mut start:usize = 0;
    let mut started = true;
    let mut ened = false;
    match config {
        Ok(lines) => {
            for line in &lines[qujian.0 .. qujian.1] {
                if line.contains("{") && started{
                    start = line.find("{").unwrap();
                    started = false;
                    ened = true;
                    continue;
                }
                if line.contains("[") {
                    continue;
                }
                if line.contains("}") {
                    if start < line.len() {
                        if start + 1 <= line.len() && line[start..start+1].trim() == "}" {
                            ened = false;
                            continue;
                        } 
                    }
                }
                if ened {
                    continue;
                }
                
                line_vec.push(line.trim().to_string());
            }
        }
        Err(err) => {
            println!("Error:{}",err);
        }
    }
    line_vec
}

fn get_all_lines(config:&Result<Vec<String>,io::Error>,path:&str) -> Vec<String> {
    let mut line_vec:Vec<String> = Vec::new();
    let qujian = get_config_qujian(config, path);
    match config {
        Ok(lines) => {
            for line in &lines[qujian.0 .. qujian.1] {
                line_vec.push(line.trim().to_string());
            }
        }
        Err(err) => {
            println!("Error:{}",err);
        }
    }
    line_vec
}

//获取string，默认返回空字符串，表示错误
pub fn get_config_string(config:&Result<Vec<String>,io::Error>, path:&str, key:&str) -> String{
    let qujian = get_config_qujian(config, path);
    let config_vec = get_config_lines(config, qujian);
    let mut ret = String::new();
    for line in config_vec {
        if line.contains(key) {
            let a = line.find("=");
            if let Some(start) = a {
                if line[..start].trim() != key {
                    continue;
                }
                if line.contains("\"") {
                    if line[..start].trim().replace('"', "") == key {
                        ret = line[start + 1..].trim().replace('"',"").to_string();
                        break;
                    } 
                }else {
                    println!("{}:不是一个字符串,默认返回空的String", line[start + 1..].trim())
                }
                
            }
        }
    }
    ret
}

//获取bool，默认返回false
pub fn get_config_bool(config:&Result<Vec<String>,io::Error>, path:&str, key:&str) -> bool{
    let qujian = get_config_qujian(config, path);
    let config_vec = get_config_lines(config, qujian);
    let mut ret = false;
    for line in config_vec {
        if line.contains(key) {
            let a = line.find("=");
            if let Some(start) = a {
                if line[..start].trim() == key {
                    let b = line[start + 1..].trim();
                    if b == "true" {
                        ret = true;
                        break;
                    }
                    else if b == "false" {
                        ret = false;
                    }
                    else {
                        println!("{}不是一个bool,默认返回false",b);
                    }
                }
            }
        }
    }
    ret
}

//获取int，返回00001122185323表示错误
pub fn get_config_int(config:&Result<Vec<String>,io::Error>, path:&str, key:&str) -> i32{
    let qujian = get_config_qujian(config, path);
    let config_vec = get_config_lines(config, qujian);
    let mut ret:i32 = 00001122185323;
    for line in config_vec {
        if line.contains(key) {
            let a = line.find("=");
            if let Some(start) = a {
                if line[..start].trim() == key{
                    let b = line[start + 1..].trim();
                    if !b.contains("\"") {
                        ret = match b.parse::<i32>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("{}不是int,返回:00001122185323表示错误", b);
                                00001122185323
                            }
                        }
                    }
                }
            }
        }
    }
    ret
}

//判断是否存在key
pub fn contains_key(config:&Result<Vec<String>,io::Error>, path:&str , key:&str) -> bool {
    let mut ret = false;
    let qujian = get_config_qujian(config, path);
    let config_vec = get_config_lines(config, qujian);
    for line in config_vec {
        if line.contains("=") {
            let a = line.find("=");
                if let Some(start) = a {
                if line[..start].trim() == key {
                    ret = true;
                }
            }
        }
    }
    ret
}


//判断是否存在path
pub fn contains_path(config:&Result<Vec<String>,io::Error>, superpath:&str, path:&str) -> bool{
    let mut ret = false;
    let all_lines = get_all_lines(config, superpath);
    for line in all_lines {
        if line.contains(path) {
            let a = line.find("[");
            let b = line.find("]");
            if let (Some(start), Some(end)) = (a, b) {
                if &line[start+1..end] == path {
                    ret = true;
                }
            }
            
        }
    }
    ret
}