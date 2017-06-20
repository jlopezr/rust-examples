pub struct Complex {
    real: f32,
    imag: f32
}

impl Complex {
    fn new(r: f32, i:f32) -> Complex {
        Complex {real: r, imag:i}
    }

    fn to_string(&self) -> String {
        format!("{}+{}i",self.real,self.imag)
    }

    fn add(&self, c:Complex) -> Complex {
        Complex {real: self.real + c.real, imag: self.imag + c.imag}
    }

    fn times_ten(&mut self ) {
        self.real = 10.0 * self.real;
        self.imag = 10.0 * self.imag;
    }

    fn abs(&self) -> f32 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
    let c1 = Complex::new(1f32,2f32);
    let c2 = Complex::new(3f32,4f32);
    println!("{}", c1.to_string());
    println!("{}", c2.to_string());
    let mut c3 = c1.add(c2);
    println!("{}", c3.to_string());
    c3.times_ten();
    println!("{}", c3.to_string());
    println!("{}", c1.abs());
}

#[cfg(test)]
//mod test {
    #[test]
    fn test_create() {
        let c1 = Complex::new(1f32,2f32);
        let c2 = Complex::new(3f32,4f32);
        assert_eq!(c1.real, 1f32);
        assert_eq!(c1.imag, 2f32);
        assert_eq!(c2.real, 3f32);
        assert_eq!(c2.imag, 4f32);
    }

    #[test]
    fn test_add() {
        let c1 = Complex::new(1f32,2f32);
        let c2 = Complex::new(3f32,4f32);
        let c3 = c1.add(c2);
        assert_eq!(c3.real, 4f32);
        assert_eq!(c3.imag, 6f32);
    }

    #[test]
    fn test_times_ten() {
        let mut c1 = Complex::new(1f32,2f32);
        c1.times_ten();
        assert_eq!(c1.real, 10f32);
        assert_eq!(c1.imag, 20f32);
    }

    #[test]
    fn test_abs() {
        let c1 = Complex::new(1f32,2f32);
        let a = c1.abs();
        assert_eq!(a, 2.236068f32);
    }

    #[test]
    fn test_to_string() {
        let c1 = Complex::new(1f32,2f32);
        let s = c1.to_string();
        assert_eq!(s, "1+2i");
    }
//}
