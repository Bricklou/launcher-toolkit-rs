pub trait MinecraftVersion {
    fn libraries_json(&self) -> &'static str;
    fn client(&self) -> &'static str;
    fn assets_index(&self) -> &'static str;
    fn json_version(&self) -> &'static str;
    fn name(&self) -> &String;
    fn is_snapshot(&self) -> bool;
    fn json_url(&self) -> &String;
}
