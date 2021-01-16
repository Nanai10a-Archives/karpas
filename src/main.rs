use karpas::boot::*;

fn main() {
    KarpasBuilder
        .build()
        .unwrap()
        .boot()
        .unwrap()
}
