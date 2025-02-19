# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Generate GutLab GraphQL schema
````
```sh 
graphql-client introspect-schema --header "Authorization: Bearer $CI_JOB_TOKEN" --output src/api/gitlab/graphql
/schema.json https://gitlab.com/api/graphql 
`
