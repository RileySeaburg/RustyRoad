# GrapeJS Example
This project was created using Rusty Road. Rusty Road is Rails for Rust. It is a CLI tool that allows you to create a new Rust project with a few commands. It also comes with TailwindCSS and Actix pre-installed.

This example project uses [GrapeJS](https://grapesjs.com/) to create a drag and drop website builder. It also uses [TailwindCSS](https://tailwindcss.com/) for styling and [Actix](https://actix.rs/) for the backend.

You can generate this example project by running the following command:

```bash
cargo install rustyroad
```
If you have issues installing Rusty Road, please see the [installation instructions](../README.md#known-issues).


Create a new RustyRoad project:

```bash
rustyroad new example-grapesjs
```

Change directories into the new project:

```bash
cd example-grapesjs
```

Install the dependencies:

```bash
yarn install
```

Install the Optional GrapesJS Builder:

```bash
rustyroad feature add grapesjs
```

## Getting Started with this Example Project

### Page Builder
To access the page builder, go to the /page/{pageId} URL. For example, if you are running the server locally, log in and go to localhost/page/1 to access the page builder for the page with id 1.
            
### Start the Server
 ```
 cargo run
 ```

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.