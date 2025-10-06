pub fn start() {
    binding_and_mutability();
    scope();
    shadowing();
}

fn binding_and_mutability() {
    {
        let x: i32 = 5; // Uninitialized but used, ERROR !
        let _y: i32; // Uninitialized but also unused, only a Warning !
    
        assert_eq!(x, 5);
        println!("Success!");
    }
    {
        let mut x = 1;
        x += 2;

        assert_eq!(x, 3);
        println!("Success!");
    }
}

fn scope() {
    {
        let x: i32 = 10;
        {
            let y: i32 = 5;
            println!("Inner scope value of x is {} and value of y is {}", x, y);
        }
        println!("Outer scope value of x is {} and value of y is out of scope", x);
    }
    {
        println!("{}, world", define_x());
    }
    fn define_x() -> &'static str {
        "hello"
    }
}

fn shadowing() {
    {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 5);
        }

        assert_eq!(x, 12);

        let x = 42;
        println!("{}", x); // Prints "42".
    }
}