use std::io::BufWriter;
use std::io::Write;
pub struct VecMap{
    pub int_map:Vec<Vec<i32>>,
    pub map:Vec<Vec<String>>,
    pub can_setup:bool,
}
impl Default for VecMap {
    fn default()->Self{
        VecMap{
            int_map:Vec::new(),
            map:Vec::new(),
            can_setup:false,
        }
    }
}
impl VecMap {
    pub fn setup_map(&mut self,row:usize,col:usize){
        if row<2 || col<2 || row>24 || col>24{
            println!("the map cant smailer than 2x2 or bigger than 25x25");
            self.can_setup = false;
        }
        else {
            self.int_map = vec![vec![0;row];col];
            self.map = vec![vec![" ".to_string();row];col];
            self.can_setup = true;
        }
    }
    pub fn cli_show_vec_map(&self){
        fn draw_top(result:&mut BufWriter<std::io::StdoutLock<'static>>,long: usize,space_of_a_square:usize){
            write!(result,"{}","┌").unwrap();
            for top_lines in 1..long{
                write!(result,"{}",if top_lines%(space_of_a_square+1)!=0{"─"} else{"┬"}).unwrap();
            }
            writeln!(result,"{}","┐").unwrap();
        }
        fn draw_mid(map:&Vec<Vec<String>>,result:&mut BufWriter<std::io::StdoutLock<'static>>,space_of_a_square:usize,row: usize,c: usize){
            for r in 0..row{
                write!(result,"│{:^sofs$}",map[c][r],sofs=space_of_a_square).unwrap();
            }
            writeln!(result,"{}","│").unwrap();
        }
        fn draw_bottom(result:&mut BufWriter<std::io::StdoutLock<'static>>,long: usize,space_of_a_square:usize,c: usize,col: usize){
            write!(result,"{}",if c == col-1{"└"} else{"├"}).unwrap();
            for top_lines in 1..long{
                write!(result,"{}",if top_lines%(space_of_a_square+1)!=0{"─"} else{if c==col-1{"┴"} else{"┼"}}).unwrap();
            }
            writeln!(result,"{}",if c == col-1{"┘"} else{"┤"}).unwrap();
        }
        if !self.can_setup{
            return
        }
        let row: usize = self.map[0].len();
        let col: usize = self.map.len();
        let space_of_a_square: usize = 4;
        let long = row*space_of_a_square+row;
        let mut result: BufWriter<std::io::StdoutLock<'static>> = BufWriter::new(std::io::stdout().lock());
        
        draw_top(&mut result, long, space_of_a_square);
        for c in 0..col{
            draw_mid(&self.map, &mut result, space_of_a_square,row, c);
            draw_bottom(&mut result, long, space_of_a_square,c, col);
        }
    }
}