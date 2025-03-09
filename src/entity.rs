pub trait Entity{
    fn get_name(&self) -> String;
    fn get_id(&self) -> u32;
}