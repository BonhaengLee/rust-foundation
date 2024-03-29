use num::Complex;
use std::fs::File;
use std::str::FromStr;
use png::Encoder;
use std::env;

/// `bounds` 크기의 `pixels` 버퍼를 `filename` 파일에 기록한다.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let width = bounds.0 as u32;
    let height = bounds.1 as u32;
    let mut encoder = Encoder::new(output, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();

    Ok(())
}

/// `s`를 `"400x600"`이나 `"1.0,0.5"`와 같은 좌표 쌍으로 파싱한다.
///
/// `s`는 정확히 <left><sep><right> 형식이어야 하는데, sep은 `separator` 인수에 넘기는 문자이며,
/// left와 right는 둘다 `T::from_str`로 파싱될 수 있는 문자열이다.
/// `separator`는 반드시 아스키 문자여야 한다.
///
/// `s`가 올바른 형식으로 되어 있으면 `Some<(x, y)>`를 반환하고, 그렇지 않으면 `None`을 반환한다.
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T, T)> {
  match s.find(separator){
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
          (Ok(l), Ok(r)) => Some((l, r)),
          _ => None
      }
    }
  }
}

#[test]
fn test_parse_pair() {
  assert_eq!(parse_pair::<i32>("", ','), None);
  assert_eq!(parse_pair::<i32>("10,", ','), None);
  assert_eq!(parse_pair::<i32>(",10", ','), None);
  assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
  assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
  assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
  assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 5 {
    eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT",
              args[0]);
    eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
              args[0]);
    std::process::exit(1);
  }

  let bounds = parse_pair(&args[2], 'x')
    .expect("error parsing image dimensions");
  let upper_left = parse_complex(&args[3])
    .expect("error parsing upper left corner point");
  let lower_right = parse_complex(&args[4])
    .expect("error parsing lower right corner point");

  let mut pixels = vec![0; bounds.0 * bounds.1];

  let threads = 8;
  let rows_per_band = bounds.1 / threads + 1;

  {
    let bands: Vec<&mut [u8]> =
      pixels.chunks_mut(rows_per_band * bounds.0).collect();
      crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
          let top = rows_per_band * i;
          let height = band.len() / bounds.0;
          let band_bounds = (bounds.0, height);
          let band_upper_left = pixel_to_point(bounds,
                                              (0, top),
                                              upper_left,
                                              lower_right);
          let band_lower_right = pixel_to_point(bounds,
                                              (bounds.0, top + height),
                                              upper_left,
                                              lower_right);

          spawner.spawn(move || {
            render(band, band_bounds, band_upper_left, band_lower_right);
          });
        }
      }).unwrap();
  }

  write_image(&args[1], &pixels, bounds)
    .expect("error writing PNG file");
}

fn complex_square_add_loop(c: Complex<f64>) {
  let mut z = Complex { re: 0.0, im: 0.0 };
  loop {
    z = z * z + c;
  }
}

/// c가 망델브로 집합에 속하는지 확인
/// 확인에 ㄴ필요한 횟수 limit으로 제한.

/// 속하지 않으면 Some(i) 반환, i는 c가 원점을 중심으로 반경 2인 원을 벗어나는데 걸린 반복 횟수
/// limit에 도달할 떼까지 속하지 않는다고 판단하지 않으면 None 반환
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
  let mut z = Complex { re: 0.0, im: 0.0 };
  for i in 0..limit {
    if z.norm_sqr() > 4.0 {
      return Some(i);
    }
    z = z * z + c;
  }

  None
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
  match parse_pair(s, ','){
    Some((re, im)) => Some(Complex { re, im}),
    None => None,
  }
}

#[test]
fn test_parse_complex() {
  assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
  assert_eq!(parse_complex(",-0.0625"), None);
}

/// 결과 이미지 픽셀 좌표로 여기에 대응하는 복소평면 위의 점 반환
///
/// `bounds`는 픽셀 단위로 된 이미지의 폭과 높이를 갖는 쌍이다.
/// `pixel`은 특정 픽셀을 나타내는 (열, 행)이다.
/// `upper_left`와 `lower_right` 매개변수는 이미지가 커버하는 영역을 지정하는
/// 복소 평면 위의 점이다.
fn pixel_to_point(bounds: (usize, usize),
pixel: (usize, usize),
upper_left: Complex<f64>,
lower_right: Complex<f64>)
-> Complex<f64>
{
  let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
  Complex {
    re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
    im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    // pixel.1은 아래로 갈수록 증가한다.
  }
}

#[test]
fn test_pixel_to_point() {
  assert_eq!(pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.75 });
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 렌더링한다.
///
/// `bounds` 인수는 한 바이트 안에 회색조로 된 픽셀 하나가 들어가는 `pixels` 버퍼의 폭과 높이를 갖는다.
/// `upper_left`와 `lower_right` 인수는 픽셀 버퍼의 왼쪽 위 모서리와 오른쪽 아래 모서리를 나타내는 복소 평면 위의 점이다.
fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
  assert!(pixels.len() == bounds.0 * bounds.1);

  for row in 0..bounds.1 {
    for column in 0..bounds.0 {
      let point = pixel_to_point(bounds, (column, row),
      upper_left, lower_right);
      pixels[row * bounds.0 + column] = match escape_time(point, 255) {
        None => 0,
        Some(count) => 255 - count as u8,
      }
    }
  }
}