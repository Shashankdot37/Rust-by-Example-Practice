//struct visibility


mod module{
    pub struct PublicContent<T>{
        pub content:T
    }
    
    pub struct PrivateContent<T>{
        content:T
    }
    //public constructor for PrivateContent
    
    impl<T: std::fmt::Debug> PrivateContent<T>{
        pub fn new(content:T)->PrivateContent<T>{
            PrivateContent{
                content: content
            }
        }
        
        pub fn print_content(&self){
            println!("{:?}", self.content)
        }
    }
}

fn main(){
    let data1 = module::PublicContent{content:"data1"};
    println!("{:?}",data1.content);
    
    let data2 = module::PrivateContent::new("data2");
    data2.print_content();
}

