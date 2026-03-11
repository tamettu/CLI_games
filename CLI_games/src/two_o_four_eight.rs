use crate::create_vec_map::VecMap;
use crate::input::input;
pub struct  TwoOFourEight{
    map:VecMap,
    empty_coordinate:Vec<(usize,usize)>
}
impl Default for TwoOFourEight{
    fn default() ->Self{
        TwoOFourEight{
            map:VecMap::default(),
            empty_coordinate:Vec::new()
        }
    }
}
impl TwoOFourEight{
    fn randomly_put_digit(&mut self){
        if self.empty_coordinate.len()!=0{
            let  (y,x) = self.empty_coordinate[rand::random_range(0..self.empty_coordinate.len())];
            self.map.int_map[y][x] = if rand::random_range(0..100) <80 {4} else {2};
        }
    }
    fn move_core(&self,dy:i32,dx:i32,long:i32){
        for _ in 0..=long{
            for y in 0..self.map.map.len(){
                let zy = y as i32*dy;
                if zy<0 || zy >= self.map.map.len()as i32{return}
                for x in 0..=self.map.map[0].len(){
                    let zx = x as i32*dx;
                    if zx<0 || zx >= self.map.map[0].len()as i32{return}
                    
                }
            }
        }
    }
}