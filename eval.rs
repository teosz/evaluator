use std::run::process_status;
use std::os;
fn runCode(v: ~str)
{
         println!("{:?}" , v);

        let q: ~[&str] = v.split_iter('.').collect();
    
        println!("{:?}" , q[0]);
        let mut opt: &[~str] = [v, "-o" + v , "-lstdc++"];
        process_status("gcc" , [v, ~"-lstdc++"]);
  
}
fn main()
{


    let mut args = os::args();
    if (args.len() < 4)
    {
        println("Invalid commands. Use --help");
    } 
    
    else 
    { 
        args.shift();
        let source = args.shift(); 
        let bsource = args.shift();
        let iofile = args.shift();
        runCode(source) ;
        
    }

    //   runCode(source);  
    // / process_status("./test", []);
    
}
