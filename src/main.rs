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
    println!("-----------------------------------");
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
    println!("-----------------------------------");
    println!("检测键值不存在的情况");
    //获取string
    let string3 = parser_se_config::get_config_string(&config, "Shen", "不存在的键");
    //获取bool
    let boolean3 = parser_se_config::get_config_bool(&config, "Shen", "不存在的键");
    //获取i32
    let integer3 = parser_se_config::get_config_int(&config, "Shen", "不存在的键");
    println!("string 返回空 ::: {}",string3);
    println!("bool 默认返回false ::: {}",boolean3);
    println!("int 默认返回 1122185323 ::: {}",integer3);
    println!("-----------------------------------");
    println!("检测键是否存在");
    //检测键是否存在
    let boolkey = parser_se_config::contains_key(&config, "Shen.Eternity", "Gover");
    println!("存在 :: boolkey = {}",boolkey);
    let boolkey2 = parser_se_config::contains_key(&config, "Shen", "不存在的键");
    println!("不存在 :: boolkey2 = {}",boolkey2);
    println!("-----------------------------------");
    println!("检测路径是否存在");
    //检测路径是否存在
    let boolpath = parser_se_config::contains_path(&config, "Shen", "Eternity");
    println!("存在 :: boolpath = {}",boolpath);
    let boolpath2 = parser_se_config::contains_path(&config, "Shen", "不存在的路径");
    println!("不存在 :: boolpath2 = {}",boolpath2);
}