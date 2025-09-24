mod parser_se_config;

fn main(){
    let config = parser_se_config::read_all_lines("./config.se");
    println!("嵌套演示");
    //获取string
    let string = parser_se_config::get_config_string(&config, "Shen.Eternity", "Gover");
    //获取bool
    let boolean = parser_se_config::get_config_bool(&config, "Shen.Eternity", "endle");
    //获取i32
    let integer = parser_se_config::get_config_int(&config, "Shen.Eternity", "age");
    println!("Gover = {}",string);
    println!("endle = {}",boolean);
    println!("age = {}",integer);

    println!("非嵌套演示");
    //获取string
    let string2 = parser_se_config::get_config_string(&config, "Shen", "Gover");
    //获取bool
    let boolean2 = parser_se_config::get_config_bool(&config, "Shen", "endle");
    //获取i32
    let integer2 = parser_se_config::get_config_int(&config, "Shen", "Freq");
    println!("Gover = {}",string2);
    println!("endle = {}",boolean2);
    println!("age = {}",integer2);
}