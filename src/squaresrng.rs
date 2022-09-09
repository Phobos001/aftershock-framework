#[derive(Debug, Copy, Clone)]
pub struct SquaresRNG {
    pub key: u64,
    pub counter: u64,
}

impl SquaresRNG {
    const U64_REMAINDER: u64 = u32::MAX as u64;
    const TEST_COUNT: u64 = 10_000_000;
    const TEST_KEY: u64 = 0x2467cb532b5ce8d1;

    /// Requires a key to be picked for number generation. A unique combo of counter and key will provide the same number each time.
    /// A single key can generate 2^64 random numbers
    /// I suggest picking a starting counter position by using the walltime of the OS, or the realtime of the program
    /// Or you can just pick a number and count up, which will provide the same set of random numbers each time
    #[cfg(feature = "keys_table")]
    pub fn new(counter: u64, key: u64) -> SquaresRNG {
        SquaresRNG {
            key: KEYS_TABLE[key % KEYS_TABLE.len()],
            counter,
        }
    }

    /// New random generator with a user-provided key. Does not guarantee the key will work well for number generation. The key must
    /// have roughly equal counts 0 bits and 1 bits (32/32) in any order to be a valid key. Evenly distributed bits provide the best consistancy
    /// as the RNG rotates data.
    /// 
    /// It is highly recommended to use a key from the keys.rs file provided in the source. If you really want to include the entire table (really unnessecary)
    /// you can do so with the feature "keys_table"
    pub fn new_with_key(counter: u64, key: u64) -> SquaresRNG {
        SquaresRNG { key, counter }
    }

    /// Returns a random 64-bit unsigned integer
    /// The output is uniform and will eventually produce all 2^64 numbers with the key.
    //#[inline]
    fn rand_u64(&mut self) -> u64 {
        let mut x: u64 = u64::wrapping_mul(self.counter, self.key);
        let y: u64 = x;
        let z: u64 = u64::wrapping_add(y, self.key);

        x = u64::wrapping_add(u64::wrapping_mul(x,x), y); x = (x >> 32) | (x << 32);
        x = u64::wrapping_add(u64::wrapping_mul(x,x), z); x = (x >> 32) | (x << 32);
        x = u64::wrapping_add(u64::wrapping_mul(x,x), y); x = (x >> 32) | (x << 32);

        self.counter += 1;
        
        return (u64::wrapping_add(u64::wrapping_mul(x,x), z) >> 32) as u64;
    }

    /// Returns a random usize within a range. For picking random elements in an array or Vec.
    //#[inline]
    pub fn rand_index(&mut self, size: u64) -> usize {
        let rand: u64 = self.rand_u64() % SquaresRNG::U64_REMAINDER;
        (rand % size) as usize
    }

    /// Returns a random f32 between 0 and 1
    //#[inline]
    pub fn randf32(&mut self) -> f32 {
        // Since self.rand() will count up through each unique u64 number, we need to do something to 
        // make number generation more uniform throughout its lifetime
        // We divide the u64 into u32::MAX chunks, where we then see get the 'percentage' of its position in that chunk
        // This is consistantly random enough without being too hard on performance.

        // Splitting rand
        let rand: u64 = self.rand_u64() % SquaresRNG::U64_REMAINDER;
        // Getting place in split
        return rand as f32 / SquaresRNG::U64_REMAINDER as f32;
    }

    /// Returns a random f32 between min and max.
    //#[inline]
    pub fn rangef32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.randf32()
    }

    /// Returns a random i32 between min and max.
    //#[inline]
    pub fn rangei32(&mut self, min: i32, max: i32) -> i32 {
        self.rangef32(min as f32, max as f32).round() as i32
    }

    /// Returns a tuple set of two f32's
    //#[inline]
    pub fn vec2f32(&mut self) -> (f32, f32) {
        (self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0))
    }

    /// Returns a tuple set of three f32's
    //#[inline]
    pub fn vec3f32(&mut self) -> (f32, f32, f32) {
        (self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0))
    }

    /// Returns a tuple set of four f32's
    //#[inline]
    pub fn vec4f32(&mut self) -> (f32, f32, f32, f32) {
        (self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0), self.rangef32(-1.0, 1.0))
    }
    
    /// Returns a random f64 between 0 and 1
    //#[inline]
    pub fn randf64(&mut self) -> f64 {
        // Since self.rand() will count up through each unique u64 number, we need to do something to 
        // make number generation more uniform throughout its lifetime
        // We divide the u64 into u32::MAX chunks, where we then see get the 'percentage' of its position in that chunk
        // This is consistantly random enough without being too hard on performance.

        // Splitting rand
        let rand: u64 = self.rand_u64() % SquaresRNG::U64_REMAINDER;
        // Getting place in split
        return rand as f64 / SquaresRNG::U64_REMAINDER as f64;
    }

    /// Returns a random f64 between min and max.
    //#[inline]
    pub fn rangef64(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.randf64()
    }

    /// Returns a random i64 between min and max.
    //#[inline]
    pub fn rangei64(&mut self, min: i64, max: i64) -> i64 {
        self.rangef64(min as f64, max as f64).round() as i64
    }

    /// Returns a tuple set of two f64's
    //#[inline]
    pub fn vec2f64(&mut self) -> (f64, f64) {
        (self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0))
    }

    /// Returns a tuple set of three f64's
    //#[inline]
    pub fn vec3f64(&mut self) -> (f64, f64, f64) {
        (self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0))
    }

    /// Returns a tuple set of four f64's
    //#[inline]
    pub fn vec4f64(&mut self) -> (f64, f64, f64, f64) {
        (self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0), self.rangef64(-1.0, 1.0))
    }
}

#[test]
pub fn test_f32_uniformality() {
    
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: f32 = 0.0;

    for _ in 0..count {
        average += rng.randf32();
    }
    
    let result: f32 = average / count as f32;
    println!("\nRNG f32 Uniformality (Closer to 0.5): {}\n", result);
    assert!(result < 0.51 && result > 0.49);
}

#[test]
pub fn test_vec2f32_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f32, f32) = (0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec2f32();
        average.0 += rnd.0;
        average.1 += rnd.1;
    }

    average.0 /= count as f32;
    average.1 /= count as f32;
    
    println!("\nRNG vec2f32 Uniformality (Closer to 0.0): x: {}, y: {}\n", average.0, average.1);

    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
}

#[test]
pub fn test_vec3f32_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f32, f32, f32) = (0.0, 0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec3f32();
        average.0 += rnd.0;
        average.1 += rnd.1;
        average.2 += rnd.2;
    }

    average.0 /= count as f32;
    average.1 /= count as f32;
    average.2 /= count as f32;
    
    println!("\nRNG vec3f32 Uniformality (Closer to 0.0): x: {}, y: {}, z: {}\n", average.0, average.1, average.2);
    
    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
    assert!(average.2 < 0.01 && average.2 > -0.01);
}

#[test]
pub fn test_vec4f32_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec4f32();
        average.0 += rnd.0;
        average.1 += rnd.1;
        average.2 += rnd.2;
        average.3 += rnd.3;
    }

    average.0 /= count as f32;
    average.1 /= count as f32;
    average.2 /= count as f32;
    average.3 /= count as f32;
    
    println!("\nRNG vec4f32 Uniformality (Closer to 0.0): w: {}, x: {}, y: {}, z: {}\n", average.3, average.0, average.1, average.2);

    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
    assert!(average.2 < 0.01 && average.2 > -0.01);
    assert!(average.3 < 0.01 && average.3 > -0.01);
}

#[test]
pub fn test_f64_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: f64 = 0.0;

    for _ in 0..count {
        average += rng.randf64();
    }
    
    let result: f64 = average / count as f64;
    println!("\nRNG f64 Uniformality (Closer to 0.5): {}\n", result);
    assert!(result < 0.51 && result > 0.49);
}

#[test]
pub fn test_vec2f64_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f64, f64) = (0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec2f64();
        average.0 += rnd.0;
        average.1 += rnd.1;
    }

    average.0 /= count as f64;
    average.1 /= count as f64;
    
    println!("\nRNG vec2f64 Uniformality (Closer to 0.0): x: {}, y: {}\n", average.0, average.1);

    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
}

#[test]
pub fn test_vec3f64_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f64, f64, f64) = (0.0, 0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec3f64();
        average.0 += rnd.0;
        average.1 += rnd.1;
        average.2 += rnd.2;
    }

    average.0 /= count as f64;
    average.1 /= count as f64;
    average.2 /= count as f64;
    
    println!("\nRNG vec3f64 Uniformality (Closer to 0.0): x: {}, y: {}, z: {}\n", average.0, average.1, average.2);
    
    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
    assert!(average.2 < 0.01 && average.2 > -0.01);
}

#[test]
pub fn test_vec4f64_uniformality() {
    let mut rng: SquaresRNG = SquaresRNG::new_with_key(0, SquaresRNG::TEST_KEY);

    let count: u64 = SquaresRNG::TEST_COUNT;
    let mut average: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

    for _ in 0..count {
        let rnd = rng.vec4f64();
        average.0 += rnd.0;
        average.1 += rnd.1;
        average.2 += rnd.2;
        average.3 += rnd.3;
    }

    average.0 /= count as f64;
    average.1 /= count as f64;
    average.2 /= count as f64;
    average.3 /= count as f64;
    
    println!("\nRNG vec4f64 Uniformality (Closer to 0.0): w: {}, x: {}, y: {}, z: {}\n", average.3, average.0, average.1, average.2);

    assert!(average.0 < 0.01 && average.0 > -0.01);
    assert!(average.1 < 0.01 && average.1 > -0.01);
    assert!(average.2 < 0.01 && average.2 > -0.01);
    assert!(average.3 < 0.01 && average.3 > -0.01);
}