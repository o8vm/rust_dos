use dos::*;
use rust_dos::*;

#[allow(dead_code)]
pub(crate) fn cooperative_multitasking_test() {
    add_cooperative_task!(task_2_main).unwrap();
    println!("Hello from main task!");
    yield_cooperative_task!();
    println!("Hello from main task! (bis)");
    yield_cooperative_task!();
    println!("Hello from main task! (tris)");
}

fn task_2_main() {
    add_cooperative_task!(task_3_main).unwrap();
    for _ in 0..2 {
        let task_number = 2;
        let task2_string = String::from("Hello from task 2!");
        println!("Message from task{}: {}", task_number, task2_string);
        yield_cooperative_task!();
    }
}

fn task_3_main() {
    for _ in 0..2 {
        let task_number = 3;
        let task2_string = String::from("Hello from task 3!");
        println!("Message from task{}: {}", task_number, task2_string);
        yield_cooperative_task!();
    }
}
