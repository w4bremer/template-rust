[package]
name = "{{snake .Module.Name}}"
version = "{{.Module.Version}}"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
async-trait = "0.1.73"
serde = { version = "1.0.187", features = ["derive"] }
serde_json = "1.0"
signals2 = "0.3.3"
{{- range .Module.Imports }}
{{ snake .Name }} = { path = "../{{ snake .Name }}" }
{{- end }}
