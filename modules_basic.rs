// modules -- visibility

mod module1{
    pub fn public_func(){
        println!("Public fucntion in module1");
    }
    
    fn private_func(){
        println!("Private function in module1");
    }
    
    //Item inside same module can access other module
    pub fn indirect_access(){
        println!("Public function in module1 that can access \n");
        private_func();
    }
    
    //Nested modules
    
    pub mod nested{
        pub fn public_nested(){
            println!("Public nested function in nested module.");
        }
        
        #[allow(dead_code)]
        fn private_nested(){
            println!("Private function in nested module.");
        }
        
        pub(in crate::module1) fn public_func_in_module1(){
            println!("Public function in module1 that called \n");
        }
        
        //self --> only available within the current module. so same as leaving private
        pub(self) fn public_func_in_nested(){
            println!("Public function in nested module.");
        }
        
        //visible within the parent module
        pub(super) fn public_func_super(){
            println!("Function with pub(super) and is visible within the parent module.")
        }
    }
    
    //if not declared pub while defining module, it'll be private by default. follows the same rule as other item. 
    
    //pub(crate) makes the function available in the current crate
    
    pub(crate) fn public_func_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }
    
}

fn main(){
    module1::public_func();
    module1::indirect_access();
    module1::nested::public_nested();
}
