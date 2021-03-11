use std::io;

// TODO Change to option return
fn f(x: f64) -> f64 {
  (x.powi(3)) + x - 1.0
}

fn avrg(a: f64, b: f64) -> f64 {
  (a + b) / 2.0
}

pub fn input() -> (f64, f64, i32) {
  let mut n1: f64 = match float_input("Input first number of closed range:") {
    Ok(i) => i,
    Err(_) => panic!("An error was risen when reading/parsing the input"),
  };
  let mut n2: f64 = match float_input("Input second and last number of closed range:") {
    Ok(i) => i,
    Err(_) => panic!("An error was risen when reading/parsing the input"),
  };
  let n: i32 = match int_input("Input required decimals of result: ") {
    Ok(i) => i,
    Err(_) => panic!("An error was risen when reading/parsing the input"),
  };

  if n1 > n2 {
    let tmp = n2;
    n2 = n1;
    n1 = tmp;
  }

  if n < 0 {
    panic!("A positive exponent to round to is required");
  }

  /*
  let i1: i32 = n1;
  let i2: i32 = n2;
  for i in i1..i2 {
    if let None = f(i.into()) {
      panic!("The function is not defined in point {} inside the provided range", i)
    }
  }
  */

  if !((f(n1) < 0.0 && f(n2) > 0.0) || (f(n1) > 0.0 && f(n2) < 0.0)) {
    panic!(
      "The Bolzano theorem requires a positive and negative number (f({})={} and f({})={} are not)",
      n1,
      f(n1),
      n2,
      f(n2)
    );
  }

  (n1, n2, n)
}

fn float_input<'a>(text: &str) -> Result<f64, &'a str> {
  let mut i = String::new();
  let n: f64;

  println!("{}", text);
  loop {
    io::stdin().read_line(&mut i).expect("Failed to read input");
    n = match i.trim().parse() {
      Ok(i) => i,
      Err(_) => {
        return Err("Invalid input");
      }
    };
    break;
  }

  Ok(n)
}

fn int_input<'a>(text: &str) -> Result<i32, &'a str> {
  let mut i = String::new();
  let n: i32;

  println!("{}", text);
  loop {
    io::stdin().read_line(&mut i).expect("Failed to read input");
    n = match i.trim().parse() {
      Ok(i) => i,
      Err(_) => {
        return Err("Invalid input");
      }
    };
    break;
  }

  Ok(n)
}

pub fn solve(mut a: f64, mut b: f64, n: i32) -> f64 {
  let mut c: f64;
  let mut count = 0;
  loop {
    count += 1;
    c = avrg(a, b);
    println!(
      "run {count}> a={a} [f(a)={fa}], b={b} [f(b)={fb}, c={c} [f(c)={fc}]. Yeah? {t} ({d})",
      count = count,
      a = a,
      fa = f(a),
      b = b,
      fb = f(b),
      c = c,
      fc = f(c),
      t = (f(c)*(10.0_f64.powi(n))).round() / 10.0_f64.powi(n) == 0.0,
      d = (f(c)*(10.0_f64.powi(n))).round() / 10.0_f64.powi(n)
    );
    if (f(c) == 0.0) || (f(c)*(10.0_f64.powi(n))).round() / 10.0_f64.powi(n) == 0.0  {
      println!("Found matching c!");
      break;
    } else if f(c) > 0.0 {
      b = c;
    } else if f(c) < 0.0 {
      a = c;
    } else {
      panic!("An unexpected situation was reached during calculations")
    };
  };
  c
}
