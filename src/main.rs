use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main] // Use the Actix Web runtime
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r###"
                <title>GCD Calulator Thingy</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute the GCD</button>
                </form>
            "###,
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring!!");
    }
    let response = 
        format!(
                "The greatest common denominator of {} and {} is <b>{}</b>.<br> <a href='http://localhost:3000'>back</a> ", form.n, form.m, gcd(form.n, form.m)
        );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
    
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let temp = m;
        m = n % m;
        n = temp;
    }
    n
}
