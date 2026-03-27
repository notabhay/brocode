fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest_file("brocode-windows-sandbox-setup.manifest");
    let _ = res.compile();
}
