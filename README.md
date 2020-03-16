![build](https://github.com/sinsoku/miteras-cli/workflows/build/badge.svg)
[![codecov](https://codecov.io/gh/sinsoku/miteras-cli/branch/master/graph/badge.svg)](https://codecov.io/gh/sinsoku/miteras-cli)

# MITERAS CLI

An (**unofficial**) command-line tool for [MITERAS](https://www.persol-pt.co.jp/miteras/).

## Installation

### Install from GitHub Releases

Download the latest version from [GitHub Releases](https://github.com/sinsoku/miteras-cli/releases).

### Homebrew on macOS

If you are using MacOS, you can install with [Homebrew](https://brew.sh/).

```console
$ brew tap sinsoku/tap
$ brew install miteras-cli
```

## Usage

### login

Save credentials using the `login` command.

```console
$ miteras login
Try logging in to MITERAS.

Org: A123456
Username: sinsoku
Password: ********

Login successful.
```

### clock-in / clock-out

```console
$ miteras clock-in
$ miteras clock-out
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/sinsoku/miteras-cli/.

## License

The Orb is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
