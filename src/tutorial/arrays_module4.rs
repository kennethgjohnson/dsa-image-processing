mod cyclical_rotation;
mod move_zeros_to_end;
mod three_way_partition_dutch_national_flag;
mod two_pointers_technique;

pub fn arrays_module4_core_array_algorithms() {
    // We will be going over 3 fundamental paterns and perhaps a variant or two,
    // including benchmarking information where appropriate, for some in-place
    // swapping algorithms/techniques.

    // 1 - Two-Pointer Technique - Array reversal
    two_pointers_technique::arrays_module4_core_array_algorithms_two_pointer_technique();

    // 2 - Cyclic Rotation - basically a shift left/right with overflow wrap around.
    cyclical_rotation::arrays_module4_core_array_algorithms_cyclical_rotation();

    // 3. Move Zeros to End
    move_zeros_to_end::arrays_module4_core_array_algorithms_move_zeros_to_end();

    // 4. Dutch National Flag (Three-Way Partitioning)
    three_way_partition_dutch_national_flag::arrays_module4_core_array_algorithms_dutch_national_flag_three_way_partitioning();
}
