use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let secret_key = String::from("bgvyzdsv");

    let mut counter = 1;
    loop {
        let teststr = secret_key.clone() + &counter.to_string();
        counter += 1;

        let mut hash = Md5::new();
        hash.input_str(&teststr);

        if hash.result_str().starts_with("000000") {
            println!("{}", teststr);
            break;
        }
    }
}
