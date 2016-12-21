struct Complex{
    real: f32,
    imaginary: f32
}
impl Complex{
    fn new(real:f32, imaginary:f32) -> Complex{
        Complex{real: real, imaginary: imaginary}
    }
    fn fmt(&self) -> String{
        let pls = match self.imaginary >= 0.0{
            true => "+",
            _ => ""
        };
        format!("{}{}{}i", self.real, pls, self.imaginary)
    }
    fn print(&self){
        println!("{}", self.fmt())
    }
    fn times_ten(&self) -> Complex{
        Complex::new(self.real*10.0, self.imaginary*10.0)
    }
}
fn main() {
    let c1 = Complex{real: 2.0, imaginary: 5.0};
    let c2 = Complex::new(5.0, -2.0);
    println!("{}", c1.to_string());
    c1.print();
    println!(c1)
    c2.print();
    c2.times_ten().print();
}
