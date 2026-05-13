std, tokio, winapi, glib, gstreamer, wgpu, cpal 

https://rustup.rs/

rustup-init.exe

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

Select 1

rustc --version

cargo new rstorch
cd rstorch

cargo run

Rust dasturlash tilida `cargo` - bu loyihani boshqarish, paketlarni yuklash, kompilyatsiya qilish va testlash uchun ishlatiladigan asosiy vositadir.

## 1. Loyiha yaratish
* `cargo new <nomi>` : Yangi binary (ishga tushsa bo'ladigan) loyiha yaratadi.
* `cargo new <nomi> --lib` : Yangi kutubxona (library) loyihasini yaratadi.
* `cargo init` : Mavjud papka ichida Rust loyihasini inisializatsiya qiladi.

## 2. Qurish va Tekshirish (Build)
* `cargo build` : Loyihani debug rejimida kompilyatsiya qiladi (target/debug/).
* `cargo build --release` : Loyihani maksimal optimizatsiya bilan kompilyatsiya qiladi (target/release/).
* `cargo check` : Kodni kompilyatsiya qilmasdan, xatolarga tekshiradi (tezkor tekshiruv).
* `cargo clean` : Barcha kompilyatsiya natijalarini (target papkasini) o'chirib tashlaydi.

## 3. Ishga tushirish va Testlash
* `cargo run` : Loyihani kompilyatsiya qiladi va darhol ishga tushiradi.
* `cargo test` : Loyihadagi barcha unit va integratsion testlarni ishga tushiradi.

## 4. Paketlar (Crates) bilan ishlash
* `cargo add <paket>` : Cargo.toml fayliga yangi kutubxonani qo'shadi.
* `cargo update` : Bog'liqliklarni (dependencies) ruxsat etilgan versiyagacha yangilaydi.
* `cargo install <paket>` : Rustda yozilgan tayyor utilitalarni tizimga o'rnatadi.

## 5. Dokumentatsiya va Formatlash
* `cargo doc --open` : Loyihangiz va barcha kutubxonalar uchun HTML qo'llanma yaratadi va ochadi.
* `cargo fmt` : Kodni Rust standartiga ko'ra formatlaydi.
* `cargo clippy` : Kodingizni tahlil qilib, sifatni oshirish uchun maslahatlar beradi.


Docs
-------------------------------------------------------------
JavaScript
const foo = 1;

Rust
let foo = 1;
Rust
let foo: i32 = 1;
--------------------------------------------------------------------
1. Butun sonlar (Integer)
Tur,Tavsif,JavaScript muqobili
i8,     8 bitli butun son (-128 dan 127 gacha),Int8Array
i16,    "16 bitli butun son (-32,768 dan 32,767 gacha)",Int16Array
i32,    32 bitli butun son. Rustda standart butun son.,`(x
i64,    64 bitli butun son. Juda katta sonlar uchun.,BigInt
i128,   128 bitli butun son. Astronomik sonlar uchun.,-
isize,  Kompyuter arxitekturasiga bog'liq (32 yoki 64 bit). Indexlar uchun.,array.length

2. Faqat musbat butun sonlar (Unsigned)
Tur,Tavsif
u8,     0 dan 255 gacha. Bytelar bilan ishlashda ko‘p ishlatiladi.
u16,    "0 dan 65,535 gacha."
u32,    0 dan 4 milliardgacha.
u64,    Juda katta musbat sonlar.
u128,   O'ta katta musbat sonlar.
usize,  Xotira manzillari va massiv uzunligi uchun (Eng ko'p ishlatiladigani).

3. O‘nlik sonlar (Floating-Point)
Tur,    Tavsif,JavaScript muqobili
f32,    "32 bitli o'nlik son. Tez ishlaydi, lekin aniqligi pastroq.",-
f64,    64 bitli o'nlik son. Rustda standart.,Number (Oddiy sonlar)
----------------------------------------------------------------
Rust kodi,Ma'nosi
let a: u8 = 255;,Eng kichik musbat tur (0 dan 255 gacha)
let b: u32 = 1000;,Standart musbat butun son
let c: u64 = 1000000;,Katta musbat butun son

Rust kodi,Ma'nosi
let d: i8 = -128;,Kichik manfiy son
let e: i32 = -5000;,Standart manfiy/musbat son (Eng ko'p ishlatiladi)
let f: i32 = 5000;,i32 turidagi musbat son

Rust kodi,Ma'nosi
let g: f32 = 3.14;,Oddiy o'nlik son (AI uchun standart)
let h: f64 = 2.71828;,O'ta aniq o'nlik son (JS dagi Number)
let i: f32 = -0.5;,Manfiy o'nlik son

--------------------------------------------------------------------

Filter js
const numbers = [1, 2, 3, 4, 5, 6];
const evens = numbers.filter(n => n === 2);

Filter Rust
let numbers = vec![1, 2, 3, 2, 4];
let evens: Vec<i32> = numbers.into_iter()
    .filter(|&n| n == 2)
    .collect();

>>>>>>>>>>>>>>>>>>>>>>>>
struct ModelResult {
    name: String,
    acc: f32,
}

let results = vec![
    ModelResult { name: "Model A".into(), acc: 0.92 },
    ModelResult { name: "Model B".into(), acc: 0.75 },
];

let high_acc: Vec<ModelResult> = results.into_iter()
    .filter(|r| r.acc > 0.8)
    .collect();
------------------------------------------------------------

const models = [{id: 1, name: "CNN"}, {id: 2, name: "RNN"}];
const model = models.find(m => m.id === 2);

struct Model { id: u32, name: String }
let models = vec![
    Model { id: 1, name: "CNN".into() },
    Model { id: 2, name: "RNN".into() }
];
let model = models.iter().find(|m| m.id == 2);

------------------------------------------------------------

const score = 85;
if (score >= 90) {
  console.log("A'lo");
} else if (score >= 70) {
  console.log("Yaxshi");
} else {
  console.log("Yomon");
}


let score = 85;
if score >= 90 {
    println!("A'lo");
} else if score >= 70 {
    println!("Yaxshi");
} else {
    println!("Yomon");
}
------------------------------------------

const status = (age >= 18) ? "Kattalar" : "Bolalar";

let status = if age >= 18 { "Kattalar" } else { "Bolalar" };

-------------------------------------

for (let i = 0; i < 5; i++) {
  console.log(i);
}

const fruits = ["olma", "anor", "behi"];
for (const f of fruits) {
  console.log(f);
}


>>>>>>>>>>


for i in 0..5 {        // 0 dan 4 gacha (5 kirmaydi)
    println!("{}", i);
}
for i in 0..=5 {       // 0 dan 5 gacha (5 ham kiradi)
    println!("{}", i);
}
let fruits = vec!["olma", "anor", "behi"];
for f in &fruits {     // & belgisi massivni o'qish uchun (borrows)
    println!("{}", f);
}
-------------------------------------------------------------------

const numbers = [1, 2, 3];
numbers.forEach(n => {
    console.log(n);
});
fruits.forEach((fruit, i) => {
    console.log(`${i}: ${fruit}`);
});

let numbers = vec![1, 2, 3];
numbers.iter().for_each(|n| {
    println!("{}", n);
});

let fruits = vec!["olma", "anor"];
fruits.iter().enumerate().for_each(|(i, fruit)| {
    println!("{}: {}", i, fruit);
});
-----------------------------------------------------
let count = 0;
while (count < 5) {
    console.log(count);
    count++;
}
while (true) {
    if (shart) break;
    // ...
}


let mut count = 0;
while count < 5 {
    println!("{}", count);
    count += 1; // Rustda count++ yo'q, count += 1 ishlatiladi
}
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // Tsiklni to'xtatib, natijani qaytaradi
    }
};
println!("Natija: {}", result); // 20 chiqadi
--------------------------------------------------------------------

const nums = [1, 2, 3, 4];
const squared = nums.map(x => x * x);
// Natija: [1, 4, 9, 16]

const users = [{name: "Abdul", id: 1}, {name: "Ali", id: 2}];
const names = users.map(u => u.name);

let nums = vec![1, 2, 3, 4];
let squared: Vec<i32> = nums.iter()
    .map(|x| x * x)   // Har bir elementni o'zgartiradi
    .collect();       // Natijalarni yangi massivga yig'adi

struct User { name: String, id: u32 }
let users = vec![
    User { name: "Abdul".into(), id: 1 },
    User { name: "Ali".into(), id: 2 }
];

let names: Vec<String> = users.iter()
    .map(|u| u.name.clone())
    .collect();
----------------------------------------------

OOP
class Camera {
  constructor(ip, model) {
    this.ip = ip;
    this.model = model;
  }

  getInfo() {
    return `${this.model} (${this.ip})`;
  }
}
const cam = new Camera("192.168.1.10", "Hikvision");

>>>>>>>>>>

struct Camera {
    ip: String,
    model: String,
}
impl Camera {
    fn new(ip: &str, model: &str) -> Self {
        Self {
            ip: ip.to_string(),
            model: model.to_string(),
        }
    }
    // Metod (Self bu JS-dagi 'this')
    fn get_info(&self) -> String {
        format!("{} ({})", self.model, self.ip)
    }
}

let cam = Camera::new("192.168.1.10", "Hikvision");
------------------------------------------------------
interface Shape {
  area(): number;
}
class Circle implements Shape {
  constructor(public radius: number) {}
  area() {
    return Math.PI * this.radius ** 2;
  }
}

trait Shape {
  fn area(&self) -> f64; // Faqat metodning nomi va qaytarish turi
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
----------------------------------------------------------------

console.log(__dirname);

const path = require('path');
const fullPath = path.join(__dirname, "data", "weights.bin");

>>>>>>>>>>>>>>>
use std::env;
let path = env::current_dir().unwrap();
println!("Joriy papka: {:?}", path);


use std::path::PathBuf;
let mut path = PathBuf::from("."); // Joriy papka
path.push("data");
path.push("weights.bin");

println!("To'liq yo'l: {:?}", path);
--------------------------------------------------

fs.writeFileSync("test.txt", "Salom!");
const data = fs.readFileSync("test.txt", "utf8");
const buffer = fs.readFileSync("model.bin");
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

use std::fs;
fs::write("test.txt", "Salom Rustdan!").expect("Faylga yozib bo'lmadi");

let data = fs::read_to_string("test.txt").expect("Faylni o'qib bo'lmadi");
println!("Kontent: {}", data);

let bytes = fs::read("model.bin").expect("O'qib bo'lmadi"); 
// Natija: Vec<u8> (Node.js dagi Buffer ning aynan o'zi)
------------------------------------------------------------

const express = require('express');
const app = express();
app.post('/upload', (req, res) => {
    res.send('Fayl qabul qilindi');
});
app.listen(3000, () => console.log('Server running on 3000'));

>>>>>>>>>>>>>>>>>>>>>>>

use axum::{routing::post, Router};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    // Marshrutlarni sozlash (Route)
    let app = Router::new().route("/upload", post(upload_handler));

    // Serverni manzilga bog'lash
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server {} portda ishga tushdi", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn upload_handler() -> &'static str {
    "Fayl qabul qilindi"
}
----------------------------------------------------------------

const net = require('net');
const server = net.createServer((socket) => {
    console.log('Mijoz ulandi');
    socket.on('data', (data) => {
        console.log('Kelgan xabar:', data.toString());
        // HTTP protokoli qoidasi bo'yicha javob qaytaramiz (Raw Text)
        const response = 
            "HTTP/1.1 200 OK\r\n" +
            "Content-Type: text/plain\r\n" +
            "Content-Length: 12\r\n" +
            "\r\n" +
            "Salom, Node!";

        socket.write(response);
        socket.end(); // Ulanishni yopish
    });
});
server.listen(3000, '127.0.0.1', () => {
    console.log('TCP Server 3000-portda ishlamoqda...');
});
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
fn main() {
    // 1. Portni bog'lash (Bind)
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Port band bo'lishi mumkin");
    println!("Rust TCP Server 3000-portda ishlamoqda...");
    // 2. Har bir ulanishni qabul qilish
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream);
            }
            Err(e) => {
                println!("Xato: {}", e);
            }
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 1024]; // 1KB bufer
    // 3. Ma'lumotni o'qish
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            println!("Kelgan so'rov:\n{}", request);

            // 4. HTTP javobni qo'lda yasash
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nSalom, Rust";
            
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => println!("O'qishda xato: {}", e),
    }
}
-------------------------------------------------------------------
const { spawn } = require('child_process');
// Masalan, Python skriptini ishga tushirish
const pythonProcess = spawn('python', ['analyze_audio.py', '--file', 'voice.wav']);
pythonProcess.stdout.on('data', (data) => {
    console.log(`Python-dan kelgan natija: ${data}`);
});
pythonProcess.stderr.on('data', (data) => {
    console.error(`Xatolik: ${data}`);
});
pythonProcess.on('close', (code) => {
    console.log(`Jarayon yakunlandi, kod: ${code}`);
});

>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

use std::process::Command;
fn main() {
    // 'python' buyrug'ini ishga tushirish
    let output = Command::new("python")
        .arg("analyze_audio.py")
        .arg("--file")
        .arg("voice.wav")
        .output() // Bu buyruq tugashini kutadi va natijani qaytaradi
        .expect("Buyruqni ishga tushirib bo'lmadi");

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("Natija: {}", s);
    } else {
        let e = String::from_utf8_lossy(&output.stderr);
        println!("Xatolik yuz berdi: {}", e);
    }
}

use std::process::{Command, Stdio};
fn main() {
    // Jarayonni "spawn" qilamiz (ishga tushiramiz)
    let mut child = Command::new("python")
        .arg("script.py")
        .stdout(Stdio::piped()) // Chiqishni "quvur" orqali tutib olish
        .spawn()                // <--- Mana shu joyi spawn
        .expect("Dasturni ishga tushirib bo'lmadi");

    println!("Python ishga tushdi, Rust esa o'z ishida davom etmoqda...");

    // Kerak bo'lganda uni kutish yoki to'xtatish mumkin
    let status = child.wait().expect("Kutishda xato");
    println!("Python jarayoni tugadi: {}", status);
}
------------------------------------------------------------

try {
    const data = fs.readFileSync('audio.wav');
    // ...
} catch (err) {
    console.error("Xato yuz berdi:", err);
}


use std::fs::File;
fn main() {
    let result = File::open("audio.wav");

    let file = match result {
        Ok(file) => file,                // Agar hammasi joyida bo'lsa
        Err(error) => {                 // Bu qismi 'catch' ga teng
            println!("Faylni ochishda xato: {:?}", error);
            return;
        }
    };
}
---------------------------------------------------------------