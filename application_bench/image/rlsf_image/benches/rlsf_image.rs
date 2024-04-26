use criterion::{criterion_group, criterion_main, Criterion};
use photon_rs::filters;
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::{resize, SamplingFilter};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invert_image", |b| b.iter(invert_image));
    c.bench_function("resize_jpg", |b| b.iter(resize_jpg));
    c.bench_function("apply_filter", |b| b.iter(apply_filter));
}

fn invert_image() {
    let mut img = open_image("input_imgs/test_image_2.jpg").expect("File should open");
    photon_rs::channels::invert(&mut img);
    let output_img_path = "output.jpg";
    save_image(img, output_img_path).unwrap();
}

fn resize_jpg() {
    let img = open_image("input_imgs/test_image_2.jpg").expect("File should open");
    let resized_img = resize(&img, 800, 600, SamplingFilter::Lanczos3);
    let output_img_path = "output.jpg";
    save_image(resized_img, output_img_path).unwrap();
}

fn apply_filter() {
    let mut img = open_image("input_imgs/test_image_2.jpg").expect("File should open");
    filters::filter(&mut img, "twenties");
    let output_img_path = "output.jpg";
    save_image(img, output_img_path).expect("File should be saved");
}

fn alter_sample_size() -> Criterion {
    Criterion::default()
        .sample_size(10_usize)
        .measurement_time(Duration::from_secs(10_u64))
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
