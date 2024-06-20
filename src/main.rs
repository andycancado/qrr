use qrcode::render::unicode;
use qrcode::{EcLevel, QrCode, Version};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text to encode>", args[0]);
        std::process::exit(1);
    }
    let text = &args[1];

    let code = QrCode::with_version(text, Version::Normal(1), EcLevel::L)
        .expect("Failed to generate QR Code");

    let image = code
        .render::<unicode::Dense1x2>()
        .quiet_zone(true)
        .module_dimensions(2, 2)
        .build();

    println!("{}", image);
}
