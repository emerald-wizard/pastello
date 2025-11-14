Added this to the workspace settings

	"rust-analyzer.linkedProjects": [
		"apps/api-rs/Cargo.tom"
	]


Also added replace directives for go but that may be unnecessary if we switch to rust.


How the settings are loaded by scope:

Defaults < User < Workspace < Folder