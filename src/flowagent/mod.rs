use crate::entity::Entity;

#[derive(Debug)]
pub struct FlowAgent{
    name: String,
    id: u32,
}

impl FlowAgent{
    pub fn new()->FlowAgent{
        //todo!();
        FlowAgent{
            name: "FlowAgent".to_string(),
            //NEED TO CREATE ID MANUALLY
            id: 0
        }
    }

    
}

impl Entity for FlowAgent{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}