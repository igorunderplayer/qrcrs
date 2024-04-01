use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    text: String,

    #[arg(long, default_value_t = false)]
    image: bool,

    #[arg(long, short, default_value_t = String::from("./qrcrs-output.png"))]
    output: String,
}

fn main() {
    let args = Args::parse();
    let text = args.text;

    if args.image {
        run_image_mode(text, args.output)
    } else {
        text_mode(text)
    }
}

fn run_image_mode(text: String, output: String) {
    qrcode_generator::to_png_to_file(text, qrcode_generator::QrCodeEcc::Low, 1024, output).unwrap()
}

fn text_mode(text: String) {
    let result: Vec<Vec<bool>> =
        qrcode_generator::to_matrix(text, qrcode_generator::QrCodeEcc::Low).unwrap();

    for row in result {
        print!("\x1b[47m"); // Set output text background to black
        for item in row {
            if item {
                print!("\x1b[30m"); // Set output text color to black
                print!("██")
            } else {
                print!("\x1b[37m"); // Set output text color to white
                print!("██")
            }
        }

        println!()
    }

    print!("\x1b[0m"); // Reset text color to terminal default
}
