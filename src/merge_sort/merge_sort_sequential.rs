pub fn merge_sort_sequential<T>(arr: &mut [T])
    where T: Eq + PartialOrd + Copy
{
    let mid_point = arr.len() / 2;

    if mid_point == 0 {
        return;
    }

    let (left, right) = arr.split_at_mut(mid_point);
    merge_sort_sequential(left);
    merge_sort_sequential(right);
    let mut temp_vec = Vec::with_capacity(left.len() + right.len());

    merge(left, right, &mut temp_vec);
    arr.copy_from_slice(&temp_vec);
}

fn merge<T>(left: &mut [T], right: &mut [T], temp_vec: &mut Vec<T>) 
    where T: Eq + PartialOrd + Copy
{   
    let mut left_pointer = 0;
    let mut right_pointer = 0;

    while left_pointer < left.len() && right_pointer < right.len() {
        if left[left_pointer] <= right[right_pointer] {
            temp_vec.push(left[left_pointer]);
            left_pointer += 1;
        } else {
            temp_vec.push(right[right_pointer]);
            right_pointer += 1;
        }
    }

    while left_pointer < left.len() {
        temp_vec.push(left[left_pointer]);
        left_pointer += 1;
    }

    while right_pointer < right.len() {
        temp_vec.push(right[right_pointer]);
        right_pointer += 1;
    }
}