#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number {
    number: i32,
}

#[get("/hello/<number>")]
fn hello(number: i32) -> String {
    format!("El numero que pasaste es: {}", number)
}

fn count_digits(number: i32) -> i32{
    number + 1
}

#[get("/value")]
fn value() -> Html<&'static str> {
    Html(
        r#"
        <form style='margin: 100px 0px; margin-bottom: 80px; text-aling:center;' action='/num' method='post'>
            <h2>Ingresa un numero </h2>
            <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
                width: 500px; font-size: 24px; border: 1px solid #1b95e0;' type='number' placeholder='Enter number' name='number' id='number'>
            <input style='font-size: 24px; padding:5px 12px; border: none; height: 40px; 
                border-radius: 3px;  color: white; background-color: #1b95e0;' type='Submit'>
        </form>
    "#,
    )
}

#[post("/num", data = "<number>")]
fn req(number: Form<Number>) -> Html<String> {
    let n_1 = count_digits(number.number);
    let s = format!(
        "<p style='font-size: 24px; color:#770494; font-weight: bolder;'>
   El numero que ingresaste es {} y escrito es:
   {}</p>",
        // number.number,    
        n_1,
        //number.number * 2
        match number.number{ 
            1 => "UNO",
            2 => "DOS",
            3 => "TRES",
            4 => "CUATRO",
            5 => "CINCO",
            6 => "SEIS",
            7 => "SIETE",
            8 => "OCHO",
            9 => "NUEVE",
            _ => "No encontrado",
        }
    );
    Html(s)
}

#[get("/")]
fn home() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Home</title>
            </head>
            <body>
                <div style="background-color:#fafafa; margin:1rem;text-align:center; border: 2px solid #ccc; padding:20px;">
                    <h2>Rust & Rocket</h2>
                    <p style="padding:1px;">Numero Cardinales</p>
                    <a href='/value' style='padding: 10px 20px; margin-top: 10px; 
                        font-size: 24px; color: white; background-color: #1b95e0; text-decoration: none; border-radius: 5px;'>Iniciemos</a><p></p>
            </body>
        </html>
    "#,
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, home, value, req])
        .launch();
}
