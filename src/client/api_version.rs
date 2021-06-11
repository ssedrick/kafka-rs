struct ApiVersion {}

struct ApiVersion3 {
    client_software_name: &str,
    client_software_version: &str,
    _tagged_fields: Vec<Tags>,
}
