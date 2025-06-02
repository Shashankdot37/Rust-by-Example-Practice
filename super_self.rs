fn function() {
    println!("called function()");
}

mod chill {
    pub fn function() {
        println!("called chill::function()");
    }
}

mod module {
    fn function() {
        println!("called module::function()");
    }
    mod chill {
        pub fn function() {
            println!("called module::chill::function()");
        }
    }

    pub fn indirect_access() {
        println!("Indirect access call that \n");
        self::function();
        function(); //both prints the same thing.

        self::chill::function();
        super::function();

        {
            use crate::chill::function as root_function;
            root_function();
        }
    }
}

fn main() {
    module::indirect_access();
}
