// @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainer.base.schema.json
typify::import_types!("./src/devcontainer.schema.json");

// TODO: https://github.com/oxidecomputer/typify/issues/896
// diff --git a/devcontainer.schema.json.bak b/devcontainer.schema.json
// index 86709ec..828f6a7 100644
// --- a/devcontainer.schema.json.bak
// +++ b/devcontainer.schema.json
// @@ -451,18 +451,6 @@
//                                                 },
//                                                 "gpu": {
//                                                         "oneOf": [
// -                                                               {
// -                                                                       "type": [
// -                                                                               "boolean",
// -                                                                               "string"
// -                                                                       ],
// -                                                                       "enum": [
// -                                                                               true,
// -                                                                               false,
// -                                                                               "optional"
// -                                                                       ],
// -                                                                       "description": "Indicates whether a GPU is required. The string \"optional\" indicates that a GPU is optional. An object value can be used to configure more detailed requirements."
// -                                                               },
//                                                                 {
//                                                                         "type": "object",
//                                                                         "properties": {
