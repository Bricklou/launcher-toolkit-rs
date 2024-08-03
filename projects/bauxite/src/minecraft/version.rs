use super::jsons::version_manifest::McVersionManifest;

pub trait MinecraftVersion {
    fn id(&self) -> &String;
    fn name(&self) -> &String;
    fn is_snapshot(&self) -> bool;
    fn json_url(&self) -> &String;
    fn manifest(&self) -> &McVersionManifest;
}
