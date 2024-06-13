pub fn start_menu(first_time: bool) {
    println!("###############################################");
    println!("#                                             #");
    if first_time {
        println!("#           Welcome to cmd-manager!           #");
        println!("#                                             #");
        println!("###############################################");
        println!("#  You'll be able to create, remove and list  #");
        println!("#         and mark as completed! GTD!         #");
    } else {
        println!("#       cmd-manager  -  Your TODO list!       #");
        println!("#                                             #");
    }
    println!("###############################################");
    println!("#                                             #");
    println!("#  Select an operation:                       #");
    println!("#                                             #");
    println!("#     '1' -> New Task                         #");
    println!("#     '2' -> Remove Task                      #");
    println!("#     '3' -> Show Tasks                       #");
    println!("#     '4' -> Complete a Task                  #");
    println!("#                                             #");
    println!("#     'Q' -> Exit                             #");
    println!("###############################################");
}
