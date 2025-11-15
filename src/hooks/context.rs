use std::fmt;
use std::path::PathBuf;

use crate::template::TemplateObjectResource;

pub struct RhaiHooksContext {
    pub template_object: TemplateObjectResource,
    pub allow_commands: bool,
    pub silent: bool,
    pub working_directory: PathBuf,
    pub destination_directory: PathBuf,
}

impl fmt::Debug for RhaiHooksContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RhaiHooksContext")
            .field("template_object", &"<TemplateObjectResource>")
            .field("allow_commands", &self.allow_commands)
            .field("silent", &self.silent)
            .field("working_directory", &self.working_directory)
            .field("destination_directory", &self.destination_directory)
            .finish()
    }
}
