use uuid::Uuid;


fn main() {
    // Create v4 uuids with open source rust uuid lib

    loop {
    let v4 = Uuid::new_v4();

    let v4_str = v4.to_string();

    println!("{}", v4_str);

    }
}

