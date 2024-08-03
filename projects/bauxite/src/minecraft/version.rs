pub trait MinecraftVersion {
    fn name(&self) -> &String;
    fn is_snapshot(&self) -> bool;
    fn json_url(&self) -> &String;
}
