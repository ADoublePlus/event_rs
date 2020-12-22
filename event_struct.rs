pub use crate::event_rs::function_struct::EventFunction;

pub struct EventSystem
{
    func_map: std::collections::HashMap<String, Vec<EventFunction>>,
    incoming_events: Vec<String>
}

impl EventSystem
{
    pub fn new() -> EventSystem
    {
        EventSystem
        {
            func_map: std::collections::HashMap::new(),
            incoming_events: vec![]
        }
    }

    pub fn execute(&mut self, func: &mut EventFunction)
    {
        let result = (func.execute_function)(self);

        match result
        {
            Ok(()) => (func.success_function)(self),
            Err(()) => (func.fail_function)(self)
        }
    }

    pub fn emit(&mut self, event_name: &str)
    {
        self.incoming_events.push(String::from(event_name));
    }

    pub fn run(&mut self)
    {
        loop 
        {
            if self.incoming_events.is_empty()
            {
                continue;
            }

            let event_name = self.incoming_events.pop().unwrap();

            if !self.func_map.contains_key(&event_name)
            {
                continue;
            }

            let mut v = self.func_map.remove_entry(&event_name).unwrap();

            for func in v.1.iter_mut()
            {
                match self.execute(func)
                {
                    _ => {}
                }
            }

            self.func_map.insert(v.0, v.1);
        }
    }

    pub fn register_event(&mut self, event_name: &str, function: EventFunction)
    {
        self.func_map
            .entry(String::from(event_name))
            .or_insert(vec![])
            .push(function);
    }
}