use rust_cloud::compute::{ResourceLimit, ResourceRequest};

fn main() {
    let request = ResourceRequest::new(2, 2048).expect("recursos válidos");
    let generous_limit = ResourceLimit::new(4, 4096).expect("límite válido");
    let small_limit = ResourceLimit::new(1, 4096).expect("límite válido");

    assert!(request.fits_within(generous_limit));
    assert!(!request.fits_within(small_limit));

    println!("La carga cabe en el límite generoso y no cabe en el límite pequeño.");
}
