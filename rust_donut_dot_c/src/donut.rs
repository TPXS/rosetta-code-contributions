use ndarray::Array;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;

const SCREEN_SIZE: usize = 40;
const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;
const ILLUMINATI: [&str; 12] = [".", ",", "-", "~", ":", ";", "=", "!", "*", "#", "$", "@"];

const R1: f32 = 1.;
const R2: f32 = 2.;
const K2: f32 = 5.;
const K1: f32 = SCREEN_SIZE as f32 * K2 * 3. / (8. * (R1 + R2));

pub fn render_frame(a: f64, b: f64) -> ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>> {
    //et (cos_a, cos_b, sin_a, sin_b) = (a.cos(), b.cos(), a.sin(), b.sin());
    let (cos_a, sin_a) = cos_sin(a);
    let (cos_b, sin_b) = cos_sin(b);
    let mut output = Array::from_elem((SCREEN_WIDTH, SCREEN_HEIGHT), " ");
    let mut z_buffer = Array::<f64, _>::zeros((SCREEN_WIDTH, SCREEN_HEIGHT));


    
    let theta = Array::range(0.0, 2.0 * std::f64::consts::PI, THETA_SPACING);
    let cos_theta = theta.mapv(f64::cos);
    let sin_theta = theta.mapv(f64::sin);

    let phi = Array::range(0.0, 2.0 * std::f64::consts::PI, PHI_SPACING);
    let cos_phi = phi.mapv(f64::cos);
    let sin_phi = phi.mapv(f64::sin);

    let circle_x = R2 + R1 * &cos_theta;
    let circle_y = R1 * &sin_theta;


    let circle_y_ab_sin = &circle_y * cos_a * sin_b;
    let circle_y_ab_cos = &circle_y * cos_a * cos_b;
    let cos_plus_sin = cos_b * &cos_phi + sin_a * sin_b * &sin_phi;
    let sin_min_cos = sin_b * &cos_phi - sin_a * cos_b * &sin_phi;

    for i in 0..90 {
        let x = circle_x[i] * &cos_plus_sin - circle_y_ab_sin[i];
        let y = circle_x[i] * &sin_min_cos + circle_y_ab_cos[i];
        let z = K2 + cos_a * circle_x[i] * &sin_phi + circle_y[i] * sin_a;
        let ooz = 1. / z;

        let x_p: Array<f64, _> = SCREEN_WIDTH as f64 / 2.0 + K1 * &ooz * x;
        let x_p = x_p.mapv(|x| x as usize);

        let y_p: Array<f64, _> = SCREEN_HEIGHT as f64 / 2.0 - K1 * &ooz *y;
        let y_p = y_p.mapv(|y| y as usize);
        
        let luminance = (cos_theta[i] * &cos_phi * sin_b - cos_a * &cos_theta[i] * &sin_phi - sin_a * &sin_theta[i] + cos_b * (cos_a * &sin_theta[i] - &cos_theta[i] * sin_a * &sin_phi)) * 8.0;
        let luminance = luminance.mapv(|element| element.round() as isize);

        for i in 0..luminance.size() {
            if (luminance[i] >= 0) & (ooz[i] > z_buffer[[x_p[i], y_p[i]]]) {
                z_buffer[[x_p[i], y_p[i]]] = ooz[i];
                output[[x_p[i], y_p[i]]] = ILLUMINATION[luminance[i] as usize];
            }
        }
    }
    output
}

pub fn run_donut(mut a: f32, mut b: f32) {
    // Renders multiple frames of 3D donut

    for _ in 0..SCREEN_SIZE * SCREEN_SIZE {
        a += THETA_SPACING;
        b += PHI_SPACING;
        let frame = render_frame(a, b);
        print!("\x1b[H");
        for row in frame.outer_iter() {
            println!("{}", row.to_vec().join(" "));
        }
    }
