# Timeshot

**Timeshot** is a **cross-platform** and **easy-to-use** command-line tool that allows you to create **snapshots** of your code, so you can quickly revert to a previous state if something goes wrong. It's particularly useful when you're making risky changes or debugging and want an easy way to restore your project.

## Installation

Right now, **Timeshot is only available for macOS. Windows and Linux versions still need to be tested, but you can build them from the source code.**
- **macOS**: Download it using [Homebrew](https://brew.sh/):
    ```bash
    brew tap stormyzio/timeshot
    # then
    brew install timeshot
    ```
  You can also download the binary from the [releases page](https://github.com/stormyzio/timeshot/releases).

## Features

- **Create a snapshot**: Save the current state of your project with `timeshot create`. Only one snapshot can be active at a time, capturing all of your code files.
- **Revert to a snapshot**: If your changes don't work out, quickly go back to your last snapshot using `timeshot back`.
- **Reset your snapshot**: Delete the current snapshot with `timeshot reset` to free up space for a new one.

## Usage

1. Create a snapshot:
   ```bash
   timeshot create
   ```

2. Revert to a snapshot:
   ```bash
    timeshot back
    ```
   
3. Reset the snapshot:
    ```bash
    timeshot reset
    ```

## .tsignore

You can create a `.tsignore` file in your project directory to exclude files or directories from being included in the snapshot. Each line in the file should be a relative path to the
file or directory you want to ignore. Some folders are ignored by default, such as `.git`, `.vscode`, `.timeshot` and `.idea`.

**Example for a Node.js project**:
```
# You can add comments
node_modules
dist
package.json
package-lock.json
```

## License

This project is licensed under the MIT License - see the [LICENSE](https://choosealicense.com/licenses/mit/) file for details.







