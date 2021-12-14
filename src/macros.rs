#[macro_export]
macro_rules! timed {
    ( $n:literal, $x:expr ) => {{
        let start = std::time::Instant::now();
        let result = $x;
        let elapsed = start.elapsed();
        println!("🚀 {}: {:?}", $n, elapsed);
        result
    }};
}
