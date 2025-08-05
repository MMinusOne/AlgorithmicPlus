use num_traits::{float::FloatCore, FromPrimitive, Num};

pub fn normalize_inline<T: Num + FromPrimitive + Copy + FloatCore>(data: &mut [T]) {
    let (min, max) = data.iter().cloned().fold(
        (
            T::from_i32(i32::MAX).unwrap(),
            T::from_i32(i32::MIN).unwrap(),
        ),
        |(min, max), value| (value.min(min), value.max(max)),
    );

    if min == max {
        for value in data.iter_mut() {
            *value = T::zero();
        }
    } else {
        for value in data.iter_mut() {
            *value = (*value - min) / (max - min);
        }
    }
}
