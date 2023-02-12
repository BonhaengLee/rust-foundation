use serde::Deserialize;
use futures::future;
use actix_web::{web, App, HttpResponse, HttpServer};

#[derive(Deserialize)]
struct GcdParameters {
  num1: u64,
  num2: u64,
}

async fn get_index() -> HttpResponse {
  HttpResponse::Ok()
  .content_type("text/html")
  .body(
    r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="num1" />
          <input type="text" name="num2" />
          <button type="submit">Compute GCD</button>
        </form>
    "#,
  )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }
  n
}


async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
  if form.num1 == 0 || form.num2 == 0 {
    return HttpResponse::BadRequest()
      .content_type("text/html")
      .body("Computing the GCD with zero is boring.");
  }

  let response = format!(
    "The greatest common divisor of the numbers {} and {} \
    is <b>{}</b>\n",
    form.num1, form.num2, gcd(form.num1, form.num2));

  HttpResponse::Ok()
    .content_type("text/html")
    .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server = HttpServer::new(|| {
      App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
  });

  println!("Serving on http://localhost:3000...");
  future::ready(
    server
      .bind("127.0.0.1:3000")
      .map_err(|e| {
          eprintln!("Can not bind server to {}:{}", "127.0.0.1", 3000);
          e
      })
  )
  .await
  .map_err(|e| e)?
  .run()
  .await
}