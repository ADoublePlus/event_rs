use event_struct::*;

fn main() -> Result<(), std::io::Error>
{
    let mut event_loop = EventSystem::new();
    let event_f = EventFunction::from_all(
        Box::new(|_e: &mut EventSystem| -> Result<(), ()> 
        {
            println!("Hello!");
            Ok(())
        }),
        Box::new(|_e: &mut EventSystem| 
        {
            println!("Success!");
        }),
        Box::new(|_e: &mut EventSystem| 
        {
            println!("Fail!");
            e.emit("hello");
        }),
    );
    event_loop.register_event("hello", event_f);
    event_loop.emit("hello");
    event_loop.run();
    Ok(())
}