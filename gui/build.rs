
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(windows) {
        use winres::{
            WindowsResource,
            //VersionInfo,
        };

        WindowsResource::new()
            .set_icon("build/icon.ico")
            /*.set_version_info(
                VersionInfo::PRODUCTVERSION,
                0b_0000_1000_1000_0000
            )*/
            .set_manifest_file("build/manifest.xml")
            .compile()?;

        static_vcruntime::metabuild();
    }

    Ok(())
}
