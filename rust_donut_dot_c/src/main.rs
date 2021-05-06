/*
#[allow(non_snake_case)]
#[allow(clippy::many_single_char_names)]

fn main() {
    let mut A = 0.0f32;
    let mut B = 0.0f32;
    let sleep_time = std::time::Duration::from_millis(std::env::var("DONUT_SLEEP").unwrap_or("30".to_string()).parse::<u64>().unwrap_or(30));

    print!("\x1b[2J"); // erase display

    loop {
        let mut z: [f32; 1760] = [0.0; 1760];
        let mut b: [char; 1760] = [' '; 1760];

        for j in (0..628).step_by(7) {
            let j = j as f32 * 0.01;

            for i in (0..628).step_by(7) {
                let i: f32 = i as f32 * 0.01;

                let c: f32 = i.sin();
                let d: f32 = j.cos();
                let e: f32 = A.sin();
                let f: f32 = j.sin();
                let g: f32 = A.cos();
                let h: f32 = d + 2.0;
                let D: f32 = 1.0 / (c * h * e + f * g + 5.0);
                let l: f32 = i.cos();
                let m: f32 = B.cos();
                let n: f32 = B.sin();
                let t: f32 = c * h * g - f * e;
                let x: i32 = (40.0 + 30.0 * D * (l * h * m - t * n)) as i32;
                let y: i32 = (12.0 + 15.0 * D * (l * h * n + t * m)) as i32;
                let o: i32 = x + 80 * y;
                let N: i32 =
                    (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as i32;

                if 22 > y && y > 0 && x > 0 && 80 > x && D > z[o as usize] {
                    z[o as usize] = D;
                    let idx: usize = if N > 0 { N as usize } else { 0 };
                    b[o as usize] = b".,-~:;=!*#$@"[idx] as char;
                }
            }
        }

        print!("\x1b[H"); // reset cursor to home position

        for (k, pixel) in b.iter().enumerate() {
            if k % 80 != 0 {
                print!("{}", pixel);
            } else {
                println!();
            }

            A += 0.00004;
            B += 0.00002;
        }

        std::thread::sleep(sleep_time);
    }
}
*/

const ILLUMINATION: &str = &".,-~:;=!*#$@";
const THETA_SPACING: f64 = 0.07;
const PHI_SPACING: f64 = 0.02;

const R1: f64 = 1.0;
const R2: f64 = 2.0;
const K2: f64 = 5.0;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize  = 40;

const K1 : f64 = SCREEN_WIDTH as f64 * K2* 3.0/(8.0*(R1+R2));

fn calculate_luminance(cos_phi: f64, sin_phi: f64, cos_theta: f64, sin_theta: f64, cos_a: f64, sin_a: f64, cos_b: f64, sin_b: f64) -> f64 {
    cos_phi*cos_theta*sin_b 
    - cos_a*cos_theta*sin_phi 
    - sin_a*sin_theta 
    + cos_b*(cos_a*sin_theta - cos_theta*sin_a*sin_phi)
}

fn cos_sin(angle: f64) -> (f64 ,f64) {
    (angle.cos(), angle.sin())
}

fn x_y_circle (cos_theta: f64, sin_theta: f64) -> (f64, f64) {
    // the x,y coordinate of the circle, before revolving
    (R2 + R1*cos_theta, R1*sin_theta)
}

fn x_y_z_circle (circle_x: f64, circle_y: f64, cos_a: f64, sin_a: f64, cos_b: f64, sin_b: f64, cos_phi: f64, sin_phi: f64) -> (f64, f64, f64) {
    // final 3D (x,y,z) coordinate after rotations 
    (circle_x * (cos_b*cos_phi + sin_a*sin_b*sin_phi) - circle_y*cos_a*sin_b,
    circle_x * (sin_b*cos_phi - sin_a*cos_b*sin_phi) + circle_y*cos_a*cos_b,
    K2 + cos_a*circle_x*sin_phi + circle_y*sin_a)
}
fn render_frame(a: f64, b: f64) {
    let (cos_a, sin_a) = cos_sin(a);
    let (cos_b, sin_b) = cos_sin(b);
    let mut output = [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
    let mut z_buffer = [[0f64; SCREEN_HEIGHT]; SCREEN_WIDTH];

    let mut theta = 0f64;

    // theta goes around the cross-sectional circle of a torus
    while theta < 2.0 * std::f64::consts::PI {
        let (cos_theta, sin_theta) = cos_sin(theta);
        theta += THETA_SPACING;
        // the x,y coordinate of the circle, before revolving 
        let (circle_x, circle_y) = x_y_circle(cos_theta, sin_theta);  

        // phi goes around the center of revolution of a torus
        let mut phi = 0f64;
        while phi < 2.0 * std::f64::consts::PI {
            let (cos_phi, sin_phi) = cos_sin(phi);
            phi += PHI_SPACING;
            // final 3D (x,y,z) coordinate after rotations 
            let (x, y, z) = x_y_z_circle(circle_x, circle_y, cos_a, sin_a, cos_b, sin_b, cos_phi, sin_phi);
            let ooz = 1.0/z;

            //x & y projections
            let x_p = (SCREEN_WIDTH as f64  / 2.0 + K1*ooz*x) as usize;
            let y_p = (SCREEN_HEIGHT as f64 / 2.0 - K1*ooz*y) as usize;

            let luminance = calculate_luminance(cos_phi, sin_phi, cos_theta, sin_theta, cos_a, sin_a, cos_b, sin_b);

            if luminance > 0f64 && x_p < SCREEN_WIDTH && y_p < SCREEN_HEIGHT {
                //println!("here");
                if ooz > z_buffer[x_p][y_p] {
                    z_buffer[x_p][y_p] = ooz;
                    let luminance_index = (luminance * 8.0).floor() as usize;
                    output[x_p][y_p] = ILLUMINATION.chars().nth(luminance_index).unwrap();
                }
            }
            
        }
    }
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            print!("{}", output[i][j]);
        }
        println!();
    }
}

fn run_donut(mut a: f64, mut b: f64, sleep_time: std::time::Duration) {
    loop {
        a+=THETA_SPACING;
        b+=PHI_SPACING;
        render_frame(a, b);
        if a > 100.0 {
            a = 0.0;
            b = 0.0;
        }
        std::thread::sleep(sleep_time);

    }
}
fn main() {
    print!("\x1b[2J"); // erase display
    let sleep_time = std::time::Duration::from_millis(std::env::var("DONUT_SLEEP").unwrap_or(String::from("30")).parse::<u64>().unwrap_or(30));
    run_donut(0.0, 0.0, sleep_time);
}

