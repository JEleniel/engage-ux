# Engage UX

[Engage UX is copyright &copy; 2025 JEleniel and released under the GNU General Public License 3.0 (or later)](LICENSE.md)

![Static Badge](https://img.shields.io/badge/2024-orange?style=plastic&logo=Rust&label=Rust&link=https%3A%2F%2Fwww.rust-lang.org%2F)
![GitHub License](https://img.shields.io/github/license/JEleniel/engage-ux?style=plastic&logo=GNU)
![Static Badge](https://img.shields.io/badge/Windows%2C%20MacOS%2C%20Linux%2C%20Android%2C%20iOS-brightblue?style=plastic&label=Operating%20System&link=https%3A%2F%2Fwww.rust-lang.org%2F)
![Static Badge](https://img.shields.io/badge/x64%2C%20ARM64-yellow?style=plastic&label=CPU&link=https%3A%2F%2Fwww.rust-lang.org%2F)

![GitHub Release](https://img.shields.io/github/v/release/JEleniel/engage-ux?sort=semver&display_name=release&style=plastic&logo=GitHub)
![GitHub contributors](https://img.shields.io/github/contributors/JEleniel/engage-ux?style=plastic&logo=GitHub)
![GitHub Discussions](https://img.shields.io/github/discussions/JEleniel/engage-ux?style=plastic&logo=GitHub)

## About

Engage UX is an application UI (a.k.a. window) and input/output manager. It is designed around the User eXperience (UX) first, and flexibility and extendability second.

## Supported Operating Systems

| Linux | Windows | MacOS | Android | iOS |
| :---: | :-----: | :---: | :-----: | :-: |
|   ✔   |    ✔    |   ✔   |    ✔    |  ✔  |

## Features

- Secure by Design + All connections between the Clients and Host (including local) are encrypted using TLS 1.3 + Supports both Operating System and administrator provided Certificate Authority certificates + All client connections are authenticated, authorized, and auditable through the Operating System + Neither the Host nor the Client record anything from the applications or OS + Supports rendering the UI on the Host system, eliminating the need for separate tools such as VNC or RDP
- Uses D-Bus as the message bus system and for Inter-Process Communication (IPC)
- Fully themable output (within device capabilities) + Control the color of every element + Full support for CSS + Support for precise scaling of output to ensure physically accurate measurements

## Design

Engage UX works by abstracting input and output devices across multiple Client Systems and presenting them all to the Host System as though they were local. This allows multiple users, input devices, output devices, and systems to act as a single unit.

Engage UX provides three components that make this possible:

- Engage UX Host Software: The code that runs on the Host System, manages the UX of the applications, and manages client connections
- Engage UX Client Software: The code running on Client Systems that interfaces with input and output devices through Device Drivers, communicates with the Host System, and renders the UX
- Device Drivers: Code that implements the Engage UX Driver ABI and communicates between Engage UX and input or output devices

```mermaid
graph
	user(User) --> input
	output --> user

	userH(User) --> inputH
	outputH --> userH

	eClientH <--> eHost
	eClient <--> eHost
	eClient2 <--> eHost
	eClient3 <--> eHost

	subgraph client3[Client System]
		input3([Input Device])
			--> iDriver3[[Input Device Driver]]
			--> eClient3{{Engage UX Client Software}}
	end

	subgraph client2[Client System]
		eClient2{{Engage UX Client Software}}
			--> oDriver2[[Output Driver]]
			--> output2([Output Device])
	end

	subgraph host[Host System]
		app[Application Software]
		<--> eHost{{Engage UX Host Software}}
			<--> os[Operating System]
		inputH([Input Device])
			--> iDriverH[[Input Device Driver]]
			--> eClientH{{Engage UX Client Software}}
			--> oDriverH[[Output Device Driver]]
			--> outputH([Output Device])
	end

	subgraph client[Client System]
		input([Input Device])
			--> iDriver[[Input Device Driver]]
			--> eClient{{Engage UX Client Software}}
			--> oDriver[[Output Device Driver]]
			--> output([Output Device])
	end
```

## Getting started

## Contributing

[Contributing to the Project](CONTRIBUTING.md)

### How to Build

## Security and Supporty

[Getting Support](SUPPORT.md)

[Reporting Security Issues](SECURITY.md)
