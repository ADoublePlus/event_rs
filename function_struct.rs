pub use crate::event_rs::event_struct::EventSystem;

pub struct EventFunction
{
    pub fail_function: Box<Fn(&mut EventSystem)>,
    pub success_function: Box<Fn(&mut EventSystem)>,
    pub execute_function: Box<Fn(&mut EventSystem) -> Result<(), ()>>
}

impl EventFunction
{
    pub fn new() -> EventFunction
    {
        EventFunction
        {
            fail_function: Box::new(|_e: &mut EventSystem| {}),
            success_function: Box::new(|_e: &mut EventSystem| {}),
            execute_function: Box::new(|_e: &mut EventSystem| Ok(()))
        }
    }

    pub fn from_all(
        exec: Box<Fn(&mut EventSystem) -> Result<(), ()>>,
        success: Box<Fn(&mut EventSystem)>,
        fail: Box<Fn(&mut EventSystem)>
    ) -> EventFunction
    {
        EventFunction
        {
            fail_function: fail,
            success_function: success,
            execute_function: exec
        }
    }

    pub fn from_exec(exec: Box<Fn(&mut EventSystem) -> Result<(), ()>>) -> EventFunction
    {
        EventFunction
        {
            fail_function: Box::new(|_e: &mut EventSystem| {}),
            success_function: Box::new(|_e: &mut EventSystem| {}),
            execute_function: exec
        }
    }

    pub fn execute(&self, _event_sys: &mut EventSystem)
    {
        let result = (self.execute_function)(_event_sys);

        match result
        {
            Ok(()) => (self.success_function)(_event_sys),
            Err(()) => (self.fail_function)(_event_sys)
        }
    }
}