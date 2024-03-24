use std::fmt; // Import `fmt`

// A structure holding two numnbers. `Debug` will be derived so the results
// constrasted with `Display`
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 