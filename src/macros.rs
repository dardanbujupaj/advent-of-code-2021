/// Macro for timing functions
///
/// Executes the expression and measures the time it takes to run.
///
/// # Examples
///
/// ```
/// use aoc::time;
///
/// // Prints '🚀 Print text: 42ns'
/// let result = time!("Print text", 1 + 2);
/// assert_eq!(result, 3);
///
/// ```
///
#[macro_export]
macro_rules! time {
    ( $n:literal, $x:expr ) => {{
        let start = std::time::Instant::now();
        let result = $x;
        let elapsed = start.elapsed();
        println!("🚀 {}: {:?}", $n, elapsed);
        result
    }};
}
