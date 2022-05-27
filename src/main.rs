fn get_topmost_int(stack: &mut Vec<u8>, stack_size: &mut usize) -> u32 {
    let base_stack = *stack_size - 4;
    *stack_size = base_stack;
    return ((stack[base_stack] as u32) * 0x1000000 + (stack[base_stack + 1] as u32) * 0x10000 +
            (stack[base_stack + 2] as u32) *0x100 + (stack[base_stack + 3] as u32)).into();
}

fn push_int(stack: &mut Vec<u8>, data_to_push: u32, stack_size: &mut usize) {
    let data_bytes = data_to_push.to_be_bytes();
    let start = *stack_size;

    for i in 0..data_bytes.len() {
        stack[i + start] = data_bytes[i];
    }
    *stack_size += data_bytes.len();
}

fn bytecode_bipush(stack: &mut Vec<u8>, data_to_push: u8, stack_size: &mut usize) {
    push_int(stack, data_to_push.into(), stack_size);
}

fn bytecode_iadd(stack: &mut Vec<u8>, stack_size: &mut usize) -> u32 {
    let sum = get_topmost_int(stack, stack_size) + get_topmost_int(stack, stack_size);
    push_int(stack, sum, stack_size);
    return sum;
}

pub(crate) fn main() {
    let bytecode: Vec<u8> = vec![1, 2, 3];
    let mut stack: Vec<u8> = vec![0; 64];
    let mut stack_size: usize = 0;

    println!("{}", bytecode[0]);

    bytecode_bipush(&mut stack, 255, &mut stack_size);
    bytecode_bipush(&mut stack, 255, &mut stack_size);
    bytecode_bipush(&mut stack, 2, &mut stack_size);

    println!("[0] {} [1] {} [2] {} [3] {}", stack[0], stack[1], stack[2], stack[3]);
    bytecode_iadd(&mut stack, &mut stack_size);
    println!("[0] {} [1] {} [2] {} [3] {}", stack[0], stack[1], stack[2], stack[3]);
    let sum = bytecode_iadd(&mut stack, &mut stack_size);
    println!("[0] {} [1] {} [2] {} [3] {}", stack[0], stack[1], stack[2], stack[3]);
    println!("soma {}", sum);
    println!("Hello, world!");
}
