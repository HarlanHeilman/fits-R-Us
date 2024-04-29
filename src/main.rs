use dirs;
use astrors::fits;



fn main() {
    let mut testfile = dirs::home_dir().unwrap();
    testfile.push("fits\\src\\ZnPc82862-00001.fits");
    let hdu_list = fits::fromfile(testfile.to_str().unwrap()).unwrap();

    println!("{:?}", hdu_list.hdus);

    match hdu_list.hdus[0] {
        astrors::io::hdulist::HDU::Primary(astrors::io::hdus::primaryhdu::PrimaryHDU {header, data}) => {
            print!("Primary HDU\n");
        },
        _ => {}
    }
}
