{{- if or .Module.Structs .Module.Enums .Module.Externs -}}
pub mod data_structs;{{ nl }}
{{- end }}
{{- range .Module.Interfaces -}}
pub mod {{snake .Name}};{{ nl }}
{{- end -}}
