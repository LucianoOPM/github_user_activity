# GitHub User Activity

This application was created to help users continue their learning journey based on the activities outlined on the [roadmap.sh](https://roadmap.sh/) website. It allows users to retrieve activity data for a specified GitHub user.

## Features
- [RoadMap URL](https://roadmap.sh/projects/github-user-activity)
- Fetch activity information for any GitHub user.
- Lightweight and fast, built with [Rust](https://www.rust-lang.org/).
- Easy-to-use commands for running and building the application.

---

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (with `cargo`)
- Internet connection (to fetch GitHub data)

### Running the Application

To run the application, use the following command:

```bash
cargo run <username>
```

Replace `<username>` with the GitHub username you want to search.

For example:

```bash
cargo run octocat
```

### Building the Application

To compile the project, use the following command:

```bash
cargo build
```

This will generate an executable binary in the `target/debug` or `target/release` folder, depending on the build profile.

### Running the Compiled Application

Once the project is compiled, you can use the binary to run the application:

1. Navigate to the `target/debug` or `target/release` folder.
2. Execute the binary and provide a GitHub username as a parameter:

   ```bash
   ./github_user_activity <username>
   ```

---

## Configuration

This application uses the GitHub public API to fetch user activity. No additional configuration is required. However, for frequent or large-scale queries, consider using a personal access token to avoid rate-limiting. You can add this feature to the application if needed.

---

## Example Output

When you run the application with a valid username, it retrieves and displays information about the user's GitHub activity (e.g., public repositories, contributions, etc.).

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m "Add new feature"`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

## Acknowledgments

- Inspired by [roadmap.sh](https://roadmap.sh/).
- Built with the Rust programming language.
- Special thanks to the GitHub API for providing access to user data.

