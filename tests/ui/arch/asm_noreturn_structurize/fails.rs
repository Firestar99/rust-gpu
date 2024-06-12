// build-pass

use spirv_std::arch::IndexUnchecked;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffer: &mut [u32],
    #[spirv(push_constant)] non_const: &u32,
) {
    if *non_const == 0 {
        // observable side-effect
        let reference = unsafe { buffer.index_unchecked_mut(0) };
        *reference = 42;
    }

    unsafe {
        spirv_std::arch::kill();
    }
}
