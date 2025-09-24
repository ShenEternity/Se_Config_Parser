mod parser_se_config;

fn main(){
    let config = parser_se_config::read_all_lines("./config.se");
    println!("{}",parser_se_config::get_config_int(&config, "Shen", "Freq2"));
    println!("{}",parser_se_config::get_config_string(&config, "Shen.Eternity", "Gover"));
    println!("{}",parser_se_config::get_config_bool(&config, "Shen.Eternity", "endle"));
    println!(".......................");
    println!("{}",parser_se_config::get_config_int(&config, "Shen.Eternity", "Gover"));
    println!("{}",parser_se_config::get_config_string(&config, "Shen", "Freq"));
    println!("{}",parser_se_config::get_config_bool(&config, "Shen.Eternity", "Gover"));
    println!(".......................");
    println!("{}",parser_se_config::contains_key(&config, "Shen", "Freq"));
    println!("{}",parser_se_config::contains_path(&config, "Shen", "Eternity"));
    println!(".......................");
    println!("{}",parser_se_config::contains_key(&config, "Shen", "Freqw"));
    println!("{}",parser_se_config::contains_path(&config, "Shen", "Eternit"));
}