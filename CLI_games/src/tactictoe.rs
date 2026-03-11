use crate::create_vec_map::VecMap;
use crate::input::input;
use std::collections::{HashMap, HashSet};
pub struct Tactictoe{
    map:VecMap,
    coordinate_have_symbol:HashMap<String,Vec<(usize,usize)>>,
}
impl Default for Tactictoe{
    fn default() -> Self {
        Tactictoe{
            map:VecMap::default(),
            coordinate_have_symbol:HashMap::new(),
        }
        
    }
}
impl Tactictoe{
    pub fn put_player_symbol(&mut self,symbol:&str,c:usize,r:usize)->bool{
        if &self.map.map[c][r] != " "{false}
        else{
            self.map.map[c][r] = symbol.to_string();
            true
        }
    }
    pub fn put_bot_symbol(&mut self,coordinate:&HashMap<usize,Vec<usize>>,symbol:&str)->(usize,usize){
        loop{
            let key = rand::random_range(1..=(self.map.map.len()*self.map.map[0].len()));
            if let Some(coor) = coordinate.get(&key){
                if self.map.map[coor[0]][coor[1]] != " "{continue;}
                else{self.map.map[coor[0]][coor[1]] = symbol.to_string();return (coor[0],coor[1])}
            };
        }
    }
    pub fn is_still_space(&self)->bool{
        for c in 0..self.map.map.len(){
            if self.map.map[c].contains(&" ".to_string()){return true}
        }
        false
    }

    pub fn win(&self,symbol:&str,win_length:i32)->bool{
        let val: &Vec<(usize, usize)> = match self.coordinate_have_symbol.get(symbol){
            Some(val) if val.len()>= win_length as usize =>val,
            _=>return false
        };
        let val_set :HashSet<_> = val.iter().copied().collect();
        let directions = [(1,0),(0,1),(1,1),(1,-1)];
        for &(x,y) in &val_set{
            for &(dx,dy) in &directions{
                let mut zx = x as i32;
                let mut zy = y as i32;
                let mut total_move = 1;
                while total_move < win_length{
                    zx += dx;
                    zy +=dy;
                    if zx  <0 || zy <0{
                        break
                    }
                    if val_set.contains(&(zx as usize,zy as usize)){
                        total_move +=1;
                        if total_move == win_length{
                            return true
                        }
                    }
                    else{break;}
                }
            }
        }
        false
    }
    pub fn show_map(&self,text:&str){
        print!("\x1B[2J\x1B[2H");
        self.map.cli_show_vec_map();
        println!("{}",text);
    }
    pub fn set_data(&mut self)->String{
        self.map.setup_map(7,7);
        if !self.map.can_setup{
            return "stop".to_string();
        }
        let mut symbol: String;
        print!("\x1B[2J\x1b[2H");
        loop{
            symbol = input("Please choice the symbol you want\nYou can choice 'x' or 'o'\n# 'x' can move first\n>").trim().to_string();
            if symbol.is_empty(){
                print!("\x1B[2J\x1b[2H");
                println!("❗Can't read the input. Please try again❗");
                continue;
            }
            let lower_symbol = symbol.to_lowercase();
            match lower_symbol.as_str(){
                "x" | "o" => {
                    symbol = lower_symbol;
                    break
                },
                "%q" => return "stop".to_string(),
                _=>{
                    println!("\x1B[2J\x1b[2H");
                    println!("❗Oops, sorry, I'm not a pro and I'm lazy, so I don't want you to use symbols other than 'x' or 'o'. Please try again❗");
                    println!("💡If you want to quit the game, please enter %q💡");
                    continue;
                }
            };  
        }
        symbol
    }
    pub fn game_start_cli(&mut self){
        loop{
            let win_length:i32 = 3;
            let symbol = self.set_data();
            if symbol == "stop"{return}
            let err_text_col = self.map.map[0].len() +4*self.map.map[0].len()+5;
            let mut coordinate:HashMap<usize,Vec<usize>> = HashMap::new();
            let mut key: usize = 0;
            for u_col in 0..self.map.map.len(){
                for u_row in 0..self.map.map[0].len(){
                    key+=1;
                    coordinate.insert(key, vec![u_col,u_row]);
                }
            };
            let bot_symbol: String = if symbol == "x"{"o".to_string()} else{"x".to_string()};
            if bot_symbol == "x"{
                let key = self.put_bot_symbol(&coordinate,&bot_symbol.to_string());
                self.coordinate_have_symbol.entry(bot_symbol.clone()).or_default().push(key);
            }
            self.show_map("");
            loop{
                let ans = input(
                    &format!("\x1B[{}Hplease enter the the coordinate (1-{})\n>"
                    ,self.map.map.len()*2+3
                    ,coordinate.len()));
                match ans.clone().as_str(){
                    "%q"=>return,
                    "%r"=>{
                        break;
                    }
                    _=>()
                }
                let key: usize;
                key = match ans.trim().to_lowercase().as_str().parse(){
                    Ok(a)=>a,
                    Err(_)=>{
                        let err_mag = format!("\x1B[2;{0}H❗Oops, seems like your input isnt a number. Please try again❗",err_text_col)
                                +&format!("\x1B[2;{0}H❗Oops, seems like your input isnt a number. Please try again❗",err_text_col)
                                +&format!("\x1B[3;{0}H💡If you want to quit, please enter %q💡",err_text_col)
                                +&format!("\x1B[4;{0}H💡If you want to restart the game, please enter %r💡",err_text_col);
                        self.show_map(&err_mag);
                        continue;
                        }
                };
                if !(1..=coordinate.len()).contains(&key){
                    self.show_map(
                        &format!("\x1B[2;{}H❗Oops, looks like the input isn't in range (1–{}). Please try again❗"
                        ,err_text_col,coordinate.len()));
                    continue;
                }
                let coor = match coordinate.get(&key){
                    Some(c)=>c,
                    _=>{
                        continue;
                    }
                };
                if !self.put_player_symbol( &symbol, coor[0], coor[1]){
                    self.show_map(
                        &format!("\x1B[2;{}H❗Oops, you can't cover the enemy symbol. Please try again❗"
                        ,err_text_col));
                    continue;
                }
                self.coordinate_have_symbol.entry(symbol.clone()).or_default().push((coor[0],coor[1]));
                if self.win(&symbol,win_length){
                    self.show_map("You win!",);
                    let _ = input("Please press enter to continue");
                    return
                }
                if !self.is_still_space(){
                    self.show_map("draw");
                    let _ = input("Please press enter to continue");
                    return
                }
                let bot_puted_place = self.put_bot_symbol(&coordinate,&bot_symbol.to_string());
                self.coordinate_have_symbol.entry(bot_symbol.clone()).or_default().push(bot_puted_place);
                if self.win(&bot_symbol,win_length){
                    self.show_map("You lose",);
                    let _ = input("Please press enter to continue");
                    return
                }
                self.show_map("",);
            };
        };
    }
}
