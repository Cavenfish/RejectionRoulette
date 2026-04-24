# RejectionRoulette

RejectionRoulette is a cross-platform desktop app for tracking job applications, interviews, and offers. 
Built with Rust and Dioxus, it runs on Linux, Windows, and macOS.

## Features

- Track job applications with company, role, status, and submission date
- Record interview details and notes for each application
- Manage offers including salary, bonus, equity, expiration date, and acceptance status
- Cross-platform support for Linux, Windows, and macOS

## Screenshots

![Applications Table](screenshots/applications_table.png)
*Applications table showing job applications with company, role, status, and submission dates*

![Interviews](screenshots/interviews.png)
*Interview tracking view with details and notes for each application*

![Offers](screenshots/offers.png)
*Offers management showing salary, bonus, equity details, and acceptance status*

![Add Application](screenshots/add_application.png)
*Add new job application form*

![Edit Application](screenshots/edit_application.png)
*Edit existing application form*

![Add Interview](screenshots/add_interview.png)
*Add interview details form*

### Themes
The app comes with multiple themes. Check out some of them below.

![Dark Theme](screenshots/dark.png)
*Dark theme*

![Light Theme](screenshots/light.png)
*Light theme*

![Cyberpunk Theme](screenshots/cyberpunk.png)
*Cyberpunk theme*

![Deep Sea Theme](screenshots/deep_sea.png)
*Deep sea theme*

![Sepia Theme](screenshots/sepia.png)
*Sepia theme*


## Installation

### Linux

Use the `.deb` bundle produced by the build pipeline or install from source.

### Windows

Use the `.msi` installer produced by the build pipeline.

### macOS

Use the `.dmg` installer produced by the build pipeline.

## Development

### Requirements

- Rust toolchain
- `dioxus-cli`
- Platform-specific GTK/WebKit dependencies for desktop bundling on Linux


