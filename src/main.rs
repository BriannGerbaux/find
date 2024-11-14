use std::io::Error;

mod arguments;
use crate::arguments::Arguments;

mod find;
use crate::find::Find;

fn main() -> Result<(), Error>{
    let mut args: Arguments = Arguments::new();
    
    args.parse();

    let mut find: Find = Find::new(&args);

    find.exec().inspect_err(|e| eprintln!("find terminated with: {}", e))?;
    find.display_found_entries();
    Ok(())
}
