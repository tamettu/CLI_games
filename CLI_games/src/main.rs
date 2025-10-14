mod mod_for_make_map;
use mod_for_make_map::VecMap;
use std::{collections::BTreeMap, io::Write};
use std::collections::HashMap;
fn input(oput:&str)->String{
    let mut input: String = String::new();
    print!("{}",oput);
    std::io::stdout().flush().unwrap();
    if std::io::stdin().read_line(&mut input).is_ok(){
        input.trim().to_string()
    }
    else{
        "".to_string()
    }
}
struct Tactictoe{
    map:VecMap,
    coordinate_have_symbol:BTreeMap<String,Vec<usize>>,
}
impl Default for Tactictoe{
    fn default() -> Self {
        Tactictoe{
            map:VecMap::default(),
            coordinate_have_symbol:BTreeMap::new()
        }
        
    }
}
impl Tactictoe{
    fn put_player_symbol(&mut self,symbol:&str,c:usize,r:usize)->bool{
        if &self.map.map[c][r] != " "{false}
        else{
            self.map.map[c][r] = symbol.to_string();
            true
        }
    }
    fn put_bot_symbol(&mut self,coordinate:&HashMap<usize,Vec<usize>>,symbol:&str)->usize{
        let _ran = rand::rng();
        loop{
            let key = rand::random_range(1..(self.map.map.len()*self.map.map[0].len()));
            if let Some(coor) = coordinate.get(&key){
                if self.map.map[coor[0]][coor[1]] != " "{continue;}
                else{self.map.map[coor[0]][coor[1]] = symbol.to_string();return key}
            };
        }
    }
    fn is_still_space(&self)->bool{
        for c in 0..self.map.map.len(){
            if self.map.map[c].contains(&" ".to_string()){return true}
        }
        false
    }
    fn win(&self,symbol:&str)->bool{
        let mut val:Vec<usize> = Vec::new();
        if let Some(values) = self.coordinate_have_symbol.get(symbol){
            if values.len()<3{return false}
            val = values.iter().cloned().collect();
        }
        if val.contains(&3) && val.contains(&5) && val.contains(&7) 
        || val.contains(&1) && val.contains(&5) && val.contains(&9){
            println!("1");
            return true
        }
        val.sort_unstable();
        if val.windows(3)
        .any(|val|val[0]%3!=0 
            && val[2] - val[0] == 2){println!("2");return true}
        for v in val.clone(){
            if val.contains(&(v+3)) 
            && val.contains(&(v+6)){return true}
        }
        false
    }
    fn show_map(&self,text:&str){
        println!("\x1B[2J\x1B[H");
        self.map.cli_show_vec_map();
        println!("{}",text);
    }
    fn game_start_cli(&mut self){
        self.map.setup_map(3,3);
        let mut coordinate:HashMap<usize,Vec<usize>> = HashMap::new();
        let mut key: usize = 0;
        for u_col in 0..self.map.map.len(){
            for u_row in 0..self.map.map[0].len(){
                key+=1;
                coordinate.insert(key, vec![u_col,u_row]);
            }
        };
        let mut symbol: String;
        self.show_map("");
        loop{
            symbol = input("Please choice the symbol you want\nYou can choice 'x' or 'o'\n# 'x' can move first\n>").trim().to_string();
            if symbol.is_empty(){
                self.show_map("❗Can't read the input. Please try again❗");
                continue;
            }
            let lower_symbol = symbol.to_lowercase();
            match lower_symbol.as_str(){
                "x" | "o" => {
                    symbol = lower_symbol;
                    break
                }
                _=>{
                    self.show_map("❗Oops, sorry, I'm not a pro and I'm lazy, so I don't want you to use symbols other than 'x' or 'o'. Please try again❗");
                    continue;
                }
            };  
        }
        let bot_symbol: String = if symbol == "x"{"o".to_string()} else{"x".to_string()};
        if bot_symbol == "x"{
            let key = self.put_bot_symbol(&coordinate,&bot_symbol.to_string());
            self.coordinate_have_symbol.entry(bot_symbol.clone()).or_default().push(key);
        }
        self.show_map("");
        loop{
            let ans = input("please enter the the coordinate (1-9)\n>");
            let mut key: usize;
                key = match ans.trim().to_lowercase().as_str().parse(){
                    Ok(a)=>a,
                    Err(_)=>{
                        self.show_map("❗Oops, seems like your input isnt a number. Please try again❗");
                        continue;
                    }
                };
            if !(1..=9).contains(&key){
                self.show_map("❗Oops, looks like the input isn't in range (1–9). Please try again❗");
                continue;
            }
            let coor = match coordinate.get(&key){
                Some(c)=>c,
                _=>{
                    continue;
                }
            };
            if !self.put_player_symbol( &symbol, coor[0], coor[1]){
                self.show_map("❗Oops, you can't cover the enemy symbol. Please try again❗");
                continue;
            }
            self.coordinate_have_symbol.entry(symbol.clone()).or_default().push(key);
            if self.win(&symbol){
                self.show_map("You win!",);
                return
            }
            if !self.is_still_space(){println!("draw");return}
            key = self.put_bot_symbol(&coordinate,&bot_symbol.to_string());
            self.coordinate_have_symbol.entry(bot_symbol.clone()).or_default().push(key);
            if self.win(&bot_symbol){
                self.show_map("You lose",);
                return
            }
            self.show_map("",);
        };
        
    }
}

fn main() {
    let mut game_tactictoe = Tactictoe::default();
    game_tactictoe.game_start_cli();
}
