trait LampTime{
    fn getLampTime(&self)-> i32;
}
enum StreetLamp{
    green,
    red,
    yellow,
}

impl LampTime for StreetLamp{
    fn getLampTime(&self) -> i32{
      match *self{
            StreetLamp::green => 60,
            StreetLamp::red => 70,
            _=> 80,
        }
    } 
}


//计算集合的和
fn Sum(vec: &[u32])-> Option<u32> {
    let mut sums: u32=0;
    for i in vec{
       match  sums.checked_add(*i) {
            Some(s) => {sums=s;}
            None => {return None;}  
       };
    }
    Some(sums)
}

 
trait CalculateArea{
    fn calculat(&self) ;
}

impl CalculateArea for triangle{
    fn calculat(&self) {
       let areas= self.bottom*self.high/2;
       println!("area {}",areas);
    }
}


impl CalculateArea for square{
    fn calculat(&self) {
       let areas= self.length*self.length;
       println!("area {}",areas);
    }
}

struct triangle{
    bottom: u32,
    high: u32
}

struct square{
    length: u32
}

fn getArea<T:CalculateArea>(graphics: &T){
    graphics.calculat();
}

fn main() {
    //红绿灯
    let reds= StreetLamp::red;
    let times=reds.getLampTime();
    println!("red ={}",times);

    //计算集合的和
    let  v=[1,2,8,17];
    let sum=Sum(&v);    
    println!("sum ={:?}",sum);


    //三角形面积
    let triangle=triangle{bottom: 10,high: 6 };
    getArea(&triangle);

     //正方形面积
     let square=square{length: 5 };
     getArea(&square);
}


 
