use std::run::process_status;
use std::os;
use std::rand;

fn randInt()
{
   let rng = rand::task_rng();
   let n: uint = rng.gen_integer_range(0u, 10);
   printfln!(n);
}
fn runCode(v: ~str)
{
        let q: ~[&str] = v.split_iter('.').collect();
        let mut opt: ~[~str] = ~[v, ~"-o" + q[0] , ~"-lstdc++"];
        if(q[1] == "cpp")
        {
           opt.push(~"-lstdc++");
        }
        process_status("gcc" , opt);
        process_status("./" + q[0], []);
        process_status("rm", [q[0].to_owned()] ); 
  

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
        randInt();      
    }

    //   runCode(source);  
    // / process_status("./test", []);
    
}
