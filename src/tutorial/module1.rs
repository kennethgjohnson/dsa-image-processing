// Array Module1 - Memory Access Patterns
use std::time::Instant; // For capturing timing information.

// Helper functions will go here as we build them
// Simple Person struct for row-based layout
struct Person {
    name: &'static str,
    age: u8,
    dob: (u32, u8, u8), // (year, month, day)
}

pub fn arrays_module1_memory_access_patterns() {
    println!("Image Processing DSA - Module 1: Memory & Access Patterns");
    // We'll build our exercises here
    // Starting with basic array allocation and address printing
    let _x = 987;
    let mut int_arr = [1, 2, 3, 4, 5];
    let ptr = int_arr.as_ptr();
    let ptr_mut = int_arr.as_mut_ptr();

    unsafe {
        let ptr2 = ptr.offset(2);
        println!("The value at 0 is {}", *ptr);
        println!("The value at 0+2 is {}", *ptr2);

        println!("We modify the value at 0 to 500->");
        *ptr_mut = 500;
        println!("The value at 0 is {}", *ptr);
        println!("ptr is at {:p}", ptr);
        let ptr1 = ptr.add(1);
        println!("ptr1 is at {:p}", ptr1);
        let ptr2 = ptr1.add(1);
        println!("ptr2 is at {:p}", ptr2);
        let ptr3 = ptr2.add(1);
        println!("ptr3 is at {:p}", ptr3);
        let ptr4 = ptr3.add(1);
        println!("ptr4 is at {:p}", ptr4);

        let ptr3 = ptr4.sub(1);
        println!("ptr3 is at {:p}", ptr3);
        let ptr2 = ptr3.sub(1);
        println!("ptr2 is at {:p}", ptr2);
        let ptr1 = ptr2.sub(1);
        println!("ptr1 is at {:p}", ptr1);
        let ptr0 = ptr1.sub(1);
        println!("ptr0 is at {:p}", ptr0);

        println!(
            "The index difference between ptr2 and ptr4 is {}",
            ptr_index_diff(ptr2, ptr4)
        );
        // It is unsafe it will let you hang yourself, even try accessing memory
        // you shouldn't but program will panic if you read outside of the
        // program's memory bounds.
        let ptr_negative_1 = ptr0.sub(1);
        println!("ptr_negative_1 is at {:p}", ptr_negative_1);
        println!("value at ptr_negative_1 = {}", *ptr_negative_1);
        let arr_element_0_address = ptr0 as usize;
        println!("arr_element_0_address {:x}", arr_element_0_address);
        println!(
            "so i32 is 4 bytes, we know the 3rd element in the array is a '4', lets move the address to where it should be..."
        );
        let arr_element_3_address = arr_element_0_address + (4 * 3); // we are move 3 elements at 4 bytes per element.
        println!(
            "So our 4th element's address is at {:x}, lets turn this address into a pointer to a i32",
            arr_element_3_address
        );
        let ptr4: *mut i32 = arr_element_3_address as *mut i32;
        println!("So we now have our pointer at the 4th element {:p}", ptr4);
        println!("And it's values is {}", *ptr4);
        println!("And now lets change it to 999.");
        *ptr4 = 999;
    }
    println!("we are outside the safe block now, lets check element index 3's value!");
    println!("the value is {}", int_arr[3]);

    println!(
        "Finally we will attempt to use our element_at function that uses pointer arithmatic."
    );
    if let Some(value) = element_at(&int_arr, 2) {
        println!("We found the value at 2: {}", value)
    } else {
        println!("We could not find the value at 2!!! :(");
    }
    if let Some(value) = element_at(&int_arr, 200) {
        println!("We found the value at 200: {}", value)
    } else {
        println!("We could not find the value at 200!!! :)");
    }

    println!("---------------------");
    println!();
    println!("We will now have two arrays, one with row layout, and one with column layout.");
    println!("We will apply two exercises to each of the arrays, ");
    println!("one searching for ages that end in 3 where the name ends in 'a'.");
    println!("one that sums the total of ages for all.");

    // -------- ROW-BASED LAYOUT --------
    let row_people = [
        Person {
            name: "Anna",
            age: 23,
            dob: (1998, 5, 12),
        },
        Person {
            name: "Leo",
            age: 30,
            dob: (1993, 7, 8),
        },
        Person {
            name: "Maria",
            age: 43,
            dob: (1981, 11, 23),
        },
        Person {
            name: "Nina",
            age: 33,
            dob: (1991, 2, 14),
        },
        Person {
            name: "Omar",
            age: 25,
            dob: (1999, 9, 3),
        },
    ];

    // -- warming up --
    // Example access
    println!(
        "Example row access: {} was born on {:?}\n",
        row_people[0].name, row_people[0].dob
    );
    let _record = format!(
        "Name: {}, Age: {}, DOB: {:04}-{:02}-{:02}",
        row_people[0].name,
        row_people[0].age,
        row_people[0].dob.0,
        row_people[0].dob.1,
        row_people[0].dob.2
    );
    println!("Example format row: {:?}\n", _record);
    let _start = Instant::now();
    // -- warm up done --
    // Exercise 1: search
    let start = Instant::now();
    for p in &row_people {
        if p.age % 10 == 3 && p.name.ends_with('a') {
            println!("Row match: {} (age {})", p.name, p.age);
        }
    }
    let row_search_time = start.elapsed();

    // Exercise 2: sum
    let start = Instant::now();
    let row_sum: u32 = row_people.iter().map(|p| p.age as u32).sum();
    let row_sum_time = start.elapsed();
    println!("Row sum of ages: {}\n", row_sum);

    // Exercise 3: full-record formatting
    let start = Instant::now();
    for p in &row_people {
        let _record = format!(
            "Name: {}, Age: {}, DOB: {:04}-{:02}-{:02}",
            p.name, p.age, p.dob.0, p.dob.1, p.dob.2
        );
        println!("{}", _record)
    }
    let row_format_time = start.elapsed();
    println!("Row full-record format time: {:?}\n", row_format_time);

    // -------- COLUMN-BASED LAYOUT --------
    // Group columns into a tuple
    let person_data = (
        ["Anna", "Leo", "Maria", "Nina", "Omar"],
        [23, 30, 43, 33, 25],
        [
            (1998, 5, 12),
            (1993, 7, 8),
            (1981, 11, 23),
            (1991, 2, 14),
            (1999, 9, 3),
        ],
    );
    let (col_names, col_ages, col_dobs) = &person_data;

    // Exercise 1: search
    let start = Instant::now();
    for ((name, &age), _) in col_names.iter().zip(col_ages).zip(col_dobs) {
        if age % 10 == 3 && name.ends_with('a') {
            println!("Column match: {} (age {})", name, age);
        }
    }
    let col_search_time = start.elapsed();

    // Exercise 2: sum
    let start = Instant::now();
    let col_sum: u32 = col_ages.iter().map(|&a| a as u32).sum();
    let col_sum_time = start.elapsed();
    println!("Column sum of ages: {}\n", col_sum);

    // Exercise 3: full-record formatting (column-based)
    let start = Instant::now();
    for i in 0..col_names.len() {
        let _record = format!(
            "Name: {}, Age: {}, DOB: {:04}-{:02}-{:02}",
            col_names[i], col_ages[i], col_dobs[i].0, col_dobs[i].1, col_dobs[i].2
        );
        println!("{}", _record);
    }
    let col_format_time = start.elapsed();
    println!("Column full-record format time: {:?}\n", col_format_time);

    // -------- TIMING RESULTS --------
    println!("Row search time:           {:?}", row_search_time);
    println!("Row sum time:              {:?}", row_sum_time);
    println!("Row format time:           {:?}", row_format_time);
    println!("Column search time:        {:?}", col_search_time);
    println!("Column sum time:           {:?}", col_sum_time);
    println!("Column format time:        {:?}", col_format_time);
}

fn element_at<T>(arr: &[T], index: usize) -> Option<&T> {
    let mut result: Option<&T> = None;
    if !arr.is_empty() && index < arr.len() {
        unsafe {
            let root_pointer_address = arr.as_ptr() as usize;
            let target_item_pointer_address =
                root_pointer_address + index * std::mem::size_of::<T>();
            let target_pointer: *const T = target_item_pointer_address as *const T;
            let reference = &*target_pointer;
            result = Some(reference)
        }
    }
    result
}

fn ptr_index_diff<T>(ptr1: *const T, ptr2: *const T) -> usize {
    // Solution 1
    // let address1 = ptr1 as usize;
    // let address2 = ptr2 as usize;
    // if address1 > address2 {
    //     (address1 - address2) / std::mem::size_of::<T>()
    // } else {
    //     (address2 - address1) / std::mem::size_of::<T>()
    // }

    // Solution 2
    unsafe { ptr2.offset_from(ptr1).abs() as usize }
}
