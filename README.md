<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/S2ZwTrA.png" alt="Project logo"></a>
</p>
<h3 align="center">Rusty Road</h3>

<div align="center">

[![Hackathon](https://img.shields.io/badge/rust-gray.svg?&logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/RileySeaburg/RustyRoad.svg)](https://github.com/RileySeaburg/RustyRoad/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RileySeaburg/RustyRoad.svg)](https://github.com/RileySeaburg/RustyRoad/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

</div>

---

<!-- Alert letting the user know to use the release candidate or the cargo crate. Not to use master as it is still in development -->

> Note: Rusty Road is still in development. Please use the release candidate or the cargo crate. Do not use master for production purposes. It is still under heavy development and is not ready for production use.

<p align="center">
  Rusty Road is a powerful and reliable web framework for the Rust programming language, inspired by Ruby on Rails. Combining the familiar conventions and ease of use of Rails with the performance and efficiency of Rust, Rusty Road empowers developers to build cutting-edge web applications with confidence.
  <br>
  The name "Rusty Road" not only reflects the language that the framework is built on, but also the journey that developers will take as they build their applications.
</p>
<p align="center">
  Born from a passion for Rust and a desire to make it more accessible to developers of all skill levels, Rusty Road was created to bridge the gap between Rust's low-level control and high-level web framework capabilities. With its lightning-fast performance and powerful features, Rusty Road is well-equipped to handle the demands of modern web development.
</p>
<p align="center">
  As Rusty Road continues to evolve, it will break new ground in Rust and web development, enabling developers to create increasingly advanced and sophisticated web applications. By providing a solid foundation for web development, Rusty Road will empower developers to achieve their greatest goals and make the world a better place through the power of software.
</p>
<p align="center">
  If you're tired of slow and unreliable web frameworks, and you're ready to take your web development to the next level with Rust, Rusty Road is your answer. Experience the perfect blend of Ruby on Rails' ease of use with the performance and efficiency of Rust, and unlock the full potential of your web development.
  <br>
  Don't let your web development be held back any longer. With Rusty Road, you can build fast and reliable applications that will delight your users and set you apart from the competition. Embrace the power of Rusty Road and elevate your web development today!
</p>




## 📝 Table of Contents

- [Problem Statement](#problem_statement)
- [Idea / Solution](#idea)
- [What is Rusty Road](#Rusty_actix)
- [Current Features](#features)
- [Dependencies / Limitations](#limitations)
- [Future Scope](#future_scope)
- [Setting up a local environment](#getting_started)
- [Usage](#usage)
- [Technology Stack](#tech_stack)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgments)

## 🧐 Problem Statement <a name = "problem_statement"></a>

Rust Needs a Rails

I outlined this in a blog post here: https://rileyseaburg.com/posts/rust-needs-a-rails

- IDEAL: In a perfect world, Rust would have a framework that is as easy to use as Ruby on Rails. It would be
  easy to learn, easy to use, and easy to maintain. It would be fast, secure, and reliable. It would be
  extensible and customizable. It would be a joy to use.
- REALITY: Rust is a powerful language, but it is not easy to use. It is not easy to learn, and it is not easy to
  maintain. It is not fast, secure, or reliable. It is not extensible or customizable. It is not a joy to use even though it is the most loved programming language on [stack overflow](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages).
- CONSEQUENCES:
  - Web servers are insecure, because most languages are not as secure as Rust.
  - Web servers are slow, because most languages are not as fast as Rust.
  - Web servers are unreliable, because most languages are not as reliable as Rust.
  - Web servers are not extensible or customizable, because most languages are not as extensible or customizable as Rust.
  - Web servers are not a joy to use, because most languages are not as fun to use as Rust.



## 💡 Idea / Solution <a name = "idea"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

## :dog: What is Rusty Road <a name = "Rusty_actix"></a>

Rusty Road is a framework written in Rust that is based on Ruby on Rails. It is designed to provide the familiar conventions and ease of use of Ruby on Rails, while also taking advantage of the performance and efficiency of Rust.

Rusty Road is intended to offer developers a powerful and reliable platform for building web applications using Rust, and its name incorporates a rust-themed crab pun in a playful and memorable way.

### Understanding Rusty Road

Rusty Road currently works with the actix web framework, the Diesel ORM, the Tera template engine, the SASS asset pipeline, and the PostgreSQL database. It also uses the dotenv crate to manage environment variables, and it uses the dotenv-linter crate to lint environment variables.

Because Rusty Road uses the actix web framework, the architecture is not exactly MVC

## 🎈 Current Features <a name="features"></a>

- Database migrations
- Database seeds
- Support for PostgreSQL
- Support for MySQL
- Support for SQLite
- Support for MongoDB (Planned)
- Routing (actix)
- Templating (Tera)
- CSS Framework (Tailwind CSS)

## ⛓️ Dependencies / Limitations <a name = "limitations"></a>

- Rust is a relatively new language, and it is not as mature as Ruby on Rails.
- Rust is not as easy to use as Ruby on Rails.
- Rust has a borrow checker, which can make it difficult to use.

## 🚀 Future Scope <a name = "future_scope"></a>

- Add support for more database adapters.
- Add support for react.
- Add support for GraphQL.
- Add support for API based microservices.
- Add support for more asset pipelines.
- One click deployment to AWS, GCP, Azure, and Digital Ocean or provide a docker image.
- Add support for more authentication frameworks.

## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development
and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Solving PostgreSQL linkage issue

If you encounter an error like this: `LINK : fatal error LNK1181: cannot open input file 'libpq.lib'`, it means the project is not able to find the libpq library. Follow these steps to resolve the issue:

1. If you haven't already, download and install PostgreSQL binaries for Windows from the [official website](https://www.postgresql.org/download/windows/).
2. Make sure to install it in an easily accessible location, like `C:\Program Files\PostgreSQL\13\`.
3. After installation, add `C:\Program Files\PostgreSQL\13\lib` to your PATH variable (`libpq.lib` should be in this directory).
   - Press `Windows key -> Type 'Environment Variables' -> Click on 'Edit the system environment variables' -> Click 'Environment Variables...' button -> In 'System Variables' section, find and select 'Path' -> Click 'Edit...' Button -> Click 'New' button -> Paste your path
4. After you've added the path to `libpq.lib` to PATH, restart your command prompt or terminal and try building the project again.

_Note: Replace `C:\Program Files\PostgreSQL\13\lib\` with your exact path where PostgreSQL is installed._


### Installing Node Version Manager (nvm) for Windows

The Rusty Road project uses Node.js, which we'll manage versions with by using Node Version Manager (nvm). To install nvm for Windows:

1. Visit the latest release page for nvm for Windows at https://github.com/coreybutler/nvm-windows/releases
2. Download the `nvm-setup.zip` file.
3. Extract the zip file and run the installer (`nvm-setup.exe`).
4. Follow the instructions provided by the installer.
5. Once nvm is installed, close your terminal or command prompt and open a new one for the changes to take effect.
6. Verify that nvm is installed correctly by typing `nvm version` into your new terminal. If a version number is displayed, nvm has been installed successfully.

### Prerequisites

Rust is required to build and run Rusty Road. You can install Rust using rustup. rustup is a tool that helps manage Rust installations, it allows for installing multiple versions of Rust and switching between them easily.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Installing

#### Crates.io
Rusty Road is available on [crates.io](https://crates.io/crates/rustyroad). You can install it using cargo:

```
cargo install rustyroad
```

Bonus Step #1 (optional): Add the following to your .bashrc or .zshrc file to make the rustyroad command available in your terminal:

```
export PATH="$HOME/.cargo/bin:$PATH"
```

Bonus Step #2 (optional): Create a symbolic link to the rustyroad command in your ~/.cargo/bin directory.
This will create a symlink from rustyroad to rr inside ~/.cargo/bin/ directory. 
Now you can invoke `rr` command which is just a symlink to rustyroad.

```
sudo ln -s ~/.cargo/bin/rustyroad /usr/local/bin/rr
```

(Windows users can download the executable from the [releases page](https://github.com/RileySeaburg/Rusty-Road/releases) and add it to their PATH.)


#### Installing from source

Clone the repository and run the setup script.

```
git clone https://github.com/RileySeaburg/RustyRoad
```

```
cd RustyRoad
```

```
cargo run
```


## 🎈 Usage <a name="usage"></a>


The cli will prompt you to create a new project. Enter the name of your project and the cli will create a new project in the current directory.

```bash
$ rustyroad
CLI for Rusty Road

Usage: rustyroad.exe <COMMAND>

Commands:
  new       Creates a new project
  generate  Generates a new route, model, or controller
  migrate   Runs migrations
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
Follow the command flow to create a new project

```shell
rustyroad new my_project
```

Generate a new route
```shell
rustyroad generate route users
```


## ⛏️ Built With <a name = "tech_stack"></a>

- [Rust](https://www.rust-lang.org/) - Programming Language
- [actix](https://actix.rs/) - Web Framework
- [Diesel](https://diesel.rs/) - ORM
- [Tera](https://tera.netlify.app/) - Template Engine
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Cucumber Rust](https://github.com/cucumber-rs) - Testing

## ✍️ Authors <a name = "authors"></a>

- [@rileyseaburg](https://github.com/RileySeaburg) - Idea & Initial work

See also the list of [contributors](https://github.com/RileySeaburg/RustyRoad/contributors)
who participated in this project.

## 🎉 Acknowledgments <a name = "acknowledgments"></a>

- Creator of Ruby on Rails, David Heinemeier Hansson (DHH)
- Creator of Rust, Graydon Hoare
